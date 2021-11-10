use crate::{
    conf::Configuration, functions::fid_to_pid, packet::Packet, parsable::Parsable,
    raw_packet::RawPacket, Direction, EventHandler, SharedState,
};
use serde::Serialize;

/*
Chunk X	Int	Chunk coordinate (block coordinate divided by 16, rounded down).
Chunk Z	Int	Chunk coordinate (block coordinate divided by 16, rounded down).
Full chunk	Boolean	See Chunk Format.
Primary Bit Mask	VarInt	Bitmask with bits set to 1 for every 16×16×16 chunk section whose data is included in Data. The least significant bit represents the chunk section at the bottom of the chunk column (from y=0 to y=15).
Heightmaps	NBT	Compound containing one long array named MOTION_BLOCKING, which is a heightmap for the highest solid block at each position in the chunk (as a compacted long array with 256 entries at 9 bits per entry totaling 36 longs). The Notchian server also adds a WORLD_SURFACE long array, the purpose of which is unknown, but it's not required for the chunk to be accepted.
Biomes length	Optional VarInt	Size of the following array; should always be 1024. Not present if full chunk is false.
Biomes	Optional array of VarInt	1024 biome IDs, ordered by x then z then y, in 4×4×4 blocks. Not present if full chunk is false. See Chunk Format § Biomes.
Size	VarInt	Size of Data in bytes.
Data	Byte array	See data structure in Chunk Format.
Number of block entities	VarInt	Number of elements in the following array.
Block entities	Array of NBT Tag	All block entities in the chunk. Use the x, y, and z tags in the NBT to determine their positions.
*/

#[derive(Debug, Clone, Serialize)]
struct ChunkSection {
    block_count: i16,
    bits_per_block: u8,
    palette: Option<Vec<i32>>,
    data_array_length: i32,
    block_ids_array: Vec<i32>,
}

#[derive(Clone, Serialize)]
pub struct ChunkData {
    chunk_x: i32,
    chunk_z: i32,
    full_chunk: bool,
    primary_bit_mask: i32,
    heightmaps: nbt::Blob,
    biomes_length: Option<i32>,
    biomes: Option<Vec<i32>>,
    size: i32,
    data: Vec<Option<ChunkSection>>,
    number_of_block_entities: i32,
    block_entities: Vec<nbt::Blob>,
}

#[async_trait::async_trait]
impl Parsable for ChunkData {
    fn default() -> Self {
        Self {
            chunk_x: 0,
            chunk_z: 0,
            full_chunk: false,
            primary_bit_mask: 0,
            heightmaps: nbt::Blob::new(),
            biomes_length: None,
            biomes: None,
            size: 0,
            data: vec![],
            number_of_block_entities: 0,
            block_entities: vec![],
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.chunk_x = packet.decode_int()?;
        self.chunk_z = packet.decode_int()?;
        self.full_chunk = packet.decode_bool()?;
        self.primary_bit_mask = packet.decode_varint()?;
        self.heightmaps = packet.decode_nbt()?;
        if self.full_chunk {
            self.biomes_length = Some(packet.decode_varint()?);
            let mut biome_list = vec![];
            for _ in 0..self.biomes_length.unwrap() {
                biome_list.push(packet.decode_varint()?);
            }
            self.biomes = Some(biome_list);
        }
        self.size = packet.decode_varint()?;

        let mut raw_chunk_data = RawPacket::from(packet.read(self.size as usize)?);
        for y in 0..16 {
            if self.primary_bit_mask & (1 << y) != 0 {
                let mut chunk_section = ChunkSection {
                    block_count: 0,
                    bits_per_block: 0,
                    palette: None,
                    data_array_length: 0,
                    block_ids_array: vec![],
                };
                chunk_section.block_count = raw_chunk_data.decode_short()?;
                chunk_section.bits_per_block = raw_chunk_data.decode_ubyte()?;

                chunk_section.bits_per_block = if chunk_section.bits_per_block <= 4 {
                    4
                } else if chunk_section.bits_per_block <= 8 {
                    chunk_section.bits_per_block
                } else {
                    255
                };

                // log::info!("bpb: {}", chunk_section.bits_per_block);

                if chunk_section.bits_per_block < 255 {
                    let palette_length = raw_chunk_data.decode_varint()?;
                    let mut palette = vec![];
                    for _ in 0..palette_length {
                        palette.push(raw_chunk_data.decode_varint()?);
                    }
                    chunk_section.palette = Some(palette);
                }

                chunk_section.data_array_length = raw_chunk_data.decode_varint()?;
                for _ in 0..chunk_section.data_array_length {
                    let long = raw_chunk_data.decode_ulong()?;
                    if self.chunk_x == 0 && self.chunk_z == 0 && y == 0 {
                        log::info!("{}", long);
                    }
                    // for i in 0..(64.0_f64 / chunk_section.bits_per_block as f64).ceil() as u8 {
                    // for i in 0..loop_amount(chunk_section.bits_per_block) {
                    for i in 0..(64 / chunk_section.bits_per_block) {
                        let mask = ((1 << chunk_section.bits_per_block) - 1)
                            << (i * chunk_section.bits_per_block);
                        let masked_long = (long & mask) >> (i * chunk_section.bits_per_block);
                        chunk_section
                            .block_ids_array
                            .push(match &chunk_section.palette {
                                Some(pal) => {
                                    if masked_long as usize >= pal.len() {
                                        // println!(
                                        //     "Oh no! {} is out of range of {} long: {:0>64b}",
                                        //     masked_long,
                                        //     pal.len(),
                                        //     long
                                        // );
                                        // masked_long as i32
                                        0
                                    } else {
                                        pal[masked_long as usize]
                                    }
                                }
                                None => masked_long as i32,
                            })
                    }
                    // if self.chunk_x == 0 && self.chunk_z == 0 {
                    //     log::info!("{}", long);
                    // }
                }
                self.data.push(Some(chunk_section));
            } else {
                self.data.push(None);
            }
        }

        self.number_of_block_entities = packet.decode_varint()?;
        for _ in 0..self.number_of_block_entities {
            self.block_entities.push(packet.decode_nbt()?);
        }
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {} {} {} {} {:?} {:?} {:?}",
            self.chunk_x,
            self.chunk_z,
            self.full_chunk,
            self.primary_bit_mask,
            self.heightmaps,
            self.biomes,
            self.data,
            // make_string_fixed_length(format!("{}", self.heightmaps), 16),
            // make_string_fixed_length(format!("{:?}", self.biomes), 16),
            // make_string_fixed_length(format!("{:?}", self.data), 16),
            self.block_entities
        )
    }

    fn packet_editing(&self) -> bool {
        true
    }

    async fn edit_packet(
        &self,
        _status: &mut SharedState,
        _plugins: &mut Vec<Box<dyn EventHandler + Send>>,
        _config: &Configuration,
    ) -> Result<Vec<(Packet, Direction)>, ()> {
        let mut raw_packet = RawPacket::new();
        raw_packet.encode_int(self.chunk_x);
        raw_packet.encode_int(self.chunk_z);
        raw_packet.encode_bool(self.full_chunk);
        raw_packet.encode_varint(self.primary_bit_mask);
        raw_packet.encode_nbt(&self.heightmaps);
        if self.full_chunk {
            raw_packet.encode_varint(self.biomes_length.unwrap());
            for biome in self.biomes.as_ref().unwrap() {
                raw_packet.encode_varint(biome.to_owned());
            }
        }

        let mut sections_packet = RawPacket::new();

        for y in 0..16 {
            if self.primary_bit_mask & (1 << y) != 0 {
                let chunk_data = self.data[y].as_ref().unwrap();

                let palette = chunk_data.palette.as_ref();
                sections_packet.encode_short(chunk_data.block_count);
                sections_packet.encode_ubyte(chunk_data.bits_per_block);
                if let Some(palette) = palette {
                    sections_packet.encode_varint(palette.len() as i32);
                    for palette_id in palette {
                        sections_packet.encode_varint(palette_id.to_owned());
                    }
                }

                let long_count = 64 * chunk_data.bits_per_block as i32;

                // if self.chunk_x == 0 && self.chunk_z == 0 {
                //     println!("{} {}", long_count, chunk_data.bits_per_block);
                // }

                sections_packet.encode_varint(long_count);

                let mut block_idx = 0;
                for _ in 0..long_count {
                    let mut long = 0;
                    // for i in 0..(64.0 / chunk_data.bits_per_block as f64).ceil() as u8 {
                    // for i in 0..loop_amount(chunk_data.bits_per_block) {
                    for i in 0..64 / chunk_data.bits_per_block {
                        let block_id = match chunk_data.block_ids_array.get(block_idx) {
                            Some(block_id) => *block_id,
                            None => 0,
                        };
                        let real_id = match palette {
                            Some(pal) => (&pal.iter().position(|r| r == &block_id).unwrap())
                                .to_owned() as i32,
                            None => block_id,
                        };
                        long |= (real_id as i64) << (chunk_data.bits_per_block * i);
                        block_idx += 1;
                    }

                    sections_packet.push_slice(&long.to_be_bytes());
                }
            }
        }

        sections_packet.prepend_length();
        raw_packet.push_vec(sections_packet.get_vec());

        raw_packet.encode_varint(self.number_of_block_entities);
        for block_entity in self.block_entities.iter() {
            raw_packet.encode_nbt(block_entity);
        }

        let mut should_be_same = Self::default();

        should_be_same.parse_packet(raw_packet.clone()).unwrap();

        // if self.chunk_x == 0 && self.chunk_z == -1 {
        //     log::info!("{:?}", self.data);
        //     log::info!("{:?}", should_be_same.data);
        // }

        Ok(vec![(
            Packet::from(raw_packet, fid_to_pid(crate::functions::Fid::ChunkData)),
            Direction::Clientbound,
        )])
    }
}
