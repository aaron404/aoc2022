use std::{cmp::Ordering, str::FromStr, string::ParseError};

enum Packet {
    List(Vec<Packet>),
    Integer(usize)
}

impl Packet {
    fn new() -> Self {
        Packet::List(Vec::new())
    }
    
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::List(l), Packet::List(r)) => {
                let len_cmp = l.len().cmp(&r.len());
                for i in 0..usize::min(l.len(), r.len()) {
                    let cmp = Packet::cmp(&l[i], &r[i]);
                    if cmp == Ordering::Equal {
                        continue;
                    }
                    return cmp;
                }
                return len_cmp;
            },
            (Packet::List(_), Packet::Integer(r)) => Packet::cmp(&Packet::List(vec![Packet::Integer(*r)]), other),
            (Packet::Integer(l), Packet::List(_)) => Packet::cmp(&Packet::List(vec![Packet::Integer(*l)]), other),
            (Packet::Integer(l), Packet::Integer(r)) => l.cmp(r),
        }
    }

    fn to_str(&self) -> String {
        match self {
            Packet::List(list) => {
                format!("[{}]", list.iter()
                    .map(|packet| packet.to_str())
                    .collect::<Vec<String>>()
                    .join(",")
                )
            },
            Packet::Integer(int) => int.to_string(),
        }
    }
}

impl FromStr for Packet {
    type Err = ParseError; 

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let packet = Packet::List(Vec::new());

        let mut stack: Vec<Packet> = vec![packet];
        // stack.push(&mut Packet::List(Vec::new()));

        let mut current_num = String::new();
        let current_list: &mut Vec<Packet> = &mut stack;

        // let bracket_lists = s.match_indices('[')
        //     .map(|(i, p)| i)
        //     .zip(s.match_indices(']').map(|(i, p)| i).rev());
        
        println!("str: {}", s);

        s.chars().skip(1).for_each(|c| {
            match c {
                '[' => todo!(),
                ']' => { current_list.pop(); },
                ',' => {
                    if current_num.len() > 0 {
                        if let Packet::List(list) = current_list.last_mut().unwrap() {
                            list.push(Packet::Integer(current_num.parse::<usize>().unwrap()));
                        }
                    } else {
                        todo!();
                    }
                },
                '0'..='9' => current_num.push(c),
                _ => panic!("invalid char"),
            }
        });

        // Ok(packet)
        todo!();
    }
}

enum ParseState {
    OpenBracket,
    ClosedBracket,
    Digit,
    Comma
}

impl ParseState {
    fn from_char(c: char) -> ParseState {
        match c {
            '[' => ParseState::OpenBracket,
            ']' => ParseState::ClosedBracket,
            ',' => ParseState::Comma,
            '0'..='9' => ParseState::Digit,
            _ => panic!("Invalid char"),
        }
    }
}

pub fn solve() {
    let input = include_str!("input/13_test.txt");

    let _result: usize = input.split("\r\n\r\n")
        .enumerate()
        .filter(|(_, pair_str)| {
            let pair = pair_str.lines() //.map(|str| str.to_string()).collect::<Vec<String>>();
            // let a = pair[0].chars();
            // let b = pair[1].chars();

            // let len = usize::min(pair[0].len(), pair[1].len());
            
            // println!("{}", pair[0]);
            // println!("{}", pair[1]);

            // let mut i1 = 0;
            // let mut i2 = 0;
            // let mut l_num = String::new();
            // let mut r_num = String::new();
            // let mut done = false;
            // while !done {
            //     let l = pair[0].as_bytes()[i1] as char;
            //     let r = pair[1].as_bytes()[i2] as char;
            //     println!("  {} {}", l, r);
            //     match (ParseState::from_char(l), ParseState::from_char(r)) {
            //         (ParseState::OpenBracket, ParseState::OpenBracket) => { i1 += 1; i2 += 1 },
            //         (ParseState::OpenBracket, ParseState::ClosedBracket) => todo!(),
            //         (ParseState::OpenBracket, ParseState::Digit) => todo!(),
            //         (ParseState::OpenBracket, ParseState::Comma) => todo!(),
            //         (ParseState::ClosedBracket, ParseState::OpenBracket) => todo!(),
            //         (ParseState::ClosedBracket, ParseState::ClosedBracket) => { i1 += 1; i2 += 1 },
            //         (ParseState::ClosedBracket, ParseState::Digit) => todo!(),
            //         (ParseState::ClosedBracket, ParseState::Comma) => todo!(),
            //         (ParseState::Digit, ParseState::OpenBracket) => todo!(),
            //         (ParseState::Digit, ParseState::ClosedBracket) => todo!(),
            //         (ParseState::Digit, ParseState::Digit) => {
            //             l_num.push(l);
            //             r_num.push(r);
            //             i1 += 1; i2 += 1;
            //         },
            //         (ParseState::Digit, ParseState::Comma) => todo!(),
            //         (ParseState::Comma, ParseState::OpenBracket) => todo!(),
            //         (ParseState::Comma, ParseState::ClosedBracket) => todo!(),
            //         (ParseState::Comma, ParseState::Digit) => todo!(),
            //         (ParseState::Comma, ParseState::Comma) => {
            //             if l_num.len() > 0 && r_num.len() > 0 {
            //                 if l_num.parse::<usize>().unwrap() < r_num.parse::<usize>().unwrap() {
            //                     return true;
            //                 }
            //             }
            //             i1 += 1; i2 += 1
            //         },
            //     }
            // }

                .map(|line| Packet::from_str(line).unwrap())
                .collect::<Vec<Packet>>();
            pair[0].cmp(&pair[1]) == Ordering::Less
        })
        .map(|(i, _str)| i)
        .sum::<usize>();

    println!("Day 13: {: >10} {: >10}", -1, -1);

    println!("{}",
        Packet::List(vec![
            Packet::Integer(1),
            Packet::Integer(2),
            Packet::List(vec![
                Packet::List(vec![
                    Packet::Integer(3)
                ]),
                Packet::Integer(4)]),
                Packet::Integer(5),
                Packet::List(vec![
                    Packet::List(vec![
                        Packet::Integer(6)
                    ]),
                    Packet::Integer(7)])
            ]).to_str());
}