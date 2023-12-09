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
//! A very straightforward challenge today. Details about the approach and
//! solution in the description for the solve function.

/// The solution for the day nine challenge.
///
/// As usual we take the input as a string and an integer to denote the part.
/// In part `1` we need to find the next number in the sequence and in part
/// `2` we need to find the first number in the sequence. For each sequence
/// (line) we split the input and parse the numbers. Then, we find the diffs
/// between each of the numbers. If all of the diffs are zero then we're done
/// finding the diffs. Otherwise we add the new list of diffs to a vector of
/// diffs and then run the same process on that list continuing until we have
/// all zeros. Then we calculate the previous and next numbers in the sequence.
/// We reverse the list of diffs that we have and add the previous value (zero
/// to start) to the next item that we pop from the beginning or the end of
/// the diff that we're processing. When we're done we've found the next (or
/// previous) number in the sequence and can add it to the vector tracking all
/// of the sequences. Finally, based on the part we can return the sum of the
/// next numbers or the previous numbers.
///
/// # Example
/// ```rust
/// # use aoc::y23d09::y23d09;
/// // probably read this from the input file...
/// let input = "9 12 17 23 28 40";
/// assert_eq!(y23d09(input, 1), 88);
/// assert_eq!(y23d09(input, 2), -3);
/// ```
pub fn y23d09(input: &str, part: u32) -> i32 {
    let mut previous_numbers = Vec::new();
    let mut next_numbers = Vec::new();

    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let previous_number = numbers[0];
        let next_number = numbers[numbers.len() - 1];
        let mut check = numbers;
        let mut all_diffs = Vec::new();
        loop {
            let mut diffs = Vec::new();

            for pair in check.windows(2) {
                diffs.push(pair[1] - pair[0]);
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
            last_value += diff[diff.len() - 1];
        }

        next_numbers.push(next_number + last_value);
        previous_numbers.push(previous_number - first_value);
    }

    if part == 1 {
        next_numbers.iter().sum()
    } else {
        previous_numbers.iter().sum()
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
