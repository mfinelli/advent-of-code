use std::fs;

use aoc::dayone;

fn main() {
    let contents = fs::read_to_string("src/input/day1.txt").expect("Unable to open input file.");

    let r = dayone::dayone(&contents, 1);
    println!("{}", r);

    let q = dayone::dayone(&contents, 3);
    println!("{}", q);
}
