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

//! Advent of Code 2023 Day 1: <https://adventofcode.com/2023/day/1>
//!
//! TODO

/// The solution for the day one challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y23d01::y23d01;
/// // probably read this from the input file...
/// let input = "";
/// assert_eq!(y23d01(input), 0);
/// ```
pub fn y23d01(input: &str) -> u32 {
    let mut total = 0;
    let numbers = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    for line in input.lines() {
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;

        for c in line.chars() {
            if numbers.contains(&c) {
                first = Some(c);
                break;
            }
        }

        for c in line.chars().rev() {
            if numbers.contains(&c) {
                last = Some(c);
                break;
            }
        }

        let number = format!("{}{}", first.unwrap(), last.unwrap());
        total += number.parse::<u32>().unwrap();
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n";

        assert_eq!(y23d01(input), 142);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day01.txt").unwrap();

        assert_eq!(y23d01(&contents), 0);
    }
}
