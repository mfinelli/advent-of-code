use std::collections::BinaryHeap;
use std::fs;

fn main() {
    let r = day1(1);
    let q = day1(3);
    println!("{}", r);
    println!("{}", q);
}

// The request for this challenge is to simply know the total amount of
// calories of the elf holding the most calories.
// We'll do this using a max heap with the amount of calories for each elf.
// We'll loop through the lines in the input summing each elf's calories and
// then when we hit a newline we'll "commit" this total to the heap. We then
// do one more commit at the end since the file doesn't have an extra trailing
// newline.
fn day1(top: u32) -> u32 {
    let contents = fs::read_to_string("src/input/day1.txt")
        .expect("Unable to open input file.");

    let lines: Vec<_> = contents.lines().collect();
    let mut current = 0;
    let mut heap = BinaryHeap::new();

    for line in lines {
        if line == "" {
            // when we hit a blank line "commit" the result to the heap
            heap.push(current);
            current = 0;
            continue;
        }

        let i: u32 = line.parse().expect("Expected an integer.");
        current += i;
    }

    // this makes it work if there's a blank newline at the end or not
    if current != 0 {
        heap.push(current);
    }

    let mut sum = 0;

    for i in 0..top {
        sum += heap.pop().unwrap();
    }

    sum

    // *heap.peek().unwrap()
}
