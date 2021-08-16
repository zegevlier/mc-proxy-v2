use crate::{
    conf::Configuration,
    functions::{fid_to_pid, Fid},
    packet::Packet,
    parsable::Parsable,
    raw_packet::RawPacket,
    Direction, SharedState,
};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct PlayerPositionRotation {
    x: f64,
    feet_y: f64,
    z: f64,
    yaw: f32,
    pitch: f32,
    on_ground: bool,
}

#[async_trait::async_trait]
impl Parsable for PlayerPositionRotation {
    fn empty() -> Self {
        Self {
            x: 0f64,
            feet_y: 0f64,
            z: 0f64,
            yaw: 0f32,
            pitch: 0f32,
            on_ground: false,
        }
    }

    fn parse_packet(&mut self, mut packet: RawPacket) -> Result<(), ()> {
        self.x = packet.decode_double()?;
        self.feet_y = packet.decode_double()?;
        self.z = packet.decode_double()?;
        self.yaw = packet.decode_float()?;
        self.pitch = packet.decode_float()?;
        self.on_ground = packet.decode_bool()?;
        Ok(())
    }

    fn get_printable(&self) -> String {
        format!(
            "{} {} {} {} {} {}",
            self.x, self.feet_y, self.z, self.yaw, self.pitch, self.on_ground,
        )
    }

    fn packet_editing(&self) -> bool {
        true
    }

    async fn edit_packet(
        &self,
        status: SharedState,
        plugins: &mut Vec<Box<dyn crate::plugin::EventHandler + Send>>,
        _config: &Configuration,
    ) -> Result<(Vec<(Packet, Direction)>, SharedState), ()> {
        let mut return_vec = Vec::new();
        for plugin in plugins {
            match plugin.on_move(self.x, self.feet_y, self.z) {
                Some(plugin_vec) => {
                    return_vec.extend(plugin_vec);
                    break;
                }
                None => continue,
            }
        }
        if !return_vec.is_empty() {
            let mut raw_packet = RawPacket::new();
            raw_packet.encode_double(self.x);
            raw_packet.encode_double(self.feet_y);
            raw_packet.encode_double(self.z);
            raw_packet.encode_float(self.yaw);
            raw_packet.encode_float(self.pitch);
            raw_packet.encode_bool(self.on_ground);
            return_vec.push((
                Packet::from(raw_packet, fid_to_pid(Fid::PlayerPositionRotation)),
                Direction::Serverbound,
            ));
        }
        Ok((return_vec, status))
    }
}
