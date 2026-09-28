use std::fmt::Display;

use siphasher::sip::SipHasher13;

struct BloomFilter {
    data: Vec<bool>,
    k: u8,
    m: usize,
}

impl Display for BloomFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, &present) in self.data.iter().enumerate() {
            if present {
                write!(f, "{index},")?;
            }
        }
        writeln!(f)
    }
}

impl BloomFilter {
    fn new(n: u64, p: f64) -> Self {
        let n = n as f64;
        let m = (-(n * p.ln()) / (2f64.ln().powi(2))).ceil();
        let k = ((m / n) * 2f64.ln()).ceil() as u8;
        let m = m as usize;
        println!("k value is {k}");
        println!("m value is {m}");
        BloomFilter {
            data: vec![false; m],
            k,
            m,
        }
    }

    fn contains(&self, data: &[u8]) -> bool {
        for pos in Self::compute_hash(self.k, self.m, data) {
            if !self.data[pos] {
                return false;
            }
        }
        true
    }

    fn add(&mut self, data: &[u8]) {
        for pos in Self::compute_hash(self.k, self.m, data) {
            self.data[pos] = true;
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

fn main() {
    println!("Hello, world!");
    let mut bloom_filter: BloomFilter = BloomFilter::new(1_000_000, 0.01);
    bloom_filter.add(b"coucou");
    println!("{bloom_filter}");
    println!("data is contains {}", bloom_filter.contains(b"coucou"));
    println!("data is contains {}", bloom_filter.contains(b"coucou 2"));
    println!("data is contains {}", bloom_filter.contains(b"coucou 3"));
    println!("data is contains {}", bloom_filter.contains(b"coucou 4"));
    println!("data is contains {}", bloom_filter.contains(b"coucou 5"));
}
