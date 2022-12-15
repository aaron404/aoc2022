#![feature(iter_array_chunks)]
#![feature(mixed_integer_ops)]
#![allow(dead_code)]
#![feature(iter_next_chunk)]

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;

fn main() {
    day_1::solve();
    day_2::solve();
    day_3::solve();
    day_4::solve();
    day_5::solve();
    day_6::solve();
    day_7::solve();
    day_8::solve();
    day_9::solve();
    day_10::day_10();
    day_11::solve();
    // day_12::solve();
    // day_13::solve();
    day_14::solve();
}
