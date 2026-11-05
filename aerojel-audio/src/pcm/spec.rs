#[derive(Clone, Copy)]
pub struct PcmSpec {
    sample_rate: u32,
    plane_count: usize,
}

impl PcmSpec {
    pub const fn new(
        sample_rate: u32,
        plane_count: usize,
    ) -> Self {
        Self {
            sample_rate,
            plane_count
        }
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn plane_count(&self) -> usize {
        self.plane_count
    }
}