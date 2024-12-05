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
//! The first part of today's challenge was fairly easy, the second part was
//! more challenging but ultimately solved with brute-force with acceptable
//! performance. To determine if a list of levels is safe we first determine
//! if the list is increasing or decreasing by looking at the first two
//! numbers. Then we can compare each pair to make sure that the difference
//! fits within the acceptable range. With this information in part one we
//! can just check how many safe reports we have. In part two, the brute force
//! solution is for any list that isn't already safe we just try removing one
//! element at a time and check to see if it makes the report safe. If it does
//! then we can stop checking, and if we don't find a result after trying to
//! remove each element then it means the report can't be made safe.

/// The solution for the day two challenge.
///
/// As usual given the input string and a part we process the input going
/// line-by-line and parsing the numbers into actual integers. If we're in
/// part one we just do a safety check and increase the counter if it's safe.
/// In part two if it's already safe then we can increment the counter as
/// usual. If it's not safe then we try removing one element from the list at
/// a time and then recheck to see if it made the list safe. If it did then
/// we increment the counter and move on, if it didn't then we move on to
/// remove the next element and check again. If we were unable to make the
/// list safe after trying to remove all of the elements then we give up.
///
/// # Example
/// ```rust
/// # use aoc::y24d02::y24d02;
/// // probably read this from the input file...
/// let input = "3 1 2 3 4 3\n9 8 7 6 7\n7 10 8 10 11\n12 14 16 18 20";
/// assert_eq!(y24d02(input, 1), 1);
/// assert_eq!(y24d02(input, 2), 3);
/// ```
pub fn y24d02(input: &str, part: u32) -> u32 {
    let mut safe = 0;

    for line in input.lines() {
        let parts: Vec<u32> = line
            .split_whitespace()
            .map(|p| p.parse().unwrap())
            .collect();

        if part == 1 && is_safe(&parts) {
            safe += 1;
        } else if part == 2 {
            if is_safe(&parts) {
                safe += 1;
            } else {
                for i in 0..parts.len() {
                    let mut n = parts.to_vec();
                    n.remove(i);
                    if is_safe(&n) {
                        safe += 1;
                        break;
                    }
                }
            }
        }
    }

    safe
}

/// Given a list of integers the `is_safe` function determines if it follows
/// all of the rules given by the prompt: all of the numbers are either
/// increasing or they are all decreasing, and any two adjacent number must
/// differ by at least one and at most three.
fn is_safe(nums: &[u32]) -> bool {
    let mut is_increasing = true;
    if nums[0] > nums[1] {
        is_increasing = false;
    }

    let windows = nums.windows(2);
    for pair in windows {
        if (is_increasing && pair[0] > pair[1])
            || (!is_increasing && pair[1] > pair[0])
        {
            return false;
        }

        if is_increasing {
            let diff = pair[1] - pair[0];
            if !(1..=3).contains(&diff) {
                return false;
            }
        } else {
            let diff = pair[0] - pair[1];
            if !(1..=3).contains(&diff) {
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
        let mut vec = vec![1, 2, 3, 4, 5];
        assert_eq!(is_safe(&vec), true);

        vec = vec![5, 4, 3, 2, 1];
        assert_eq!(is_safe(&vec), true);

        vec = vec![1, 1, 2, 3, 4, 5];
        assert_eq!(is_safe(&vec), false);

        vec = vec![1, 2, 3, 4, 5, 5];
        assert_eq!(is_safe(&vec), false);

        vec = vec![5, 1, 2, 3, 4, 5];
        assert_eq!(is_safe(&vec), false);

        vec = vec![1, 2, 6, 7, 8];
        assert_eq!(is_safe(&vec), false);

        vec = vec![8, 7, 6, 2, 1];
        assert_eq!(is_safe(&vec), false);
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

        assert_eq!(y24d02(input, 1), 2);
        assert_eq!(y24d02(input, 2), 4);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2024/day02.txt").unwrap();

        assert_eq!(y24d02(&contents, 1), 390);
        assert_eq!(y24d02(&contents, 2), 439);
    }
}
