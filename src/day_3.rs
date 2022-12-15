pub fn solve() {
    let input = include_str!("input/3.txt");

    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let result1: i32 = input.lines().map(|line| {
        let compartments = line.split_at(line.len() / 2);
        let m = letters.as_bytes().iter()
            .filter(|c| compartments.0.as_bytes().contains(c) && compartments.1.as_bytes().contains(c))
            .map(|i| {
                match i {
                    65..=90 => *i as i32 - 65 + 27,
                    97..=122 => *i as i32 - 96,
                    _ => 0 as i32,
                }
            })
            .sum::<i32>();
        m
    }).sum();

    let result2: i32 = input.lines().collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            for char in letters.as_bytes() {
                if chunk.iter().all(|&x| x.as_bytes().contains(char)) {
                    return *char
                }
            }
            0
        }).map(|i| match i {
            65..=90 => i as i32 - 65 + 27,
            97..=122 => i as i32 - 96,
            _ => 0 as i32,
        })
        .sum::<i32>();
    println!("Day  3: {: >10} {: >10}", result1, result2);
}