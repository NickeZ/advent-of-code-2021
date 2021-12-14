use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut input = File::open(std::env::args().nth(1).unwrap()).unwrap();
    let mut content = String::new();
    input.read_to_string(&mut content).unwrap();
    {
        let mut one_count = 0;
        let mut four_count = 0;
        let mut seven_count = 0;
        let mut eight_count = 0;
        for line in content.trim().split('\n') {
            let right = line.split('|').nth(1).unwrap();
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
        for line in content.trim().split('\n') {
            let left = line.split('|').next().unwrap().split_whitespace().map(|s| HashSet::from_iter(s.chars())).collect::<Vec<HashSet<_>>>();
            let integers = find_encoding(left);

            let mut code = String::new();
            let right = line.split('|').nth(1).unwrap();
            for num in right.split_whitespace().map(|s| HashSet::from_iter(s.chars())) {
                let i = integers.iter().enumerate().find(|x| x.1 == &num).unwrap();
                code.push_str(&i.0.to_string())
            }
            sum += str::parse::<u32>(&code).unwrap();
        }
        println!("{}", sum);
    }
}

fn find_encoding(left: Vec<HashSet<char>>) -> [HashSet<char>; 10] {
    let mut integers: [HashSet<char>; 10] = Default::default();
    for s in &left {
        // find 1
        if s.len() == 2 {
            integers[1] = s.clone();
        }
        // find 4
        if s.len() == 4 {
            integers[4] = s.clone();
        }
        // find 7
        if s.len() == 3 {
            integers[7] = s.clone();
        }
        // find 8
        if s.len() == 7 {
            integers[8] = s.clone();
        }
    }
    for s in &left {
        // find 3
        if s.len() == 5 && s.intersection(&integers[1]).count() == 2 {
            integers[3] = s.clone();
        }
        // find 9
        if s.len() == 6 && s.intersection(&integers[4]).count() == 4 {
            integers[9] = s.clone();
        }
        // find 0
        if s.len() == 6 && s.intersection(&integers[1]).count() == 2 && s.intersection(&integers[4]).count() == 3 {
            integers[0] = s.clone();
        }
    }
    // find 6
    for s in &left {
        if s.len() == 6 && s.intersection(&integers[0]).count() == 5 && s.intersection(&integers[9]).count() == 5 {
            integers[6] = s.clone();
            break;
        }
    }
    // find 5
    for s in &left {
        if s.len() == 5 && s.intersection(&integers[6]).count() == 5 {
            integers[5] = s.clone();
            break;
        }
    }
    // find 2
    for s in &left {
        if s.len() == 5 && s.intersection(&integers[5]).count() == 3 {
            integers[2] = s.clone();
            break;
        }
    }
    integers
}
