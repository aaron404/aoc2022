struct IntRange {
    min: i32,
    max: i32,
}

fn range_contains(range1: &IntRange, range2: &IntRange) -> bool {
    range2.min >= range1.min && range2.max <= range1.max
}

fn range_overlaps(range1: &IntRange, range2: &IntRange) -> bool {
    (range2.min >= range1.min && range2.min <= range1.max) ||
    (range2.max >= range1.min && range2.max <= range1.max)
}

// overlapping range detection
pub fn solve() {
    let input = include_str!("input/4.txt");

    let count1 = input
        .lines()
        .filter(|line| {
            let assignments = line.split(",").collect::<Vec<&str>>();
            let a = assignments[0].split("-").map(|c| c.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            let b = assignments[1].split("-").map(|c| c.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            let r1 = IntRange {min: a[0], max: a[1]};
            let r2 = IntRange {min: b[0], max: b[1]};
            range_contains(&r1, &r2) || range_contains(&r2, &r1)
        })
        .count();

    let count2 = input
    .lines()
    .filter(|line| {
        let assignments = line.split(",").collect::<Vec<&str>>();
        let a = assignments[0].split("-").map(|c| c.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let b = assignments[1].split("-").map(|c| c.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let r1 = IntRange {min: a[0], max: a[1]};
        let r2 = IntRange {min: b[0], max: b[1]};
        range_overlaps(&r1, &r2) || range_overlaps(&r2, &r1)
    })
    .count();

    println!("Day  4: {: >10} {: >10}", count1, count2);
}