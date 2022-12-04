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

//! Advent of Code 2022 Day 4: <https://adventofcode.com/2022/day/4>
//!
//! I would describe my solutions to the day four challenges as the brute
//! force solutions, there are probably much better, cleverer, and more
//! efficient solutions.
//!
//! In both parts we start by converting the string representations into their
//! numeric counterparts so that we can operate on them as numbers and/or
//! ranges.
//!
//! The first part is relatively simple, we just do a comparison if the
//! first range is contained in the second or vice versa. In the second part
//! We loop through all of the numbers in the first range and check if any of
//! the numbers are contained in the second range.

/// The solution for the day four challenge.
///
/// Given the input as a string and an integer depending if we want to solve
/// part `1` or part `2` of the challenge, we start by doing the usual split
/// of lines and initialization of the return sum. Then we loop through the
/// lines and convert them into their numerical couterparts as explained
/// above.
///
/// In the first part it's just a simple check if the first range start is
/// greater than (or equal to) the second range start _and_ the first range
/// end is less than (or equal to) the second range end (or vice versa).
///
/// The second part we convert the first range into an actual range (plus one
/// on the end because ranges are not inclusive) and then loop through the
/// numbers in the second range to see if one of them is contained in the
/// first range (then break in case more numbers are contained which gives us
/// too large of a number).
///
/// # Example
/// ```rust
/// # use aoc::y22d04::y22d04;
/// // probably read this from the input file...
/// let input = "2-3,3-4\n1-1,1-4\n";
/// assert_eq!(y22d04(input, 1), 1);
/// assert_eq!(y22d04(input, 2), 2);
/// ```
pub fn y22d04(input: &str, part: u32) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut sum = 0;

    for line in lines {
        let pairs: Vec<&str> = line.split(',').collect();
        let first_range: Vec<u32> =
            pairs[0].split('-').map(|r| r.parse().unwrap()).collect();
        let second_range: Vec<u32> =
            pairs[1].split('-').map(|r| r.parse().unwrap()).collect();

        if part == 1 {
            if (first_range[0] >= second_range[0]
                && first_range[1] <= second_range[1])
                || (second_range[0] >= first_range[0]
                    && second_range[1] <= first_range[1])
            {
                sum += 1;
            }
        } else {
            for i in first_range[0]..(first_range[1] + 1) {
                if (second_range[0]..(second_range[1] + 1)).contains(&i) {
                    sum += 1;
                    break;
                }
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
        let mut input = "2-4,6-8\n2-3,4-5\n5-7,7-9";
        assert_eq!(y22d04(input, 1), 0);
        assert_eq!(y22d04(input, 2), 1);

        input = "2-8,3-7\n6-6,4-6\n2-6,4-8\n";
        assert_eq!(y22d04(input, 1), 2);
        assert_eq!(y22d04(input, 2), 3);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("src/input/day4.txt").unwrap();

        assert_eq!(y22d04(&contents, 1), 448);
        assert_eq!(y22d04(&contents, 2), 794);
    }
}
