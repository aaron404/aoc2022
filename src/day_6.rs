// create a bitmask of ascii codes, then count the ones
fn msg_is_unique(msg: &[u8]) -> bool {
    let mask = msg.iter().fold(0u128, |acc, &x| acc | (1 << x));
    mask.count_ones() as usize == msg.len()
}

fn get_first_unique_n(data: &[u8], n: usize) -> usize {
    data.windows(n)
        .enumerate()
        .filter(|window| msg_is_unique(window.1))
        .nth(0)
        .expect("Error parsing input").0 + n
}

pub fn solve() {
    const START_OF_PACKET_SIZE: usize = 4;
    const START_OF_MSG_SIZE: usize = 14;

    let input = include_str!("input/6.txt");

    let data = input.as_bytes();

    let first_packet_offset = get_first_unique_n(data, START_OF_PACKET_SIZE);
    let first_msg_offset = get_first_unique_n(data, START_OF_MSG_SIZE);

    println!("Day  6: {: >10} {: >10}", first_packet_offset, first_msg_offset);
}