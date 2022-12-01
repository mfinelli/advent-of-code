//! Advent of Code 2022 Day 1: <https://adventofcode.com/2022/day/1>
//!
//! This challenge essentially boils down to getting the `n` largest items,
//! where in part one `n` is one.
//!
//! As described on the challenge page each elf is carrying one or more (food)
//! items with a (calorie) value. Each item is on one line and each elf is
//! separated by a (single) empty line.
//!
//! The solution as I have envisioned it is to use a [max
//! heap](https://en.wikipedia.org/wiki/Min-max_heap) to keep track of the
//! items and then return the requested number of items in largest-first order.

use std::collections::BinaryHeap;

/// The solution for the day one challenge.
///
/// Given the input as a string, it splits by lines and then in a single pass
/// loops through the items. As it sees each value it adds it to the current
/// total for that elf and when it encounters an empty line it then marks that
/// elf as finished by adding their total value to the heap.
///
/// The second argument corresponds to how many elves' totals should be
/// included when returning the total output. This means that part one can be
/// solved by providing `1` and part two can be solved by providing `3`.
///
/// # Example
/// ```rust
/// # use aoc::dayone::dayone;
/// let input = "1\n2\n\n3\n"; // probably read this from the input file...
/// assert_eq!(dayone(input, 1), 3);
/// assert_eq!(dayone(input, 2), 6);
/// ```
pub fn dayone(input: &str, top: u32) -> u32 {
    let lines: Vec<_> = input.lines().collect();

    let mut current = 0;
    let mut heap = BinaryHeap::new();

    for line in lines {
        if line == "" {
            // when we get a blank line "commit" the result to the heap
            heap.push(current);
            current = 0;
            continue;
        }

        let i: u32 = line.parse().expect("Expected an integer.");
        current += i;
    }

    // this makes it work if there's a blank newline at the end or not:
    // if there is a final newline then we already committed the final result
    // in the loop above, if there isn't (and therefore current is non-zero)
    // then we need to commit the final result now
    if current != 0 {
        heap.push(current);
    }

    let mut sum = 0;
    for _i in 0..top {
        sum += heap.pop().unwrap();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let mut input = "1\n2\n\n3\n4\n\n5";
        assert_eq!(dayone(input, 1), 7);

        input = "1\n2\n\n3\n4\n\n5\n";
        assert_eq!(dayone(input, 1), 7);
        assert_eq!(dayone(input, 2), 12);

        input = "1\n2\n\n3";
        assert_eq!(dayone(input, 1), 3);
        assert_eq!(dayone(input, 2), 6);

        input = "1\n2\n\n3\n\n4";
        assert_eq!(dayone(input, 1), 4);
        input = "1\n2\n\n3\n\n4\n";
        assert_eq!(dayone(input, 1), 4);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("src/input/day1.txt").unwrap();

        assert_eq!(dayone(&contents, 1), 69528);
        assert_eq!(dayone(&contents, 3), 206152);
    }
}
