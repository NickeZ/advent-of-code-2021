use std::fs::File;
use std::io::Read;
use std::collections::VecDeque;

const SIZE: usize = 1500;

#[derive(Debug)]
enum FoldType {
    Horizontal,
    Vertical,
}

fn main() {
    let mut input = File::open(std::env::args().nth(1).unwrap()).unwrap();
    let mut content = String::new();
    input.read_to_string(&mut content).unwrap();



    let mut data = Box::new([[false; SIZE]; SIZE]);
    let mut instructions = VecDeque::new();
    for line in content.trim().split_whitespace() {
        if line.split(",").count() == 2 {
            let mut split = line.split(",");
            let x = split.next().unwrap().parse::<usize>().unwrap();
            let y = split.next().unwrap().parse::<usize>().unwrap();
            data[y][x] = true
        } else {
            if line.starts_with("y=") {
                let y = std::str::from_utf8(&line.as_bytes()[2..]).unwrap().parse::<usize>().unwrap();
                instructions.push_back((FoldType::Horizontal, y))
            } else if line.starts_with("x=") {
                let x = std::str::from_utf8(&line.as_bytes()[2..]).unwrap().parse::<usize>().unwrap();
                instructions.push_back((FoldType::Vertical, x))
            }
        }
    }

    let mut first = true;
    while let Some(inst) = instructions.pop_front() {
        match inst.0 {
            FoldType::Horizontal => {
                for i in 0..inst.1 {
                    for x in 0..data[i].len() {
                        data[i][x] |= data[2 * inst.1 - i][x];
                        data[2 * inst.1 - i][x] = false;
                    }
                }
            },
            FoldType::Vertical => {
                for i in 0..inst.1 {
                    for y in 0..data[i].len() {
                        data[y][i] |= data[y][2 * inst.1 - i];
                        data[y][2 * inst.1 - i] = false;
                    }
                }
            }
        }
        if first {
            let mut count = 0;
            for row in data.iter() {
                for cell in row {
                    if *cell {
                        count += 1
                    }
                }
            }
            println!("{}", count);
            first = false;
        }
    }

    for row in data.iter().take(6) {
        for cell in &row[..39] {
            print!("{}", if *cell {'#'} else {'.'});
        }
        println!();
    }

}
