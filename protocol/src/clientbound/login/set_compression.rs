use crate::packet;
use packet::{SharedState, Varint};

packet! {
    SetCompression, all,
    {
        threshold: Varint,
    }
}

impl Parsable for SetCompression {
    fn update_status(&self, status: &mut SharedState) -> Result<(), ()> {
        status.compress = self.threshold.to::<i32>() as u32;
        Ok(())
    }
}
