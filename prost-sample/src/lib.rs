use std::io::Cursor;

use prost::Message;

// Include the `items` module, which is generated from items.proto.
pub mod items {
    // include!(concat!(env!("OUT_DIR"), "/snazzy.items.rs"));
    include!(concat!("../generated", "/snazzy.items.rs"));
}

pub fn create_large_shirt(color: String) -> items::Shirt {
    // Let mut shirt = items::Shirt::default();
    // shirt.color = color;
    // shirt.set_size(items::shirt::Size::Large);
    // shirt
    items::Shirt {
        color,
        ..Default::default()
    }
}

pub fn serialize_shirt(shirt: &items::Shirt) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(shirt.encoded_len());
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    shirt.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_shirt(buf: &[u8]) -> Result<items::Shirt, prost::DecodeError> {
    items::Shirt::decode(&mut Cursor::new(buf))
}
