use std::fmt::Display;

use siphasher::sip::SipHasher13;

struct BloomFilter<const COUNT: usize> {
    data: [bool; COUNT],
    k: u8,
}

impl<const COUNT: usize> Display for BloomFilter<COUNT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, present) in self.data.iter().enumerate() {
            if *present {
                write!(f, "{index},")?;
            }
        }
        writeln!(f)
    }
}

impl<const COUNT: usize> BloomFilter<COUNT> {
    fn new(k: u8) -> Self {
        BloomFilter {
            data: [false; COUNT],
            k,
        }
    }

    fn contains(&self, data: &[u8]) -> bool {
        for pos in Self::compute_hash(self.k, data) {
            if !self.data[pos] {
                return false;
            }
        }
        true
    }

    fn add(&mut self, data: &[u8]) {
        for pos in Self::compute_hash(self.k, data) {
            self.data[pos] = true;
        }
    }

    fn compute_hash<'a>(k: u8, data: &'a [u8]) -> impl Iterator<Item = usize> + 'a {
        (0..k).map(|i| {
            let hasher = SipHasher13::new_with_key(&[i; 16]);
            let hash = hasher.hash(data);
            hash as usize % COUNT
        })
    }
}

fn main() {
    println!("Hello, world!");
    let mut bloom_filter: BloomFilter<128> = BloomFilter::new(5);
    bloom_filter.add(b"coucou");
    println!("{bloom_filter}");
    println!("data is contains {}", bloom_filter.contains(b"coucou"));
    println!("data is contains {}", bloom_filter.contains(b"coucou 2"));
    println!("data is contains {}", bloom_filter.contains(b"coucou 3"));
    println!("data is contains {}", bloom_filter.contains(b"coucou 4"));
    println!("data is contains {}", bloom_filter.contains(b"coucou 5"));
}
