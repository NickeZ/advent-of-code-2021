use std::fs::File;
use std::io::Read;

#[derive(PartialEq, Debug)]
enum Token {
    LeftParen,
    LeftSquare,
    LeftCurly,
    LeftPointy,
    RightParen,
    RightSquare,
    RightCurly,
    RightPointy,
}

fn parse(b: u8) -> Token {
    match b {
        b'(' => Token::LeftParen,
        b'[' => Token::LeftSquare,
        b'{' => Token::LeftCurly,
        b'<' => Token::LeftPointy,
        b')' => Token::RightParen,
        b']' => Token::RightSquare,
        b'}' => Token::RightCurly,
        b'>' => Token::RightPointy,
        _ => panic!("invalid byte"),
    }
}

fn main() {
    let mut input = File::open(std::env::args().nth(1).unwrap()).unwrap();
    let mut content = String::new();
    input.read_to_string(&mut content).unwrap();

    let mut global_errors = [0u32; 4];
    let mut autocomplete_scores = Vec::new();
    for line in content.trim().split_whitespace() {
        let mut stack = std::collections::VecDeque::new();
        let mut errors = [0u32; 4];
        for tok in line.bytes().map(parse) {
            use Token::*;
            match tok {
                LeftParen => {
                    stack.push_back(RightParen);
                },
                RightParen => {
                    if let Some(t) = stack.pop_back() {
                        if t != RightParen {
                            errors[0] += 1;
                            break;
                        }
                    }
                }
                LeftSquare => {
                    stack.push_back(RightSquare);
                },
                RightSquare => {
                    if let Some(t) = stack.pop_back() {
                        if t != RightSquare {
                            errors[1] += 1;
                            break;
                        }
                    }
                }
                LeftCurly => {
                    stack.push_back(RightCurly);
                },
                RightCurly => {
                    if let Some(t) = stack.pop_back() {
                        if t != RightCurly {
                            errors[2] += 1;
                            break;
                        }
                    }
                }
                LeftPointy => {
                    stack.push_back(RightPointy);
                },
                RightPointy => {
                    if let Some(t) = stack.pop_back() {
                        if t != RightPointy {
                            errors[3] += 1;
                            break;
                        }
                    }
                }
            }
        }
        for i in 0..4 {
            global_errors[i] += errors[i]
        }
        if errors == [0, 0, 0, 0] {
            let mut autocomplete_score = 0u64;
            while let Some(tok) = stack.pop_back() {
                use Token::*;
                let p = match tok {
                    RightParen => 1,
                    RightSquare => 2,
                    RightCurly => 3,
                    RightPointy => 4,
                    _ => panic!("invalid token"),
                };
                autocomplete_score *= 5;
                autocomplete_score += p;
            }
            autocomplete_scores.push(autocomplete_score);
        }
    }
    let points = global_errors[0] * 3 + global_errors[1] * 57 + global_errors[2] * 1197 + global_errors[3] * 25137;
    println!("{}", points);

    autocomplete_scores.sort();
    println!("{}", autocomplete_scores[autocomplete_scores.len()/2])
}
