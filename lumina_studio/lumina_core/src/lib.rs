use std::hash::Hash;

// ============================================================================
// Precision Rational Time
// ============================================================================
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RationalTime {
    pub value: i64,
    pub timescale: u32,
}

impl RationalTime {
    pub fn new(value: i64, timescale: u32) -> Self {
        Self { value, timescale }
    }

    pub fn to_seconds(&self) -> f64 {
        self.value as f64 / self.timescale as f64
    }
}
