use std::fs::File;
use std::io::Read;

struct Map {
    width: usize,
    height: usize,
    data: Vec<u32>,
    mask: Vec<u32>,
    cave_count: u32,
    cave_sizes: Vec<u32>
}

struct MapIterator {
    width: usize,
    height: usize,
    pos: isize,
}

impl Map {
    pub fn get(&self, x: isize, y: isize) -> u32 {
        if x < self.width as isize && x >= 0 && y < self.height as isize && y >= 0 {
            self.data[(y as usize)*self.width + x as usize]
        } else {
            u32::max_value()
        }
    }

    pub fn fill(&mut self, x: isize, y: isize) {
        self.cave_count += 1;
        self.fill_priv(x, y);
    }

    fn fill_priv(&mut self, x: isize, y: isize) {
        if self.get(x, y) < 9 && self.mask[(y as usize)*self.width + x as usize] == 0 {
            self.mask[(y as usize)*self.width + x as usize] = self.cave_count;
            self.fill_priv(x-1, y);
            self.fill_priv(x+1, y);
            self.fill_priv(x, y-1);
            self.fill_priv(x, y+1);
            if self.cave_sizes.len() <= self.cave_count as usize {
                self.cave_sizes.push(1)
            } else {
                self.cave_sizes[self.cave_count as usize] += 1;
            }
        }
    }

    pub fn keys(&self) -> MapIterator {
        MapIterator {
            width: self.width,
            height: self.height,
            pos: 0
        }
    }
}

impl Iterator for MapIterator {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<(isize, isize)> {
        if self.pos == (self.width*self.height) as isize {
            None
        } else {
            let cur = self.pos;
            self.pos += 1;
            Some((cur%(self.width as isize), cur/(self.width as isize)))
        }
    }
}


fn main() {
    let mut input = File::open(std::env::args().nth(1).unwrap()).unwrap();
    let mut content = String::new();
    input.read_to_string(&mut content).unwrap();
    let width = content.split_whitespace().next().unwrap().len();
    let height = content.trim().split_whitespace().count();
    let data = content.trim().split_whitespace().flat_map(|s| s.bytes().map(|x| (x - b'0') as u32)).collect::<Vec<u32>>();
    let mask = {
        let mut mask = Vec::new();
        mask.resize_with(data.len(), || 0);
        mask
    };
    let mut map = Map {width, height, data, mask, cave_count: 0, cave_sizes: Vec::new()};

    let mut count = 0;
    for (x, y) in map.keys() {
        if is_low_point(&map, x, y) {
            count += map.get(x, y) + 1;
            map.fill(x, y);
        }
    }
    println!("{}", count);

    let mut product = 1;
    for _ in 0..3 {
        if let Some((index, largest)) = map.cave_sizes.iter().copied().enumerate().max_by_key(|x| x.1) {
            map.cave_sizes.remove(index);
            product *= largest
        };
    }
    println!("{}", product);
}

fn is_low_point(map: &Map, x: isize, y: isize) -> bool {
    let val = map.get(x, y);
    map.get(x-1, y) > val && map.get(x+1, y) > val && map.get(x, y-1) > val && map.get(x, y+1) > val
}
