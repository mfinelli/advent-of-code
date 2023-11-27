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
//! It took me a little while to wrap my head around what look-and-say is but
//! once I figured it out this was relatively easy to implement.

/// The solution for the day ten challenge.
///
/// We take the input as a string and the number of times to perform the
/// look-and-say as arguments and then return the length of the resulting
/// string as requested by the prompt. We start by converting the input string
/// into a vector of integers and then run the look-and-say function on the
/// result over and over until we've run it the requested number of times.
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

/// This function implements the look-and-say algorithm. It's relatively
/// straightforward: we mark the first digit as the "previous" and then start
/// counting from the second digit (if there is one). At each digit we check
/// to see if the digit is the same as the previous one. If it is then we
/// increment the current counter. If it's not then we add the current count
/// and "previous" into the final result, and then set the "previous" to the
/// new current and then reset the counter. Finally, push the final count and
/// digit onto the vector and we're done.
fn lookandsay(input: Vec<u32>) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();

    let mut previous = input[0];
    let mut count = 1;

    for digit in input.iter().skip(1) {
        if *digit != previous {
            result.push(count);
            result.push(previous);
            previous = *digit;
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
    fn it_works() {
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
        assert_eq!(y15d10(&contents, 50), 5103798);
    }
}
