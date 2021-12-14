use std::fs::File;
use std::io::Read;
use num_bigint::BigUint;

fn main() {
    let mut input = File::open(std::env::args().nth(1).unwrap()).unwrap();
    let mut content = String::new();
    input.read_to_string(&mut content).unwrap();
    {
        let fishes_init: Vec<u8> = content.trim().split(',').map(str::parse).map(Result::unwrap).collect();

        let mut fishes: Vec<BigUint> = Vec::new();
        fishes.resize_with(9, || BigUint::from(0u64));
        for f in &fishes_init {
            fishes[*f as usize] += BigUint::from(1u64);
        }

        for i in 0..256 {
            let fishes_0 = fishes[0].clone();

            for bin in 1..=8 {
                fishes[bin-1] = fishes[bin].clone();
            }
            fishes[6] += fishes_0.clone();
            fishes[8] = fishes_0.clone();
            if i == 79 {
                println!("80 days: {}", fishes.iter().sum::<BigUint>());
            }
        }
        println!("256 days: {}", fishes.iter().sum::<BigUint>());
    }
}
