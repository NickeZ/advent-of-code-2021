use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

// vx goes towards 0 each step
// vy increases by 1 each step

fn main() {
    let mut input = File::open(std::env::args().nth(1).unwrap()).unwrap();
    let mut content = String::new();
    input.read_to_string(&mut content).unwrap();

    let mut split = content.split_whitespace();
    let _ = split.next().unwrap();
    let _ = split.next().unwrap();
    let x_range = split.next().unwrap().trim_end_matches(",");
    let y_range = split.next().unwrap();

    let mut x_range = std::str::from_utf8(&x_range.as_bytes()[2..]).unwrap().split("..");
    let x_range_min = x_range.next().unwrap().parse::<i32>().unwrap();
    let x_range_max = x_range.next().unwrap().parse::<i32>().unwrap();
    let x_range = x_range_min..=x_range_max;

    let mut y_range = std::str::from_utf8(&y_range.as_bytes()[2..]).unwrap().split("..");
    let y_range_min = y_range.next().unwrap().parse::<i32>().unwrap();
    let y_range_max = y_range.next().unwrap().parse::<i32>().unwrap();
    let y_range = y_range_min..=y_range_max;

    println!("{:?} {:?}", x_range, y_range);

    let mut set = HashSet::new();
    for vx in 0..1000 {
        for vy in -1000..1000 {
            let (is_good, max_y) = is_good(vx, vy, &x_range, &y_range);

            if is_good {
                set.insert((vx, vy, max_y));
            }
        }
    }
    println!("{:?}", set.iter().max_by_key(|(_, _, max_y)| max_y).unwrap());

    println!("{:?}", set.iter().count());
}

fn is_good(init_vx: i32, init_vy: i32, xrange: &std::ops::RangeInclusive<i32>, yrange: &std::ops::RangeInclusive<i32>) -> (bool, i32) {
    let mut x = 0;
    let mut vx = init_vx;
    let mut y = 0;
    let mut vy = init_vy;
    let mut max_y = 0;
    loop {
        x += vx;
        y += vy;
        if vx > 0 {
            vx -= 1;
        }
        vy -= 1;
        max_y = y.max(max_y);

        if xrange.contains(&x) && yrange.contains(&y) {
            return (true, max_y);
        }
        if x > *xrange.end() || y < *yrange.start() {
            return (false, 0);
        }
    }

}
