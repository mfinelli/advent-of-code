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
//! A rather challenging problem for day one! Part one was very straightforward
//! but it took me a while to figure out how to solve part two. My initial
//! implementation used a regex to try and account for both numbers and the
//! number-words but there were many edge cases and after a few wrong
//! submission attempts I gave up and got the trick from the subreddit. The
//! trick is rather simple: just replace the number words with the number
//! wrapped by its number word (e.g., "one" becomes "one1one"). This lets us
//! also account for multiple numbers encoded into a single word (see example
//! below in the function description).

/// The solution for part one of the day one challenge.
///
/// We take the input as a string and the part that we're solving (in part `2`
/// we treat number-words as their number equivalents). We loop through each
/// line and if we're in part two then we replace the number-words with their
/// number equivalents (we wrap the number with its number-word so that if
/// there are two words together we can account for both of them e.g., eightwo
/// should provide both an 8 and a 2). Then we check each character from the
/// input until we find a number match for the first digit and then we reverse
/// the input and check again to find the second digit. We put them together,
/// parse the result as an integer and add it to our total.
///
/// # Example
/// ```rust
/// # use aoc::y23d01::y23d01;
/// // probably read this from the input file...
/// let input = "one1twothree4\n24\nthree4five";
/// assert_eq!(y23d01(input, 1), 82);
/// assert_eq!(y23d01(input, 2), 73);
/// ```
pub fn y23d01(input: &str, part: u32) -> u32 {
    let numbers = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut total = 0;

    for line in input.lines() {
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;

        let search = if part == 2 {
            line.replace("zero", "zero0zero")
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
        } else {
            line.to_string()
        };

        for c in search.chars() {
            if numbers.contains(&c) {
                first = Some(c);
                break;
            }
        }

        for c in search.chars().rev() {
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
        let mut input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n";
        assert_eq!(y23d01(input, 1), 142);

        input = concat!(
            "two1nine\n",
            "eightwothree\n",
            "abcone2threexyz\n",
            "xtwone3four\n",
            "4nineeightseven2\n",
            "zoneight234\n",
            "7pqrstsixteen\n",
        );
        assert_eq!(y23d01(input, 2), 281);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day01.txt").unwrap();

        assert_eq!(y23d01(&contents, 1), 55712);
        assert_eq!(y23d01(&contents, 2), 55413);
    }
}
