#[macro_export]
macro_rules! packet {
    ($name:ident, all, {$($field_name:ident : $field_type:ty),* $(,)?}) => {
        packet!($name, struc, $($field_name : $field_type),*);
        packet!($name, disp, $($field_name),*);
        packet!($name, def, $($field_name),*);
        packet!($name, dec, $($field_name),*);
        packet!($name, enc, $($field_name),*);
    };
    ($name:ident, -disp, {$($field_name:ident : $field_type:ty),* $(,)?}) => {
        packet!($name, struc, $($field_name : $field_type),*);
        packet!($name, def, $($field_name),*);
        packet!($name, dec, $($field_name),*);
        packet!($name, enc, $($field_name),*);
    };
    ($name:ident, struc, $($field_name:ident : $field_type:ty),* $(,)?) => {
        use crate::{parsable::Parsable, functions::fid_to_pid};
        use packet::{RawPacket, SafeDefault, Packet};

        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        pub struct $name {
            $(pub $field_name: $field_type),*
        }
    };
    ($name:ident, disp, $($field_name:ident),* $(,)?) => {
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", <[&str]>::join(&[$(&format!("{}", self.$field_name)),*], " "))
            }
        }
    };
    ($name:ident, def, $($field_name:ident),* $(,)?) => {
        impl SafeDefault for $name {
            fn default() -> Self {
                Self {
                    $($field_name: SafeDefault::default()),*
                }
            }
        }
    };
    ($name:ident, dec, $($field_name:ident),* $(,)?) => {
        impl packet::ProtoDec for $name {
            #[allow(unused_variables)]
            fn decode(&mut self, p: &mut RawPacket) -> packet::Result<()> {
                $(self.$field_name = p.decode()?;)*
                Ok(())
            }
        }
    };
    ($name:ident, enc, $($field_name:ident),* $(,)?) => {
        impl packet::ProtoEnc for $name {
            #[allow(unused_variables)]
            fn encode(&self, p: &mut RawPacket) -> packet::Result<()> {
                $(p.encode(&self.$field_name)?;)*
                Ok(())
            }

            fn encode_packet(&self) -> packet::Result<Packet> {
                let mut p = RawPacket::new();
                self.encode(&mut p)?;
                Ok(Packet::from(p, fid_to_pid(crate::functions::Fid::$name)))
            }
        }
    };
}

#[macro_export]
macro_rules! functions_macro {
    (
        clientbound {
            handshaking {
                $($ch_pid:expr => $ch_fid:ident),*  $(,)?
            }
            status {
                $($cs_pid:expr => $cs_fid:ident),*  $(,)?
            }
            login {
                $($cl_pid:expr => $cl_fid:ident),*  $(,)?
            }
            play {
                $($cp_pid:expr => $cp_fid:ident),*  $(,)?
            }
        }
        serverbound {
            handshaking {
                $($sh_pid:expr => $sh_fid:ident),*  $(,)?
            }
            status {
                $($ss_pid:expr => $ss_fid:ident),* $(,)?
            }
            login {
                $($sl_pid:expr => $sl_fid:ident),*  $(,)?
            }
            play {
                $($sp_pid:expr => $sp_fid:ident),*  $(,)?
            }
        }
    ) => {
        use packet::Varint;
        use packet::SafeDefault;

        pub struct Functions {
            map: HashMap<Direction, HashMap<State, HashMap<Varint, Fid>>>,
            list: HashMap<Fid, Box<dyn Parsable + Send + Sync>>,
        }

        pub fn fid_to_pid(fid: Fid) -> Varint {
            match fid {
                Fid::Unparsable => Varint::from(-1),
                $(Fid::$ch_fid => Varint::from($ch_pid),)*
                $(Fid::$cs_fid => Varint::from($cs_pid),)*
                $(Fid::$cl_fid => Varint::from($cl_pid),)*
                $(Fid::$cp_fid => Varint::from($cp_pid),)*
                $(Fid::$sh_fid => Varint::from($sh_pid),)*
                $(Fid::$ss_fid => Varint::from($ss_pid),)*
                $(Fid::$sl_fid => Varint::from($sl_pid),)*
                $(Fid::$sp_fid => Varint::from($sp_pid),)*
            }
        }

        impl Functions {
            pub fn new() -> Self {
                let map: HashMap<Direction, HashMap<State, HashMap<Varint, Fid>>> = hashmap! {
                    Direction::Clientbound => hashmap! {
                        State::Handshaking => hashmap! [
                            $(Varint::from($ch_pid) => Fid::$ch_fid),*
                        ],
                        State::Status => hashmap! [
                            $(Varint::from($cs_pid) => Fid::$cs_fid),*
                        ],
                        State::Login => hashmap! [
                            $(Varint::from($cl_pid) => Fid::$cl_fid),*
                        ],
                        State::Play => hashmap! [
                            $(Varint::from($cp_pid) => Fid::$cp_fid),*
                        ],
                    },
                    Direction::Serverbound => hashmap! {
                        State::Handshaking => hashmap! [
                            $(Varint::from($sh_pid) => Fid::$sh_fid),*
                        ],
                        State::Status => hashmap! [
                            $(Varint::from($ss_pid) => Fid::$ss_fid),*
                        ],
                        State::Login => hashmap! [
                            $(Varint::from($sl_pid) => Fid::$sl_fid),*
                        ],
                        State::Play => hashmap! [
                            $(Varint::from($sp_pid) => Fid::$sp_fid),*
                        ],
                    }
                };
                let mut list: HashMap<Fid, Box<dyn Parsable + Send + Sync>> = HashMap::with_capacity(
                    $crate::functions_macro!(@COUNT; $($sh_fid),*) +
                    $crate::functions_macro!(@COUNT; $($ss_fid),*) +
                    $crate::functions_macro!(@COUNT; $($sl_fid),*) +
                    $crate::functions_macro!(@COUNT; $($sp_fid),*) +
                    $crate::functions_macro!(@COUNT; $($ch_fid),*) +
                    $crate::functions_macro!(@COUNT; $($cs_fid),*) +
                    $crate::functions_macro!(@COUNT; $($cl_fid),*) +
                    $crate::functions_macro!(@COUNT; $($cp_fid),*)
                );
                $(list.insert(Fid::$sh_fid, Box::new(serverbound::handshaking::$sh_fid::default()));)*
                $(list.insert(Fid::$ss_fid, Box::new(serverbound::status::$ss_fid::default()));)*
                $(list.insert(Fid::$sl_fid, Box::new(serverbound::login::$sl_fid::default()));)*
                $(list.insert(Fid::$sp_fid, Box::new(serverbound::play::$sp_fid::default()));)*
                $(list.insert(Fid::$ch_fid, Box::new(clientbound::handshake::$ch_fid::default()));)*
                $(list.insert(Fid::$cs_fid, Box::new(clientbound::status::$cs_fid::default()));)*
                $(list.insert(Fid::$cl_fid, Box::new(clientbound::login::$cl_fid::default()));)*
                $(list.insert(Fid::$cp_fid, Box::new(clientbound::play::$cp_fid::default()));)*
                Self {
                    map,
                    list
                }
            }

            pub fn get_name(&self, direction: &Direction, state: &State, pid: &Varint) -> Option<&Fid> {
                self.map
                    .get(direction)
                    .unwrap()
                    .get(state)
                    .unwrap()
                    .get(pid)
            }

            pub fn get(&self, id: &Fid) -> Option<Box<dyn Parsable + Send + Sync>> {
                self.list.get(id).cloned()
            }
        }

        impl Default for Functions {
            fn default() -> Self {
                Self::new()
            }
        }
    };
    (@COUNT; $($thing:ident),*  $(,)?) => {
        <[()]>::len(&[$($crate::functions_macro!(@SUB; $thing)),*])
    };
    (@SUB; $thing:ident) => {
        ()
    };
}
