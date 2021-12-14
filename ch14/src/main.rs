use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use itertools::Itertools;
use num_bigint::BigUint;

fn main() {
    let mut input = File::open(std::env::args().nth(1).unwrap()).unwrap();
    let mut content = String::new();
    input.read_to_string(&mut content).unwrap();

    let (init, inst) = {
        let mut init = String::new();
        let mut inst = HashMap::new();
        for (i, line) in content.trim().split('\n').enumerate() {
            if i == 0 {
                init = line.to_string();
            } else if i == 1 {
            } else {
                let mut split = line.split_whitespace();
                let key = split.next().unwrap();
                split.next().unwrap();
                let val = split.next().unwrap();
                inst.insert((key.as_bytes()[0], key.as_bytes()[1]), val.as_bytes()[0]);
            }
        }
        (init, inst)
    };
    {
        let mut res = init.clone();
        for _ in 0..10 {
            let last = res.as_bytes()[res.len()-1];
            res = String::from_utf8(res.bytes().tuple_windows().map(|x:(u8, u8)| [x.0, inst[&x]]).flatten().collect::<Vec<u8>>()).unwrap();
            res.push(last as char);
        }

        let mut bins = HashMap::new();
        for c in res.chars() {
            let counter = bins.entry(c).or_insert(0);
            *counter += 1;
        }

        let max = bins.values().max().unwrap();
        let min = bins.values().min().unwrap();
        println!("{}", max - min);
    }
    {
        let mut pair_counts = init.bytes().tuple_windows().fold(HashMap::new(), |mut acc:HashMap<(u8,u8), BigUint>, pair:(u8,u8)| {
            let counter = acc.entry(pair).or_default();
            *counter += 1u64;
            acc
        });

        let mut char_counts = init.bytes().fold(HashMap::new(), |mut acc:HashMap<u8, BigUint>, c| {
            let counter = acc.entry(c).or_default();
            *counter += 1u64;
            acc
        });

        for _ in 0..40 {
            char_counts = pair_counts.iter().fold(char_counts, |mut acc, (k, v)| {
                let counter = acc.entry(inst[k]).or_default();
                *counter += v;
                acc
            });
            pair_counts = pair_counts.iter().fold(HashMap::new(), |mut acc, (k, v)| {
                let counter = acc.entry((k.0, inst[k])).or_default();
                *counter += v;
                let counter = acc.entry((inst[k], k.1)).or_default();
                *counter += v;
                acc
            });
        }
        let max = char_counts.values().max().unwrap();
        let min = char_counts.values().min().unwrap();
        println!("{}", max - min);
    }
}
