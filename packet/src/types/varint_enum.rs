// Modified from https://github.com/feather-rs/feather/blob/e4791aae9f285c3cf417c58a76f83e2d632ee2c5/feather/protocol/src/packets.rs

#[macro_export]
macro_rules! varint_enum {
    ($name:ident; $($thing:meta),* $(;)? {
        $(
            $discriminant:literal = $variant:ident
            $(
                {
                    $(
                        $field:ident: $typ:ty
                    ),* $(,)?
                }
            )?
        ),* $(,)?
    }) => {
        use $crate::Varint;
        #[derive(
            Debug, Clone, PartialEq, Eq, PartialOrd, Hash, serde::Serialize, serde::Deserialize, $($thing),*
        )]
        pub enum $name {
            Unknown,
            $(
                $variant
                $(
                    {
                        $(
                            $field: $typ,
                        )*
                    }
                )?,
            )*
        }

        impl $crate::ProtoEnc for $name {
            fn encode(&self, p: &mut $crate::RawPacket) -> $crate::Result<()> {
                match self {
                    $(
                        $name::$variant
                        $(
                            {
                                $(
                                    $field,
                                )*
                            }
                        )? => {
                            p.encode(&Varint::from($discriminant))?;
                            $($(p.encode($field)?);*)?
                        },
                    )*
                    $name::Unknown => return Err($crate::Error::UnknownEnum),
                }
                Ok(())
            }
        }

        impl $crate::ProtoDec for $name {
            fn decode_ret(p: &mut $crate::RawPacket) -> $crate::Result<Self>
            where
                Self: Sized,
            {
                let s: usize = p.decode::<Varint>()?.into();
                Ok(match s {
                    $(
                        $discriminant => $name::$variant
                        $(
                            {
                                $(
                                    $field: p.decode()?
                                )*
                            }
                        )?,
                    )*
                    _ => $name::Unknown,
                })
            }

            fn decode(&mut self, p: &mut $crate::RawPacket) -> $crate::Result<()> {
                *self = Self::decode_ret(p)?;
                Ok(())
            }
        }

        impl $crate::SizedDefault for $name {
            fn default() -> Self {
                $name::Unknown
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::ProtoEnc;

    #[test]
    fn test_varint_enum() {
        varint_enum! {
            State; {
                0 = Login,
                1 = Play,
            }
        }
        let mut p = crate::RawPacket::new();
        State::Login.encode(&mut p).unwrap();
        assert_eq!(p.decode::<State>().unwrap(), State::Login);
        p.clear();
        State::Play.encode(&mut p).unwrap();
        assert_eq!(p.decode::<State>().unwrap(), State::Play);
    }

    #[test]
    fn test_varint_enum_with_fields() {
        varint_enum! {
            State; {
                0 = Login {
                    name: String,
                },
            }
        }
        let mut p = crate::RawPacket::new();
        State::Login {
            name: "Hi!".to_string(),
        }
        .encode(&mut p)
        .unwrap();
        assert_eq!(
            p.decode::<State>().unwrap(),
            State::Login {
                name: "Hi!".to_string()
            }
        );
    }
}
