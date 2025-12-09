/* Copyright 2025 Mario Finelli
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

//! Advent of Code 2025 Day 6: <https://adventofcode.com/2025/day/6>
//!
//! TODO

/// The solution for the day six challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y25d06::y25d06;
/// // probably read this from the input file...
/// let input = concat!(
/// );
/// assert_eq!(y25d06(input), 1);
/// ```
pub fn y25d06(input: &str) -> u64 {
    let mut sum = 0;

    let mut problems: Vec<Vec<u64>> = Vec::new();
    let lines: Vec<_> = input.lines().collect();
    let op_index = lines.len()-1;

    for (i, line) in lines.iter().enumerate() {
        for (j, num) in line.split_whitespace().enumerate() {
            if i != op_index {
                if i == 0 {
                    let nums: Vec<u64> = Vec::new();
                    problems.push(nums);
                }

                let n = num.parse().unwrap();
                problems[j].push(n);
            } else {
                if num == "+" {
                    sum += problems[j].iter().sum::<u64>();
                } else {
                    sum += problems[j].iter().product::<u64>();
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
        let input = concat!(
            "123 328  51 64\n",
            "45 64  387 23\n",
            "6 98  215 314\n",
            "*   +   *   +\n",
        );

        assert_eq!(y25d06(input), 4277556);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2025/day06.txt").unwrap();

        assert_eq!(y25d06(&contents), 4580995422905);
    }
}
