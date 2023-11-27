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

//! Advent of Code 2015 Day 10: <https://adventofcode.com/2015/day/10>
//!
//! TODO

/// The solution for the day nine challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y15d10::y15d10;
/// // probably read this from the input file...
/// let input = "1";
/// assert_eq!(y15d10(input, 10), 26);
/// ```
pub fn y15d10(input: &str, times: u32) -> u32 {
    let mut nums: Vec<u32> = input
        .trim()
        .chars()
        .map(|n| n.to_digit(10).unwrap())
        .collect();

    for _ in 0..times {
        nums = lookandsay(nums);
    }

    nums.len().try_into().unwrap()
}

/// TODO
fn lookandsay(input: Vec<u32>) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();

    let mut previous = input[0];
    let mut count = 1;

    for i in 1..input.len() {
        let digit = input[i];

        if digit != previous {
            result.push(count);
            result.push(previous);
            previous = digit;
            count = 1;
        } else {
            count += 1;
        }
    }

    result.push(count);
    result.push(previous);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_lookandsay() {
        assert_eq!(lookandsay(vec!(1)), vec!(1, 1));
        assert_eq!(lookandsay(vec!(1, 1)), vec!(2, 1));
        assert_eq!(lookandsay(vec!(2, 1)), vec!(1, 2, 1, 1));
        assert_eq!(lookandsay(vec!(1, 2, 1, 1)), vec!(1, 1, 1, 2, 2, 1));
        assert_eq!(lookandsay(vec!(1, 1, 1, 2, 2, 1)), vec!(3, 1, 2, 2, 1, 1));
    }

    #[test]
    fn tit_works() {
        let input = "1\n";

        assert_eq!(y15d10(input, 1), 2); // 11
        assert_eq!(y15d10(input, 2), 2); // 21
        assert_eq!(y15d10(input, 3), 4); // 1211
        assert_eq!(y15d10(input, 4), 6); // 111221
        assert_eq!(y15d10(input, 5), 6); // 312211
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day10.txt").unwrap();

        assert_eq!(y15d10(&contents, 40), 360154);
    }
}
