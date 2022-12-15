// move crates one at a time
fn move_crates_fifo(stacks: &mut Vec<Vec<char>>, mov: &CrateMove) {
    // Using the values from the tuple, pop values off the "from" stack and push to the "to" stack
    for _ in 0..mov.count {
        let c = stacks[mov.from].pop().unwrap();
        stacks[mov.to].push(c);
    }
}

// move crates multiple at a time
fn move_crates_lifo(stacks: &mut Vec<Vec<char>>, mov: &CrateMove) {
    // Use a temporary stack to push and pop from to maintain the order when placing on the "to" stack
    let mut tmp: Vec<char> = Vec::new();
    // let last_n = stacks[mov.from].
    for _ in 0..mov.count {
        let c = stacks[mov.from].pop().unwrap();
        tmp.push(c);
    }
    for _ in 0..mov.count {
        stacks[mov.to].push(tmp.pop().unwrap());
    }
}

struct CrateMove {
    count: usize,
    from: usize,
    to: usize,
}

pub fn solve() {
    let input = include_str!("input/5.txt");

    // split input at the empty line
    let mut split = input.split("\r\n\r\n");

    // parse crate stack input into vec of strings
    let crates_input = split.next().unwrap().lines().collect::<Vec<&str>>();

    // parse crate move input into vec of CrateMove
    // Moves are in the format 'move 6 from 5 to 7', so we can split at spaces, skip the first word
    // and then grab every second value from there, parsing to integers and collecting into a CrateMove
    let move_list: Vec<CrateMove> = split.next().unwrap()
        .lines()
        .map(|line| {
            let nums = line.split_ascii_whitespace()
                .skip(1)
                .step_by(2)
                .map(|s| s.parse().unwrap())
                .collect::<Vec<usize>>();
            CrateMove {count: nums[0], from: nums[1] - 1, to: nums[2] - 1}
        }).collect();     

    // use bottom row of crates to count num of stacks
    let num_stacks = crates_input.last()
        .unwrap()
        .split_ascii_whitespace()
        .count();

    // represent each stack as a vec of chars, bottom crate first
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..num_stacks {
        stacks.push(Vec::new());
    }

    // parse crate input strings into stack struct, starting from bottom
    crates_input.iter()
        .rev()
        .for_each(|line| {
            // get chars in chunks of 4 (each crate is 3 chars + a space)
            // map each chunk to a single char, and filter out empty crates
            line.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, c)| *c != ' ')
                .for_each(|(i, c)| stacks[i].push(c))
        });

    // iterate over two callbacks to process the input for each part of the challenge
    let results = [move_crates_fifo, move_crates_lifo].iter()
        .map(|move_crates_fn| {
            let mut stacks2 = stacks.clone();

            move_list.iter()
                .for_each(|mov| {
                    move_crates_fn(&mut stacks2, mov);
                });
    
            stacks2.iter()
                .map(|stack| *stack.last().unwrap())
                .collect()
        })
        .collect::<Vec<String>>();

    println!("Day  5: {: >10} {: >10}", results[0], results[1]);
}