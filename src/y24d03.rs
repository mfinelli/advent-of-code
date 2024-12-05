/* Copyright 2024 Mario Finelli
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

//! Advent of Code 2024 Day 3: <https://adventofcode.com/2024/day/3>
//!
//! TODO

use regex::Regex;

/// The solution for the day three challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y24d03::y24d03;
/// // probably read this from the input file...
/// let input = concat!(
/// );
/// assert_eq!(y24d03(input, 1), 0);
/// ```
pub fn y24d03(input: &str, part: u32) -> u32 {
    let s = Regex::new(r"(do\(\))|(don't\(\))").unwrap();
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;

    let mut actions: Vec<bool> = Vec::new();
    actions.push(true); // start with instructions enabled

    if part == 2 {
        for (_, [action]) in s.captures_iter(input).map(|c| c.extract()) {
            if action == "do()" {
                actions.push(true);
            } else {
                actions.push(false);
            }
        }

        let mut i = 0;
        let blocks = s.split(input);
        for block in blocks {
            if !actions[i] {
                i += 1;
                continue;
            }

            for (_, [l, r]) in re.captures_iter(block).map(|c| c.extract()) {
                let left: u32 = l.parse().unwrap();
                let right: u32 = r.parse().unwrap();

                sum += left * right;
            }

            i += 1;
        }
    } else {
        for (_, [l, r]) in re.captures_iter(input).map(|c| c.extract()) {
            let left: u32 = l.parse().unwrap();
            let right: u32 = r.parse().unwrap();

            sum += left * right;
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
        let mut input = concat!(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)",
            "+mul(32,64]then(mul(11,8)mul(8,5))",
        );
        assert_eq!(y24d03(input, 1), 161);

        input = concat!(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+",
            "mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        assert_eq!(y24d03(input, 2), 48);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2024/day03.txt").unwrap();

        assert_eq!(y24d03(&contents, 1), 180233229);
        assert_eq!(y24d03(&contents, 2), 95411583);
    }
}
