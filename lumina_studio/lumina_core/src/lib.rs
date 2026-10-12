pub mod db;
pub mod media;
pub mod audio;

use std::hash::{Hash, Hasher};
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
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

// Implement PartialEq consistent with Ord cross-multiplication
impl PartialEq for RationalTime {
    fn eq(&self, other: &Self) -> bool {
        let left = self.value as i128 * other.timescale as i128;
        let right = other.value as i128 * self.timescale as i128;
        left == right
    }
}

impl Eq for RationalTime {}

// Implement Hash consistent with PartialEq by simplifying the fraction
impl Hash for RationalTime {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Find greatest common divisor to normalize the fraction before hashing
        let mut a = self.value.abs() as u64;
        let mut b = self.timescale as u64;
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        let gcd = a.max(1); // Avoid div by 0

        let norm_val = self.value / (gcd as i64);
        let norm_scale = self.timescale / (gcd as u32);

        norm_val.hash(state);
        norm_scale.hash(state);
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
