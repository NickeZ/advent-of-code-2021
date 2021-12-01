use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = File::open(std::env::args().nth(1).unwrap()).unwrap();
    let mut line = String::new();
    input.read_to_string(&mut line).unwrap();
    let mut prev = 0;
    let mut count_inc = 0;
    for (i, num) in line.split_whitespace().enumerate() {
        let num = num.parse().unwrap();
        if i != 0 && num > prev {
            count_inc += 1;
        }
        prev = num;

    }
    println!("{}", count_inc);
}
