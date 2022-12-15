fn noop(cycle: &mut i32) {
    *cycle += 1;
}

fn add_1(cycle: &mut i32) {
    *cycle += 1;
}

fn add_2(x: &mut i32, cycle: &mut i32, val: i32) {
    *cycle += 1;
    *x += val;
}

fn get_signal_strength(x: i32, cycle: i32) -> i32 {
    if (cycle - 20) % 40 == 0 {
        return x * cycle;
    }
    0
}

fn draw_pixel(x: i32, cycle: i32) {
    if (x - (cycle - 1) % 40).abs() < 2 {
        print!("#");
    } else {
        print!(" ");
    }
    if (cycle) % 40 == 0 {
        println!("");
    }
}

pub fn solve() {
    let input = include_str!("input/10.txt");

    let mut x: i32 = 1;
    let mut cycle: i32 = 0;
    let mut sig_strength: i32 = 0;

    for line in input.lines() {
        if line.starts_with("noop") {
            noop( &mut cycle);
            sig_strength += get_signal_strength(x, cycle);
            draw_pixel(x, cycle);
        } else if line.starts_with("add") {
            let mut split = line.split(" ");
            let _ = split.next().unwrap();
            let val = split.next().expect("missing value for add").parse::<i32>().expect("failed to parse value");
            noop(&mut cycle);//add_1(&mut x, &mut cycle, val);
            draw_pixel(x, cycle);
            sig_strength += get_signal_strength(x, cycle);
            cycle += 1;
            draw_pixel(x, cycle);
            sig_strength += get_signal_strength(x, cycle);
            x += val;
        } else {
            panic!("invalid input");
        }
    }

    println!("Day 10: {: >10} {: >10}", sig_strength, -1);
}