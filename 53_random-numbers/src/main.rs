// Rust's `rand` crate provides pseudorandom number generation.

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

fn main() {
    let mut rng = rand::thread_rng();

    // rand.Intn(100) equivalent: gen_range(0..100)
    print!("{},", rng.gen_range(0..100));
    print!("{}", rng.gen_range(0..100));
    println!();

    // rand.Float64() equivalent: gen::<f64>()
    println!("{}", rng.gen::<f64>());

    // Random floats in range 5.0..10.0.
    print!("{},", rng.gen_range(5.0f64..10.0));
    print!("{}", rng.gen_range(5.0f64..10.0));
    println!();

    // Seeded RNG for reproducible sequences (time-seeded here).
    // Go: rand.NewSource(time.Now().UnixNano())
    // Rust: rand::thread_rng() already uses a random seed.
    // For an explicit time seed, use SeedableRng with a u64 seed.
    let seed1 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    let mut r1 = StdRng::seed_from_u64(seed1);
    print!("{},", r1.gen_range(0..100));
    print!("{}", r1.gen_range(0..100));
    println!();

    // Fixed seed produces the same sequence every time.
    let mut r2 = StdRng::seed_from_u64(42);
    print!("{},", r2.gen_range(0..100));
    print!("{}", r2.gen_range(0..100));
    println!();

    // Same seed again produces the same sequence.
    let mut r3 = StdRng::seed_from_u64(42);
    print!("{},", r3.gen_range(0..100));
    print!("{}", r3.gen_range(0..100));
}
