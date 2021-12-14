use std::fs::File;
use std::io::Read;
fn main() {
    let mut input = File::open(std::env::args().nth(1).unwrap()).unwrap();
    let mut content = String::new();
    input.read_to_string(&mut content).unwrap();
    {
        let heights = content.trim().split(',').map(str::parse).map(Result::unwrap).collect::<Vec<u32>>();
        let max = heights.iter().copied().reduce(u32::max).unwrap();
        let mut best_height_cost = u32::max_value();
        for i in 0..max {
            let i_cost = cost_to_realign(&heights, i);
            if i_cost < best_height_cost {
                best_height_cost = i_cost;
            }
        }

        println!("{}", best_height_cost);

    }

    {
        let heights = content.trim().split(',').map(str::parse).map(Result::unwrap).collect::<Vec<u32>>();
        let max = heights.iter().copied().reduce(u32::max).unwrap();
        let mut best_height_cost = u32::max_value();
        for i in 0..max {
            let i_cost = cost_to_realign_2(&heights, i);
            if i_cost < best_height_cost {
                best_height_cost = i_cost;
            }
        }

        println!("{}", best_height_cost);
    }
}

fn cost_to_realign(heights: &[u32], level: u32) -> u32 {
    heights.iter().copied().map(|x| (level as i32 - x as i32).abs()).sum::<i32>() as u32
}

fn cost_to_realign_2(heights: &[u32], level: u32) -> u32 {
    heights.iter().copied().map(|x| {
        let abs = (level as i32 - x as i32).abs();
        abs * (abs + 1) / 2
    }).sum::<i32>() as u32
}

#[test]
fn testytest() {
    assert!(0 == cost_to_realign_2(&[1], 1));
    assert!(1 == cost_to_realign_2(&[1], 2));
    assert!(3 == cost_to_realign_2(&[1], 3));
}

