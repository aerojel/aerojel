pub mod spec;
pub mod slice;
pub mod buffer;
pub mod source;
// TODO: pub mod fifo;

mod utils;

pub use source::PcmSource;
pub use spec::PcmSpec;

pub type PcmSample = f32;