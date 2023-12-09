/* Copyright 2023 Mario Finelli
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

//! Advent of Code 2023 Day 9: <https://adventofcode.com/2023/day/9>
//!
//! TODO

/// The solution for the day nine challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y23d09::y23d09;
/// // probably read this from the input file...
/// let input = "";
/// assert_eq!(y23d09(input, 1), 1);
/// assert_eq!(y23d09(input, 2), 1);
/// ```
pub fn y23d09(input: &str, part: u32) -> i32 {
    let mut first_numbers = Vec::new();
    let mut next_numbers = Vec::new();

    for line in input.lines() {
        let numbers: Vec<i32> = line.split_whitespace().map(|n| n.parse().unwrap()).collect();

        let first_number = numbers[0];
        let next_number = numbers[numbers.len() -1];
        let mut check = numbers;
        let mut all_diffs = Vec::new();
        loop {
            let mut diffs = Vec::new();

            for pair in check.windows(2) {
                diffs.push(pair[1]-pair[0]);
            }

            if diffs.iter().all(|d| *d == 0) {
                break;
            }

            all_diffs.push(diffs.clone());
            check = diffs;
        }

        let mut first_value = 0;
        let mut last_value = 0;
        for diff in all_diffs.iter().rev() {
            first_value = diff[0] - first_value;
            last_value = diff.last().unwrap() + last_value;
        }

        next_numbers.push(next_number + last_value);
        first_numbers.push(first_number - first_value);
    }

    if part == 1 {
        next_numbers.iter().sum()
    } else {
        first_numbers.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45\n";

        assert_eq!(y23d09(input, 1), 114);
        assert_eq!(y23d09(input, 2), 2);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day09.txt").unwrap();

        assert_eq!(y23d09(&contents, 1), 1901217887);
        assert_eq!(y23d09(&contents, 2), 905);
    }
}
