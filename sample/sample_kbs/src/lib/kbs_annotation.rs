// kbs_specific_annotation_packet

extern crate serde;

use self::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AnnotationPacket {
    pub key_url: String,
    pub wrapped_key: Vec<u8>,
    pub wrap_type: String,
}
