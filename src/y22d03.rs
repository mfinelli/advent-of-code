/* Copyright 2022 Mario Finelli
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Advent of Code 2022 Day 3: <https://adventofcode.com/2022/day/3>
//!
//! The day three challenge basically amounts to finding characters in
//! strings.
//!
//! For the first part we need to find a single character that appears in both
//! the first half and the second half of each string. I approached this by
//! looping through each line and in the first half of the string adding each
//! character to a [`std::collections::HashSet`] as it's seen. Then in the
//! second half of the string we check each character as we pass to see if it
//! exists in the Set -- once we find a match we're done for that line.
//!
//! I didn't try, but it _may_ be faster or more efficient to just split the
//! string in two and then loop through the second half only and just run a
//! `contains` check on the entire first half string.
//!
//! This is similar to the approach that I took for the second part of the
//! challenge. We loop through each group, but we actually only loop through
//! the characters in the third string of the group and check if both of the
//! first two strings contain the character in which case we're done.
//!
//! Finally, for determining the priority we just define a string with the
//! priorities in order so that we can do a simple lookup for a given
//! character (plus one because the string is zero-indexed).

use std::collections::HashSet;

const PRIORITY: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// The solution for the day three challenge. In reality calls a second
/// function based on the second argument (part `1` or part `2`).
///
/// For part one given the input as a string we loop through the characters in
/// each line. If we're in the first half we add them to the set, if we're in
/// the second half we check if the character is in the set (meaning that
/// we've found the duplicate) and return the priority. Then break because
/// it's possible that the duplicate character can exist more than once which
/// results in a total that is too large.
///
/// For the second part given the input as a string we calculate the number of
/// elf groups (lines divided by three because the elves are in groups of
/// three). Then for each group we loop through the characters in the third
/// group and check if it exists in both the first and second group (this
/// could also be loop through the first and check the second and third or
/// loop through the second and check the first and third with the same
/// outcome) and if so add the priority to the total.
///
/// # Example
/// ```rust
/// # use aoc::y22d03::y22d03;
/// // probably read this from the input file...
/// let input = "jaabbcck\njddeeffl\njgghhiim\n";
/// assert_eq!(y22d03(input, 1), 15);
/// assert_eq!(y22d03(input, 2), 10);
/// ```
pub fn y22d03(input: &str, part: u32) -> u32 {
    if part == 1 {
        y22d03p1(input)
    } else {
        y22d03p2(input)
    }
}

fn y22d03p1(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut sum = 0;

    for line in lines {
        let mut set = HashSet::new();
        let half = line.len() / 2;
        let mut i = 1;
        for ch in line.chars() {
            if i <= half {
                set.insert(ch);
            } else if set.contains(&ch) {
                sum += PRIORITY.find(ch).unwrap() as u32 + 1;
                break;
            }
            i += 1;
        }
    }

    sum
}

fn y22d03p2(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let groups = lines.len() / 3;

    let mut sum = 0;

    for gi in 0..groups {
        for c in lines[gi * 3 + 2].chars() {
            if lines[gi * 3].contains(c) && lines[gi * 3 + 1].contains(c) {
                sum += PRIORITY.find(c).unwrap() as u32 + 1;
                break;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let mut input = "cDeFeg";
        assert_eq!(y22d03p1(input), 5);

        input = "ABBCaaDEEF\nHabcHdef\n";
        assert_eq!(y22d03p1(input), 35);

        input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        assert_eq!(y22d03p1(input), 16);

        input = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        assert_eq!(y22d03p1(input), 38);

        input = "PmmdzqPrVvPwwTWBwg";
        assert_eq!(y22d03p1(input), 42);

        input = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn";
        assert_eq!(y22d03p1(input), 22);

        input = "ttgJtRGJQctTZtZT";
        assert_eq!(y22d03p1(input), 20);

        input = "CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(y22d03p1(input), 19);

        input = concat!(
            "vJrwpWtwJgWrhcsFMMfFFhFp\n",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n",
            "PmmdzqPrVvPwwTWBwg\n"
        );
        assert_eq!(y22d03p2(input), 18);

        input = concat!(
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n",
            "ttgJtRGJQctTZtZT\n",
            "CrZsJsPPZsGzwwsLwLmpwMDw"
        );
        assert_eq!(y22d03p2(input), 52);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2022/day03.txt").unwrap();

        assert_eq!(y22d03(&contents, 1), 7831);
        assert_eq!(y22d03(&contents, 2), 2683);
    }
}
