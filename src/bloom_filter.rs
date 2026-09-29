use std::fmt::Display;

use siphasher::sip::SipHasher13;

const USIZE_LEN: usize = 64;

pub struct BloomFilter {
    data: Vec<u64>,
    k: u8,
    m: usize,
}

impl Display for BloomFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, &present) in self.data.iter().enumerate() {
            if present != 0 {
                write!(f, "{index},")?;
            }
        }
        writeln!(f)
    }
}

impl BloomFilter {
    pub fn new(n: u64, p: f64) -> Self {
        let n = n as f64;
        let m = (-(n * p.ln()) / (2f64.ln().powi(2))).ceil();
        let k = ((m / n) * 2f64.ln()).ceil() as u8;
        let m = m as usize;
        let size = m.div_ceil(USIZE_LEN);
        println!("k value is {k}");
        println!("m value is {m}");
        println!("size value is {size}");
        BloomFilter {
            data: vec![0; size],
            k,
            m,
        }
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
