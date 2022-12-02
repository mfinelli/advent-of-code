//! Advent of Code 2022 Day 2: <https://adventofcode.com/2022/day/2>
//!
//! Since the total number of combinations is so small (9 for every possible
//! outcome) I have solved this by doing a static lookup for any given input.
//!
//! The difference between part one and two only change the point value
//! assigned to each outcome (depending on whether the second letter i.e.,
//! your move means what to _play_ or what to _do_).

use std::collections::HashMap;

/// The solution for the day two challenge.
///
/// Given the input as a string, it splits by lines and then creates a running
/// tally as it loops through them as each possible outcome is stored in a
/// HashMap with its value.
///
/// The second argument simply decides whether we should use the what to _play_
/// method (part `1`) or the what to _do_ method (part `2`).
///
/// # Example
/// ```rust
/// # use aoc::daytwo::daytwo;
/// let input = "A X\nB X"; // probably read this from the input file...
/// assert_eq!(daytwo(input, 1), 5);
/// assert_eq!(daytwo(input, 2), 4);
/// ```
pub fn daytwo(input: &str, part: u32) -> u32 {
    let matchups: HashMap<&str, u32> = if part == 1 {
        HashMap::from([
            ("A X", 4), // rock v. rock (1) / draw (3)
            ("A Y", 8), // rock v. paper (2) / win (6)
            ("A Z", 3), // rock v. scissors (3) / lose (0)
            ("B X", 1), // paper v. rock (1) / lose (0)
            ("B Y", 5), // paper v. paper (2) / draw (3)
            ("B Z", 9), // paper v. scissors (3) / win (6)
            ("C X", 7), // scissors v. rock (1) / win (6)
            ("C Y", 2), // scissors v. paper (2) / lose (0)
            ("C Z", 6), // scissors v. scissors (3) / draw (3)
        ])
    } else {
        HashMap::from([
            ("A X", 3), // rock v. lose (scissors): 0 + 3 = 3
            ("A Y", 4), // rock v. draw (rock): 3 + 1 = 4
            ("A Z", 8), // rock v. win (paper): 6 + 2 = 8
            ("B X", 1), // paper v. lose (rock): 0 + 1 = 1
            ("B Y", 5), // paper v. draw (paper): 3 + 2 = 5
            ("B Z", 9), // paper v. win (scissors): 6 + 3 = 9
            ("C X", 2), // scissors v. lose (paper): 0 + 2 = 2
            ("C Y", 6), // scissors v. draw (scissors): 3 + 3 = 6
            ("C Z", 7), // scissors v. win (rock): 6 + 1 = 7
        ])
    };

    let lines: Vec<_> = input.lines().collect();
    let mut total = 0;

    for line in lines {
        total += matchups.get(line).unwrap();
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let mut input = "A X\nA Y\n";
        assert_eq!(daytwo(input, 1), 12);
        assert_eq!(daytwo(input, 2), 7);

        input = "A Z\nB X\nC Y";
        assert_eq!(daytwo(input, 1), 6);
        assert_eq!(daytwo(input, 2), 15);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("src/input/day2.txt").unwrap();

        assert_eq!(daytwo(&contents, 1), 15691);
        assert_eq!(daytwo(&contents, 2), 12989);
    }
}
