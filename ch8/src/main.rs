use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = File::open(std::env::args().nth(1).unwrap()).unwrap();
    let mut content = String::new();
    input.read_to_string(&mut content).unwrap();
    {
        let mut one_count = 0;
        let mut four_count = 0;
        let mut seven_count = 0;
        let mut eight_count = 0;
        for line in content.trim().split("\n") {
            let right = line.split("|").nth(1).unwrap();
            let word_sizes = right.split_whitespace().map(str::len);
            for size in word_sizes {
                match size {
                    2 => one_count += 1,
                    3 => seven_count += 1,
                    4 => four_count += 1,
                    7 => eight_count += 1,
                    _ => ()
                }
            }
        }
        println!("{}", one_count + seven_count + four_count + eight_count);
    }
    {
        let mut sum = 0u32;
        for line in content.trim().split("\n") {
            let left = line.split("|").next().unwrap();
            let right = line.split("|").nth(1).unwrap();

            let integers = find_encoding(&left);

            let mut code = String::new();
            for num in right.split_whitespace() {
                let i = integers.iter().enumerate().find(|x| str_eq_set(x.1, num)).unwrap();
                code.push_str(&i.0.to_string())
            }
            sum += str::parse::<u32>(&code).unwrap();
        }
        println!("{}", sum);
    }
}

fn find_encoding<'a>(left: &str) -> [String; 10] {
    let left = left.split_whitespace();
    let mut integers: [String; 10] = Default::default();
    // find 1
    for s in left.clone() {
        if s.len() == 2 {
            integers[1] = String::from(s);
            break;
        }
    }
    // find 4
    for s in left.clone() {
        if s.len() == 4 {
            integers[4] = String::from(s);
            break;
        }
    }
    // find 7
    for s in left.clone() {
        if s.len() == 3 {
            integers[7] = String::from(s);
            break;
        }
    }
    // find 8
    for s in left.clone() {
        if s.len() == 7 {
            integers[8] = String::from(s);
            break;
        }
    }
    // find 3
    for s in left.clone() {
        if s.len() == 5 && str_union(s, &integers[1]).len() == 2 {
            integers[3] = String::from(s);
            break;
        }
    }
    // find 9
    for s in left.clone() {
        if s.len() == 6 && str_union(s, &integers[4]).len() == 4 {
            integers[9] = String::from(s);
            break;
        }
    }
    // find 0
    for s in left.clone() {
        if s.len() == 6 && str_union(s, &integers[1]).len() == 2 && str_union(s, &integers[4]).len() == 3 {
            integers[0] = String::from(s);
            break;
        }
    }
    // find 6
    for s in left.clone() {
        if s.len() == 6 && str_union(s, &integers[0]).len() == 5 && str_union(s, &integers[9]).len() == 5 {
            integers[6] = String::from(s);
            break;
        }
    }
    // find 5
    for s in left.clone() {
        if s.len() == 5 && str_union(s, &integers[6]).len() == 5 {
            integers[5] = String::from(s);
            break;
        }
    }
    // find 2
    for s in left.clone() {
        if s.len() == 5 && str_union(s, &integers[5]).len() == 3 {
            integers[2] = String::from(s);
            break;
        }
    }
    integers
}

pub fn str_union(a: &str,b: &str) -> String {
    let mut s = String::new();
    for item in a.chars() {
        if let Some(found) = b.chars().find(|x| *x ==item) {
            s.push(found)
        }
    }
    s
}

pub fn str_eq_set(a: &str, b: &str) -> bool {
    a.len() == b.len() && str_union(a, b).len() == a.len()
}
