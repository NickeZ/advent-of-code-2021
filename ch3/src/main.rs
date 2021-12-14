use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = File::open(std::env::args().nth(1).unwrap()).unwrap();
    let mut content = String::new();
    input.read_to_string(&mut content).unwrap();

    {
        let mut ones = Vec::new();
        let mut count = 0;

        for line in content.split_whitespace() {
            for (i, b) in line.bytes().map(|c| (c - b'0') as u32).enumerate() {
                if i >= ones.len() {
                    ones.push(0u32);
                }
                ones[i] += b;
            }
            count += 1;
        }


        let mut gamma = 0u32;
        let mut epsilon = 0u32;

        for item in ones {
            gamma <<= 1;
            epsilon <<= 1;
            gamma += if item > count/2 {1} else {0};
            epsilon += if item > count/2 {0} else {1};
        }

        println!("{}", gamma*epsilon);
    }

    let mut index_ox = Vec::new();
    let mut index_co2 = Vec::new();
    let mut count_ox = 0;
    let mut count_co2 = 0;
    let mut total_ox = 0;
    let mut total_co2 = 0;
    let mut pos = 0;
    let mut ox_rate_idx = 0;
    let mut co2_rate_idx = 0;

    loop {
        for (i, line) in content.split_whitespace().enumerate() {
            if i >= index_ox.len() {
                index_ox.push(true)
            }
            if i >= index_co2.len() {
                index_co2.push(true)
            }
            let b = (line.as_bytes()[pos] - b'0') as usize;
            if index_ox[i] {
                count_ox += b;
                total_ox += 1;
            }
            if index_co2[i] {
                count_co2 += b;
                total_co2 += 1;
            }
        }

        for (i, line) in content.split_whitespace().enumerate() {
            if total_ox > 1 {
                if count_ox*2 >= total_ox {
                    if line.as_bytes()[pos] == b'0' {
                        index_ox[i] = false;
                    }
                } else if line.as_bytes()[pos] == b'1' {
                    index_ox[i] = false;
                }
            }
            if total_co2 > 1 {
                if count_co2*2 >= total_co2 {
                    if line.as_bytes()[pos] == b'1' {
                        index_co2[i] = false;
                    }
                } else if line.as_bytes()[pos] == b'0' {
                    index_co2[i] = false;
                }
            }
        }

        count_ox = 0;
        for (i, _b) in index_ox.iter().enumerate().filter(|b| *b.1) {
            count_ox += 1;
            ox_rate_idx = i;
        }
        count_co2 = 0;
        for (i, _b) in index_co2.iter().enumerate().filter(|b| *b.1) {
            count_co2 += 1;
            co2_rate_idx = i;
        }
        if count_ox == 1 && count_co2 == 1 {
            break;
        }

        pos += 1;
        count_ox = 0;
        count_co2 = 0;
        total_ox = 0;
        total_co2 = 0;

    }

    let ox_rate = i32::from_str_radix(content.split_whitespace().nth(ox_rate_idx).unwrap(), 2).unwrap();
    let co2_rate = i32::from_str_radix(content.split_whitespace().nth(co2_rate_idx).unwrap(), 2).unwrap();

    println!("{}", ox_rate*co2_rate);

}
