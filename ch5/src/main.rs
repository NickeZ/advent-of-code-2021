#![feature(let_else)]
use core::num::ParseIntError;
use std::fs::File;
use std::io::Read;

const SIZE:usize = 1000;

struct Map {
    counts: [[u16; SIZE]; SIZE]
}

impl Map {
    pub fn new() -> Map {
        Map {counts: [[0; SIZE]; SIZE] }
    }
    pub fn draw(&mut self, x1: u32, y1: u32, x2: u32, y2: u32) {
        let x1 = x1 as usize;
        let x2 = x2 as usize;
        let y1 = y1 as usize;
        let y2 = y2 as usize;
        if y1 == y2 {
            if x1 < x2 {
                for i in x1..=x2 {
                    self.counts[y1][i] += 1;
                }
            } else {
                for i in x2..=x1 {
                    self.counts[y1][i] += 1;
                }
            }
        } else { // x1 == x2
            if y1 < y2 {
                for i in y1..=y2 {
                    self.counts[i][x1] += 1;
                }
            } else {
                for i in y2..=y1 {
                    self.counts[i][x1] += 1;
                }
            }
        }
    }
    pub fn draw_diag(&mut self, x1: u32, y1: u32, x2: u32, y2: u32) {
        let x1 = x1 as usize;
        let x2 = x2 as usize;
        let y1 = y1 as usize;
        let y2 = y2 as usize;
        if y1 < y2 && x1 < x2 {
            for i in 0..=x2-x1 {
                self.counts[y1+i][x1+i] += 1;
            }
        }
        else if y1 < y2 && x1 > x2 {
            for i in 0..=x1-x2 {
                self.counts[y1+i][x1-i] += 1;
            }
        }
        else if y1 > y2 && x1 < x2 {
            for i in 0..=x2-x1 {
                self.counts[y1-i][x1+i] += 1;
            }
        }
        else if y1 > y2 && x1 > x2 {
            for i in 0..=x1-x2 {
                self.counts[y1-i][x1-i] += 1;
            }
        }
    }
    pub fn count_overlapping(&self) -> u32 {
        let mut count = 0;
        for i in 0..SIZE {
            for j in 0..SIZE {
                if self.counts[i][j] > 1 {
                    count += 1;
                }
            }
        }
        count
    }
}

fn parse_coor<'a>(tokens: &mut impl Iterator<Item=&'a str>) -> Result<Option<(u32, u32)>, ParseIntError> {
    let Some(coor) = tokens.next() else {
        return Ok(None)
    };
    let mut coor = coor.split(",");
    let Some(x) = coor.next() else {
        return Ok(None)
    };
    let Some(y) = coor.next() else {
        return Ok(None)
    };
    Ok(Some((x.parse()?, y.parse()?)))
}

fn parse_edge<'a>(tokens: &mut impl Iterator<Item=&'a str>) -> Result<Option<((u32, u32), (u32, u32))>, ParseIntError> {
    let Some(left) = parse_coor(tokens)? else {
        return Ok(None);
    };
    let _arrow = tokens.next().unwrap();
    let Some(right) = parse_coor(tokens)? else {
        return Ok(None);
    };
    Ok(Some((left, right)))
}

fn parse_coords_p1<'a>(mut tokens: impl Iterator<Item=&'a str>) -> Map {
    let mut m = Map::new();
    while let Some(edge) = parse_edge(&mut tokens).unwrap() {
        if edge.0.0 == edge.1.0 || edge.0.1 == edge.1.1 {
            m.draw(edge.0.0, edge.0.1, edge.1.0, edge.1.1);
        }
    }
    m
}

fn parse_coords_p2<'a>(mut tokens: impl Iterator<Item=&'a str>) -> Map {
    let mut m = Map::new();
    while let Some(edge) = parse_edge(&mut tokens).unwrap() {
        if edge.0.0 == edge.1.0 || edge.0.1 == edge.1.1 {
            m.draw(edge.0.0, edge.0.1, edge.1.0, edge.1.1);
        } else {
            m.draw_diag(edge.0.0, edge.0.1, edge.1.0, edge.1.1);
        }
    }
    m
}

fn main() {
    let mut input = File::open(std::env::args().nth(1).unwrap()).unwrap();
    let mut content = String::new();
    input.read_to_string(&mut content).unwrap();

    let m = parse_coords_p1(content.split_whitespace());
    println!("{}", m.count_overlapping());

    let m = parse_coords_p2(content.split_whitespace());
    println!("{}", m.count_overlapping());
}
