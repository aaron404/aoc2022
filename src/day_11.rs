use std::collections::BinaryHeap;

#[derive(Clone)]
enum Operation {
    Square,
    Mult(usize),
    Add(usize),
}

#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    div: usize,
    next_true: usize,
    next_false: usize,
    count: usize,
}

fn parse_input(input: &str) -> Vec<Monkey> {
    // parse monkeys - first split at empty line to process each monkey separately
    input.split("\r\n\r\n")
        .map(|monkey_desc| {
            // clear all the clutter from input, leaving only numbers and whitespace
            let simplified_desc = monkey_desc.chars()
                .filter(|c| {
                    match c {
                        '0'..='9' => true,
                        ' ' => true,
                        '\n' => true,
                        '*' => true,
                        '+' => true,
                        _ => false,
                    }
                }).collect::<String>();
            let mut lines = simplified_desc.lines();

            // parse each line of the monkey's description, store in Monkey struct
            let [_, items, op_str, div, next_true, next_false] = lines.next_chunk::<6>().unwrap();
            let items = items
                .split_ascii_whitespace()
                .map(|num| num.parse::<usize>().expect("failed to parse starting items"))
                .collect::<Vec<usize>>();
            
            let op_str = op_str.trim();
            let operation = if op_str == "*" {
                Operation::Square
            } else if op_str.starts_with("*") {
                Operation::Mult(op_str.split(" ").last().expect("missing operand").parse::<usize>().unwrap())
            } else {
                Operation::Add(op_str.split(" ").last().expect("missing operand").parse::<usize>().unwrap())
            };

            let div = div.trim().parse::<usize>().expect("failed to parse test");

            let next_true = next_true.trim().parse::<usize>().expect("failed to parse 'if true'");
            let next_false = next_false.trim().parse::<usize>().expect("failed to parse 'if false'");

            Monkey { items, operation, div, next_true, next_false, count: 0 }
        }).collect::<Vec<Monkey>>()
}

pub fn solve() {

    // number of rounds and the worry relief factor for each part of the challenge
    let variations: [(usize, usize); 2] = [(20, 3), (10000, 1)];
    
    let results = variations.iter().map(|(num_rounds, relief_factor)| {
        let mut monkeys = parse_input(include_str!("input/11.txt"));

        // Compute a common multiple of each of the monkeys' divisors. It can be used
        // to modulo the worry values which grow quickly during part two
        let base: usize = monkeys.iter().map(|monkey| monkey.div).product();

        // Perform rounds of throws
        for _ in 0..*num_rounds {
            for i in 0..monkeys.len() {
                let monkey = monkeys[i].clone();
                for item in monkey.items.iter() {
                    let new_val = match monkey.operation {
                        Operation::Square => (item * item) % base / relief_factor,
                        Operation::Mult(val) => (item * val) % base / relief_factor,
                        Operation::Add(val) => (item + val) % base / relief_factor,
                    };
                    let new_monkey = if new_val % monkey.div == 0 {
                        monkey.next_true
                    } else {
                        monkey.next_false
                    };
                    monkeys[new_monkey].items.push(new_val);
                }
                monkeys[i].count += monkeys[i].items.len();
                monkeys[i].items.clear();
            }
        }
        
        BinaryHeap::from_iter(monkeys.iter().map(|monkey| monkey.count))
            .into_iter_sorted()
            .next_chunk::<2>().expect("less than two monkeys found")
            .iter()
            .product::<usize>()
    }).collect::<Vec<usize>>();

    println!("Day 11: {: >10} {: >10}", results[0], results[1]);
}