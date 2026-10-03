use std::{error::Error, fmt::Display};

use siphasher::sip::SipHasher13;

const USIZE_LEN: usize = 64;

#[derive(Debug)]
pub struct BloomFilter {
    data: Vec<u64>,
    k: u8,
    m: usize,
}

#[derive(Debug)]
pub enum BloomError {
    InvalidN(u64),
    InvalidP(f64),
}

impl Display for BloomFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "k value is {}", self.k)?;
        writeln!(f, "m value is {}", self.m)?;
        writeln!(f, "number of bucket is {}", self.data.len())
    }
}

impl Error for BloomError {}

impl Display for BloomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BloomError::InvalidN(n) => writeln!(f, "n must be positive, got {n}"),
            BloomError::InvalidP(p) => writeln!(f, "p must be between 0 and 1, got {p}"),
        }
    }
}

impl BloomFilter {
    pub fn new(n: u64, p: f64) -> Result<Self, BloomError> {
        if n == 0 {
            return Err(BloomError::InvalidN(n));
        }
        if !(p > 0. && p < 1.) {
            return Err(BloomError::InvalidP(p));
        }
        let n = n as f64;
        let m = (-(n * p.ln()) / (2f64.ln().powi(2))).ceil();
        let k = ((m / n) * 2f64.ln()).ceil() as u8;
        let m = m as usize;
        let size = m.div_ceil(USIZE_LEN);
        Ok(BloomFilter {
            data: vec![0; size],
            k,
            m,
        })
    }

    pub fn contains(&self, data: &[u8]) -> bool {
        Self::compute_hash(self.k, self.m, data).all(|pos| {
            let block_index = pos / USIZE_LEN;
            let block = self.data[block_index];
            block & 1u64 << (pos % USIZE_LEN) != 0
        })
    }

    pub fn add(&mut self, data: &[u8]) {
        for pos in Self::compute_hash(self.k, self.m, data) {
            let block_index = pos / USIZE_LEN;
            let new_block = self.data[block_index] | 1u64 << (pos % USIZE_LEN);
            self.data[block_index] = new_block;
        }
    }

    fn compute_hash(k: u8, m: usize, data: &[u8]) -> impl Iterator<Item = usize> {
        (0..k).map(move |i| {
            let hasher = SipHasher13::new_with_key(&[i; 16]);
            let hash = hasher.hash(data);
            hash as usize % m
        })
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use super::*;

    #[test]
    fn should_compute_k_and_m() {
        // Given / When
        let bloom_filter = BloomFilter::new(1_000, 0.1).expect("must be Ok");

        // Then
        assert_eq!(bloom_filter.k, 4, "k must be 4");
        assert_eq!(bloom_filter.m, 4793, "m must be 4793");
        assert_eq!(bloom_filter.data.len(), 75, "buckets number must be 75");
    }

    #[test]
    fn should_fail_when_n_is_zero() {
        // Given / When
        let bloom_filter = BloomFilter::new(0, 0.1);

        // Then
        assert_matches!(bloom_filter, Err(BloomError::InvalidN(0)));
    }

    #[test]
    fn should_fail_when_p_is_negative() {
        // Given / When
        let bloom_filter = BloomFilter::new(10, -1.);

        // Then
        assert_matches!(
            bloom_filter,
            Err(BloomError::InvalidP(-1.)),
            "new function must return an error when p is negative"
        );
    }

    #[test]
    fn should_fail_when_p_is_greater_than_1() {
        // Given / When
        let bloom_filter = BloomFilter::new(10, 2.);

        // Then
        assert_matches!(
            bloom_filter,
            Err(BloomError::InvalidP(2.)),
            "new function must return an error when p i greater than 1"
        );
    }
}
