use std::fs::File;
use std::io::Read;

const SIZE: usize = 10;

struct Grid {
    height: u32,
    width: u32,
    data: [[u8; SIZE]; SIZE],
    has_flashed: [[bool; SIZE]; SIZE],
}

impl Grid {
    pub fn new(data: [[u8; SIZE]; SIZE]) -> Grid {
        Grid {height: SIZE as u32, width: SIZE as u32, data, has_flashed: [[false; SIZE]; SIZE]}
    }
    pub fn bump(&mut self, x: isize, y: isize) {
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            self.data[y as usize][x as usize] += 1;
        }
    }
    pub fn keys(&self) -> GridIterator {
        GridIterator {pos: 0, width: SIZE, height: SIZE}
    }
    pub fn flash(&mut self, x: isize, y: isize) -> bool {
        if self.has_flashed[y as usize][x as usize] {
            return false;
        }
        self.has_flashed[y as usize][x as usize] = true;
        for i in x-1..=x+1 {
            for j in y-1..=y+1 {
                self.bump(i, j)
            }
        }
        self.data[y as usize][x as usize] = 0;
        true
    }
    pub fn get(&self, x: isize, y: isize) -> u8 {
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            self.data[y as usize][x as usize]
        } else {
            0
        }
    }

    pub fn all_flashed(&self) -> bool {
        for (x, y) in self.keys() {
            if !self.has_flashed[y as usize][x as usize] {
                return false
            }
        }
        true
    }

    pub fn reset_flash_state(&mut self) {
        for (x, y) in self.keys() {
            if self.has_flashed[y as usize][x as usize] {
                self.data[y as usize][x as usize] = 0;
            }
        }
        self.has_flashed = [[false; SIZE]; SIZE];
    }
}

struct GridIterator {
    pos: usize,
    width: usize,
    height: usize,
}

impl Iterator for GridIterator {
    type Item = (isize, isize);
    fn next(&mut self) -> Option<(isize, isize)> {
        let cur = self.pos;
        self.pos += 1;
        if cur < self.width*self.height {
            Some(((cur%self.width) as isize, (cur/self.height) as isize))
        } else {
            None
        }
    }
}

fn main() {
    let mut input = File::open(std::env::args().nth(1).unwrap()).unwrap();
    let mut content = String::new();
    input.read_to_string(&mut content).unwrap();

    let mut data = [[0; SIZE]; SIZE];
    for (i, line) in content.trim().split_whitespace().enumerate() {
        let row = line.bytes().map(|c| c - b'0').collect::<Vec<u8>>();
        data[i].copy_from_slice(&row)
    }

    let mut grid = Grid::new(data);
    let mut count = 0;
    let all_flashed_step;
    let mut i = 0;
    loop {
        for (x, y) in grid.keys() {
            grid.bump(x, y);
        }
        loop {
            let mut flashed = false;
            for (x, y) in grid.keys() {
                if grid.get(x, y) > 9 && grid.flash(x, y) {
                    flashed = true;
                    if i < 100 {
                        count += 1;
                    }
                }
            }
            if !flashed {
                break;
            }
        }

        if grid.all_flashed() {
            all_flashed_step = i;
            break;
        }

        grid.reset_flash_state();
        i += 1;
    }
    println!("{}", count);
    println!("{}", all_flashed_step + 1);
}
