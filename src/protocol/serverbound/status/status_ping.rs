use crate::packet;

packet! { StatusPing, all, {
    payload: i64,
}}

impl Parsable for StatusPing {}
