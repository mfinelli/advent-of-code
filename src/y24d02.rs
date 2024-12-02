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

//! Advent of Code 2024 Day 2: <https://adventofcode.com/2024/day/2>
//!
//! TODO

/// The solution for the day two challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y24d02::y24d02;
/// // probably read this from the input file...
/// let input = concat!(
/// );
/// assert_eq!(y24d02(input), 0);
/// ```
pub fn y24d02(input: &str) -> u32 {
    let mut safe = 0;

    let lines: Vec<_> = input.lines().collect();
    for line in lines {
        let parts: Vec<u32> = line.split_whitespace().map(|p| p.parse().unwrap()).collect();

        if is_safe(&parts) {
            safe += 1;
        }
    }

    safe
}

/// TODO
fn is_safe(nums: &Vec<u32>) -> bool {
    let mut is_increasing = true;
    if nums[0] > nums[1] {
        is_increasing = false;
    }

    let mut windows = nums.windows(2);
    while let Some(pair) = windows.next() {
        if is_increasing && pair[0] > pair[1] {
            return false;
        } else if !is_increasing && pair[1] > pair[0] {
            return false;
        }

        if is_increasing {
            let diff = pair[1] - pair[0];
            if diff < 1 || diff > 3 {
                return false;
            }
        } else {
            let diff = pair[0] - pair[1];
            if diff < 1 || diff > 3 {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_is_safe() {
    }

    #[test]
    fn it_works() {
        let input = concat!(
            "7 6 4 2 1\n",
            "1 2 7 8 9\n",
            "9 7 6 2 1\n",
            "1 3 2 4 5\n",
            "8 6 4 4 1\n",
            "1 3 6 7 9\n",
        );

        assert_eq!(y24d02(input), 2);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2024/day02.txt").unwrap();

        assert_eq!(y24d02(&contents), 390);
    }
}
