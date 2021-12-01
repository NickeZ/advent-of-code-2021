use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = File::open(std::env::args().nth(1).unwrap()).unwrap();
    let mut line = String::new();
    input.read_to_string(&mut line).unwrap();
    let (_, count) = line
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .enumerate()
        .fold((0, 0), |(prev, count), (i, num)| {
            if i != 0 && num > prev {
                (num, count + 1)
            } else {
                (num, count)
            }
        });
    println!("{}", count);
}
