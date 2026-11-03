#[derive(Clone)]
pub enum Image {
    Bytes(bytes::Bytes),
    Rgba {
        pixels: bytes::Bytes,
        width: u32,
        height: u32,
    }
}

impl Image {
    pub fn empty_rgba() -> Self {
        Self::Rgba {
            pixels: bytes::Bytes::new(),
            width: 0,
            height: 0,
        }
    }
}