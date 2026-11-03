use std::sync::Arc;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AerojelId {
    InlineBytes {
        data: [u8; 16],
        len: u8,
    },
    HeapBytes(Arc<[u8]>),
}

impl AerojelId {
    pub const fn from_i32(id: i32) -> Self {
        let data = id as i128;

        Self::InlineBytes {
            data: data.to_ne_bytes(),
            len: size_of::<i32>() as u8,
        }
    }
}
