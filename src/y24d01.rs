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

//! Advent of Code 2024 Day 1: <https://adventofcode.com/2024/day/1>
//!
//! TODO

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// The solution for the day one challenge.
/// 
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y24d01::y24d01;
/// // probably read this from the input file...
/// let input = concat!(
///     "1   2\n",
///     "3   4",
/// );
/// assert_eq!(y24d01(input), 2);
/// ```
pub fn y24d01(input: &str) -> u32 {
    let mut left = BinaryHeap::new();
    let mut right = BinaryHeap::new();
    let mut sum = 0;

    let lines: Vec<_> = input.lines().collect();
    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();

        let l: u32 = parts[0].parse().unwrap();
        let r: u32 = parts[1].parse().unwrap();

        left.push(Reverse(l));
        right.push(Reverse(r));
    }

    while let Some(Reverse(l)) = left.pop() {
        let Reverse(r) = right.pop().unwrap();
        if l > r {
            sum += l - r;
        } else {
            sum += r - l;
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
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n";
        assert_eq!(y24d01(input), 11)
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2024/day01.txt").unwrap();

        assert_eq!(y24d01(&contents), 2176849);
    }
}
