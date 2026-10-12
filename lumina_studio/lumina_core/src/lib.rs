pub mod db;
pub mod media;
pub mod audio;

use std::hash::Hash;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

// Custom ordering is required to compare fractions properly (cross multiplication)
impl PartialOrd for RationalTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RationalTime {
    fn cmp(&self, other: &Self) -> Ordering {
        let left = self.value as i128 * other.timescale as i128;
        let right = other.value as i128 * self.timescale as i128;
        left.cmp(&right)
    }
}
