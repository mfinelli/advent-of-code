/* Copyright 2022-2023 Mario Finelli
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

//! Advent of Code 2015 Day 5: <https://adventofcode.com/2015/day/5>
//!
//! This challenge is relatively straightforward, it just needs to compare
//! various substrings to check for matches and other conditions.

/// The solution for the day five challenge.
///
/// We expect the input as a string and either part `1` or part `2` to decide
/// which ruleset to use to determine "nice" strings.
///
/// In part one nice strings contain at least three vowels (`aeiou` only),
/// contains at least one letter that appears twice in a row, and does _not_
/// contain any of these substrings: `ab`, `cd`, `pq`, or `xy`. The final rule
/// takes precedence over the first two, that is if a string meets the first
/// two criteria but has one of the forbidden substrings then it is still
/// naughty.
///
/// In part two the rules are slightly different. A nice string contains at
/// least one pair of non-overlapping (in position in the string) characters
/// and at least one character that repeats after one other character.
///
/// # Example
/// ```rust
/// # use aoc::y15d05::y15d05;
/// // probably read this from the input file...
/// let input = "abc\naeidd\nxxxxyx";
/// assert_eq!(y15d05(input, 1), 1);
/// assert_eq!(y15d05(input, 2), 1);
/// ```
pub fn y15d05(input: &str, part: u32) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut nice_strings = 0;

    for line in lines {
        if (part == 1 && is_nice_p1(line)) || (part == 2 && is_nice_p2(line)) {
            nice_strings += 1;
        }
    }

    nice_strings
}

/// Evaluates the part one rules for the given string and returns true or
/// false if the string is nice or not.
fn is_nice_p1(s: &str) -> bool {
    // these bad strings are immediately disqualifying so check them first
    let bad_strings = ["ab", "cd", "pq", "xy"];
    for bad_string in bad_strings {
        if s.contains(bad_string) {
            return false;
        }
    }

    let chars: Vec<_> = s.chars().collect();
    let mut vowels = 0;
    let mut double = false;
    for (i, c) in chars.iter().enumerate() {
        if c == &'a' || c == &'e' || c == &'i' || c == &'o' || c == &'u' {
            vowels += 1;
        }

        // if we're not on the last character we can check if the next
        // character is the same
        if i < chars.len() - 1 && c == &chars[i + 1] {
            double = true;
        }

        // we can stop checking as soon as we meet all of the conditions
        if double && vowels >= 3 {
            return true;
        }
    }

    false
}

/// Evaluates the part two rules for the given string and returns true or
/// false if the string is nice or not.
fn is_nice_p2(s: &str) -> bool {
    let chars: Vec<_> = s.chars().collect();

    let mut non_overlapping_pair = false;
    let mut repeat_with_middle = false;
    for i in 0..chars.len() {
        // if we're not on the last character (windows are two characters long
        // and we check the current character plus the next one) then we can
        // evaluate the other character windows
        if i != chars.len() - 1 {
            for (j, window) in chars.windows(2).enumerate() {
                // if we're in a window that overlaps with the current window
                // then skip it (note the last condition checks to make sure
                // we're on at least the second character before trying to
                // discard previous windows that can't exist)
                if i == j || i + 1 == j || (i >= 1 && i - 1 == j) {
                    continue;
                }

                if chars[i] == window[0] && chars[i + 1] == window[1] {
                    non_overlapping_pair = true;
                }
            }
        }

        // if we're not on the penultimate character we can look ahead two
        // characters to see if it matches the current character
        if i < chars.len() - 2 && chars[i] == chars[i + 2] {
            repeat_with_middle = true;
        }

        // like in part one we can stop evaluating as soon as we meet all of
        // the conditions
        if non_overlapping_pair && repeat_with_middle {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_is_nice_p1() {
        assert!(!is_nice_p1("aei"));
        assert!(!is_nice_p1("xazegov"));
        assert!(!is_nice_p1("aeiouaeiouaeiou"));
        assert!(!is_nice_p1("xx"));
        assert!(!is_nice_p1("abcdde"));
        assert!(!is_nice_p1("aabbccdd"));
        assert!(is_nice_p1("ugknbfddgicrmopn"));
        assert!(is_nice_p1("aaa"));
        assert!(!is_nice_p1("jchzalrnumimnmhp"));
        assert!(!is_nice_p1("haegwjzuvuyypxyu"));
        assert!(!is_nice_p1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_is_nice_p2() {
        assert!(is_nice_p2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice_p2("xxyxx"));
        assert!(!is_nice_p2("uurcxstgmygtbstg"));
        assert!(!is_nice_p2("ieodomkazucvgmuy"));
    }

    #[test]
    fn it_works() {
        let input = concat!(
            "ugknbfddgicrmopn\n",
            "aaa\n",
            "jchzalrnumimnmhp\n",
            "haegwjzuvuyypxyu\n",
            "dvszwmarrgswjxmb\n",
            "qjhvhtzxzqqjkmpb\n",
            "xxyxx\n",
            "uurcxstgmygtbstg\n",
            "ieodomkazucvgmuy\n",
        );

        assert_eq!(y15d05(input, 1), 2);
        assert_eq!(y15d05(input, 2), 2);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day05.txt").unwrap();

        assert_eq!(y15d05(&contents, 1), 238);
        assert_eq!(y15d05(&contents, 2), 69);
    }
}
