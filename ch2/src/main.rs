use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = File::open(std::env::args().nth(1).unwrap()).unwrap();
    let mut content = String::new();
    input.read_to_string(&mut content).unwrap();

    let mut x = 0i32;
    let mut y = 0i32;

    let mut lines = content.split_whitespace();

    while let Some(next) = lines.next() {
        let val = lines.next().unwrap().parse::<i32>().unwrap();
        match next {
            "forward" => x += val,
            "down" => y += val,
            "up" => y -= val,
            _ => panic!("invalid token"),
        }
    }

    println!("{}", x*y);

    let mut x = 0i32;
    let mut y = 0i32;
    let mut aim = 0i32;

    let mut lines = content.split_whitespace();


    while let Some(next) = lines.next() {
        let val = lines.next().unwrap().parse::<i32>().unwrap();
        match next {
            "forward" => {
                x += val;
                y += aim*val;
            },
            "down" => {
                aim += val;
            },
            "up" => {
                aim -= val;
            },
            _ => panic!("invalid token"),
        }
    }

    println!("{}", x*y);
}
