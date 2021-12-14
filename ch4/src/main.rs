use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Board {
    pub id: usize,
    pub nums: [[u8; 5]; 5],
    pub marked: [[bool; 5]; 5],
}

impl Board {
    pub fn new(id: usize, nums: [[u8; 5]; 5]) -> Board {
        Board {
            id,
            nums,
            marked: [[false; 5]; 5],
        }
    }

    pub fn mark(&mut self, n: u8) {
        for i in 0..5 {
            for j in 0..5 {
                if self.nums[i][j] == n {
                    self.marked[i][j] = true;
                }
            }
        }
    }

    pub fn score(&self) -> u32 {
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.marked[i][j] {
                    sum += self.nums[i][j] as u32
                }
            }
        }
        sum
    }

    pub fn won(&self) -> bool {
        for i in 0..5 {
            let mut count_row = 0;
            let mut count_col = 0;
            for j in 0..5 {
                if self.marked[i][j] {
                    count_row += 1
                }
                if self.marked[j][i] {
                    count_col += 1
                }
            }
            if count_row == 5 {
                return true;
            }
            if count_col == 5 {
                return true;
            }
        }
        false
    }
}

fn parse_drawn(input: &str) -> Vec<u8> {
    input
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn parse_boards<'a>(
    input: &'a mut impl Iterator<Item = &'a str>,
) -> Result<Vec<Board>, std::num::ParseIntError> {
    let mut boards = Vec::new();

    loop {
        let mut nums = [[0; 5]; 5];
        for row in &mut nums {
            for cell in row.iter_mut() {
                if let Some(num) = input.next() {
                    *cell = num.parse()?;
                } else {
                    return Ok(boards);
                }
            }
        }
        boards.push(Board::new(boards.len(), nums))
    }
}

fn any_won(boards: &[Board]) -> Option<usize> {
    for (i, b) in boards.iter().enumerate() {
        if b.won() {
            return Some(i);
        }
    }
    None
}

fn winners(boards: &[Board], removed: &[bool]) -> Vec<usize> {
    let mut res = Vec::new();
    for (i, b) in boards.iter().enumerate() {
        if b.won() && !removed[i] {
            res.push(i);
        }
    }
    res
}

fn last_winner(boards: &[Board], removed: &mut [bool]) -> Option<usize> {
    for won in winners(boards, removed) {
        if removed.iter().filter(|x| **x).count() < boards.len() - 1 {
            removed[won] = true;
        } else {
            return Some(won)
        }
    }
    None
}

fn main() {
    let mut input = File::open(std::env::args().nth(1).unwrap()).unwrap();
    let mut content = String::new();
    input.read_to_string(&mut content).unwrap();

    {
        let mut split = content.split_whitespace();
        let drawn = parse_drawn(split.next().unwrap());
        let mut boards = parse_boards(&mut split).unwrap();

        let mut winner = 0;
        let mut final_num = 0;

        for n in drawn {
            for b in &mut boards {
                b.mark(n)
            }
            if let Some(won) = any_won(&boards) {
                winner = won;
                final_num = n;
                break;
            }
        }

        println!(
            "Board {} won with {}",
            winner,
            final_num as u32 * boards[winner].score()
        );
    }

    {
        let mut split = content.split_whitespace();

        let drawn = parse_drawn(split.next().unwrap());
        let mut boards = parse_boards(&mut split).unwrap();
        let mut removed = Vec::new();
        removed.resize_with(boards.len(), ||{false});

        let mut winner = 0;
        let mut final_num = 0;

        for n in drawn {
            for b in &mut boards {
                b.mark(n)
            }
            if let Some(won) = last_winner(&boards, &mut removed) {
                winner = won;
                final_num = n;
                break;
            }
        }

        println!(
            "Board {} won last with {} {}",
            boards[winner].id,
            final_num as u32 * boards[winner].score(),
            final_num
        );
    }
}
