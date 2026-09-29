mod bloom_filter;

use bloom_filter::BloomFilter;

fn main() {
    println!("Hello, world!");
    let mut bloom_filter: BloomFilter = BloomFilter::new(100, 0.01);
    // let mut bloom_filter: BloomFilter = BloomFilter::new(10, 0.0001);
    bloom_filter.add(b"coucou");
    println!("{bloom_filter}");
    println!("data is contains {}", bloom_filter.contains(b"coucou"));
    println!("data is contains {}", bloom_filter.contains(b"coucou 2"));
    println!("data is contains {}", bloom_filter.contains(b"coucou 3"));
    println!("data is contains {}", bloom_filter.contains(b"coucou 4"));
    println!("data is contains {}", bloom_filter.contains(b"coucou 5"));
}
