
pub fn solve() {
    // split input at empty lines
    let input = include_str!("input/1.txt").split("\r\n\r\n");

    // split list of calories into lines, map strings to integers, sum each list, and collect into a vector
    let mut sums = input.map(|cal_list| 
        cal_list.lines()
            .map(|cal_count| cal_count.parse::<u32>().expect("invalid input"))
            .sum::<u32>()
        ).collect::<Vec<u32>>();

    // reverse sort the vector (highest first)
    sums.sort_unstable_by(|a, b| b.cmp(a));

    // extract highest and sum of top 3
    let highest = sums.first().expect("invalid input");
    let sum_of_top3 = sums.iter().take(3).sum::<u32>();

    println!("Day  1: {: >10} {: >10}", highest, sum_of_top3);
}
