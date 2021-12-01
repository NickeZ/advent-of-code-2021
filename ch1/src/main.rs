use itertools::Itertools;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = File::open(std::env::args().nth(1).unwrap()).unwrap();
    let mut line = String::new();
    input.read_to_string(&mut line).unwrap();

    let count = line
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .tuple_windows()
        .filter(|(a, b)| if b > a { true } else { false })
        .count();
    println!("{}", count);

    let count = line
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .tuple_windows()
        .tuple_windows()
        .filter(|((a1, a2, a3), (b1, b2, b3))| {
            if b1 + b2 + b3 > a1 + a2 + a3 {
                true
            } else {
                false
            }
        })
        .count();
    println!("{}", count);
}
