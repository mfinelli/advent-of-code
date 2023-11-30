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

//! Advent of Code 2015 Day 17: <https://adventofcode.com/2015/day/17>
//!
//! TODO

use itertools::Itertools;

/// The solution for the day seventeen challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y15d17::y15d17;
/// // probably read this from the input file...
/// let input = "";
/// assert_eq!(y15d17(input, 10), 0);
/// ```
pub fn y15d17(input: &str, liters: u32) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut bottles: Vec<u32> = Vec::new();
    let mut total = 0;

    for line in lines {
        bottles.push(line.parse().unwrap());
    }

    for i in 1..bottles.len() {
        for combination in bottles.iter().combinations(i) {
            if combination.into_iter().sum::<u32>() == liters {
                total += 1;
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = "20\n15\n10\n5\n5\n";
        assert_eq!(y15d17(input, 25), 4);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day17.txt").unwrap();

        assert_eq!(y15d17(&contents, 150), 1638);
    }
}
