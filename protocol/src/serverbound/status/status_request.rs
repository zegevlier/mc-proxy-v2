use crate::packet;

packet! { StatusRequest, all, {}}

impl Parsable for StatusRequest {}
