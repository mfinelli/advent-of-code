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
//! A very easy challenge today that amounts to using all of the combinations
//! of containers that we can use and seeing if we can use that combination to
//! transport the desired number of liters of eggnog.

use itertools::Itertools;

/// The solution for the day seventeen challenge.
///
/// We take the usual input as a string. The second argument is for the number
/// of liters that we want to transport. The third argument is for the part of
/// the problem that we want to solve (part `1` for the total number of
/// possible combinations and part `2` for the number of combinations using the
/// fewest number of containers). We start by parsing the input to create a
/// vector of integers representing the capacity of each container. Then we
/// find all of the possible combinations using from one to the total number
/// of containers that we have. For each combination we check to see if it adds
/// up to the correct number of liters. If it does then we increment our
/// counter. If we're on part two as soon as we have a non-zero number of
/// combinations we can return the answer because we take combinations from
/// one to number of containers and the prompt only asks how many combinations
/// are possible using the fewest number of containers.
///
/// # Example
/// ```rust
/// # use aoc::y15d17::y15d17;
/// // probably read this from the input file...
/// let input = "2\n3\n4\n5\n6\n7\n8\n9";
/// assert_eq!(y15d17(input, 10, 1), 4);
/// assert_eq!(y15d17(input, 10, 2), 3);
/// ```
pub fn y15d17(input: &str, liters: u32, part: u32) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut containers: Vec<u32> = Vec::new();
    let mut total = 0;

    for line in lines {
        containers.push(line.parse().unwrap());
    }

    for i in 1..containers.len() {
        let mut combinations = 0;

        for combination in containers.iter().combinations(i) {
            if combination.into_iter().sum::<u32>() == liters {
                combinations += 1;
            }
        }

        if part == 2 && combinations != 0 {
            return combinations;
        }

        total += combinations;
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
        assert_eq!(y15d17(input, 25, 1), 4);
        assert_eq!(y15d17(input, 25, 2), 3);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day17.txt").unwrap();

        assert_eq!(y15d17(&contents, 150, 1), 1638);
        assert_eq!(y15d17(&contents, 150, 2), 17);
    }
}
