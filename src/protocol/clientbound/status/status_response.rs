use crate::packet;

packet! { StatusResponse, all,
    {
        json_response: String,
    }
}

impl Parsable for StatusResponse {}
