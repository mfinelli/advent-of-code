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

//! Advent of Code 2025 Day 5: <https://adventofcode.com/2025/day/5>
//!
//! TODO

/// The solution for the day five challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y25d05::y25d05;
/// // probably read this from the input file...
/// let input = concat!(
/// );
/// assert_eq!(y25d05(input), 1);
/// ```
pub fn y25d05(input: &str) -> u64 {
    let mut sum = 0;
    let parts: Vec<_> = input.split("\n\n").collect();
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for range in parts[0].lines().into_iter() {
        let range_parts: Vec<_> = range.split('-').collect();
        let lower: u64 = range_parts[0].parse().unwrap();
        let upper: u64 = range_parts[1].parse().unwrap();
        ranges.push((lower, upper));
    }

    for item in parts[1].lines().into_iter() {
        let value: u64 = item.parse().unwrap();

        for (lower, upper) in &ranges {
            if value >= *lower && value <= *upper {
                sum += 1;
                break;
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
            "3-5\n",
            "10-14\n",
            "16-20\n",
            "12-18\n",
            "\n",
            "1\n",
            "5\n",
            "8\n",
            "11\n",
            "17\n",
            "32\n",
        );

        assert_eq!(y25d05(input), 3);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2025/day05.txt").unwrap();

        assert_eq!(y25d05(&contents), 598);
    }
}
