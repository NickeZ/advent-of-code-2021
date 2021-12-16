use std::fs::File;
use std::io::Read;
use itertools::Itertools;

#[derive(Debug)]
enum Packet {
    Literal(Literal),
    Operator(Operator)
}

#[derive(Debug)]
struct Literal {
    version: u8,
    num: u64,
}

#[derive(Debug)]
struct Operator {
    version: u8,
    type_id: u8,
    subpackets: Vec<Packet>
}

impl Packet {
    fn from_bytes(bytes: &[u8], stream_offset: usize) -> (Self, usize) {
        let version = get_bits(bytes, stream_offset, 3);
        let typ = get_bits(bytes, stream_offset+3, 3) as u8;
        match typ {
            4 => {
                let mut offset = stream_offset + 6;
                let mut leading_bit = get_bit(bytes, offset);
                let mut num:u64 = 0;
                loop {
                    num <<= 4;
                    num |= get_bits(bytes, offset+1, 4);
                    offset += 5;
                    if leading_bit == 0 {
                        break;
                    }
                    leading_bit = get_bit(bytes, offset);
                }
                (Packet::Literal(Literal {version: version as u8, num}), offset - stream_offset)
            },
            _ => {
                let mut subpackets = Vec::new();
                let mut offset = stream_offset + 6;
                match get_bit(bytes, offset) {
                    0 => { // 15-bit payload length
                        offset += 1;
                        let payload_len = get_bits(bytes, offset, 15);
                        offset += 15;
                        while offset < stream_offset + 16 + payload_len as usize {
                            let (p, read_len) = Packet::from_bytes(bytes, offset);
                            subpackets.push(p);
                            offset += read_len;
                        }
                    },
                    1 => { // 11-bit number of packets
                        offset += 1;
                        let payload_count = get_bits(bytes, offset, 11);
                        offset += 11;
                        for _ in 0..payload_count {
                            let (p, read_len) = Packet::from_bytes(bytes, offset);
                            subpackets.push(p);
                            offset += read_len;
                        }
                    },
                    _ => panic!("should be 0 or 1"),
                }
                (Packet::Operator(Operator {version: version as u8, type_id: typ, subpackets}), offset - stream_offset)
            }
        }
    }
}

fn to_hex(s: &str) -> Vec<u8> {
    s.chars().map(|x| x.to_digit(16).unwrap() as u8).tuples().map(|(a, b)| a << 4 | b).collect()
}

fn get_bit(bytes: &[u8], offset: usize) -> u8 {
    bytes[offset/8] >> (7 - (offset - (offset/8)*8)) & 0x1
}

fn get_bits(bytes: &[u8], offset: usize, len: usize) -> u64 {
    let mut num = 0u64;
    for i in 0..len {
        num |= ((get_bit(bytes, offset+i)) as u64) << (len-i-1)
    }
    num
}

fn sum_versions(p: &Packet) -> usize {
    match p {
        Packet::Literal(l) => l.version as usize,
        Packet::Operator(o) => o.version as usize + o.subpackets.iter().fold(0usize, |acc, x| {
            acc + sum_versions(x)
        }),
    }
}

fn value_transmission(p: &Packet) -> usize {
    match p {
        Packet::Literal(l) => l.num as usize,
        Packet::Operator(o) => {
            match o.type_id {
                0 => o.subpackets.iter().fold(0usize, |acc, x| {
                    acc + value_transmission(x)
                }),
                1 => o.subpackets.iter().fold(1usize, |acc, x| {
                    acc * value_transmission(x)
                }),
                2 => o.subpackets.iter().fold(usize::max_value(), |acc, x| {
                    acc.min(value_transmission(x))
                }),
                3 => o.subpackets.iter().fold(usize::min_value(), |acc, x| {
                    acc.max(value_transmission(x))
                }),
                5 => if value_transmission(&o.subpackets[0]) > value_transmission(&o.subpackets[1]) {1} else {0},
                6 => if value_transmission(&o.subpackets[0]) < value_transmission(&o.subpackets[1]) {1} else {0},
                7 => if value_transmission(&o.subpackets[0]) == value_transmission(&o.subpackets[1]) {1} else {0},
                _ => panic!("invalid type_id"),
            }
        }
    }
}

fn main() {
    let mut input = File::open(std::env::args().nth(1).unwrap()).unwrap();
    let mut content = String::new();
    input.read_to_string(&mut content).unwrap();

    let (p, _) = Packet::from_bytes(&to_hex(content.trim()), 0);

    println!("{}", sum_versions(&p));
    println!("{}", value_transmission(&p));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lit() {
        let h = to_hex("D2FE28");
        let (p, _) = Packet::from_bytes(&h, 0);
        println!("{:?}", p);
        if let Packet::Literal(l) = p {
            assert_eq!(l.version, 6);
            assert_eq!(l.num, 2021);
        } else {
            panic!("should be literal type");
        }
    }

    #[test]
    fn test_op1() {
        let h = to_hex("38006F45291200");
        let (p, _) = Packet::from_bytes(&h, 0);
        println!("{:?}", p);
        if let Packet::Operator(p) = p {
            assert_eq!(p.version, 1);
        } else {
            panic!("should be operator type");
        }
    }

    #[test]
    fn test_op2() {
        let h = to_hex("EE00D40C823060");
        let (p, _) = Packet::from_bytes(&h, 0);
        println!("{:?}", p);
        if let Packet::Operator(p) = p {
            assert_eq!(p.version, 7);
        } else {
            panic!("should be operator type");
        }
    }

    #[test]
    fn test_sum1() {
        let h = to_hex("8A004A801A8002F478");
        let (p, _) = Packet::from_bytes(&h, 0);
        assert_eq!(sum_versions(&p), 16);
    }

    #[test]
    fn test_sum2() {
        let h = to_hex("620080001611562C8802118E34");
        let (p, _) = Packet::from_bytes(&h, 0);
        assert_eq!(sum_versions(&p), 12);
    }

    #[test]
    fn test_sum3() {
        let h = to_hex("C0015000016115A2E0802F182340");
        let (p, _) = Packet::from_bytes(&h, 0);
        assert_eq!(sum_versions(&p), 23);
    }

    #[test]
    fn test_sum4() {
        let h = to_hex("A0016C880162017C3686B18A3D4780");
        let (p, _) = Packet::from_bytes(&h, 0);
        assert_eq!(sum_versions(&p), 31);
    }
}
