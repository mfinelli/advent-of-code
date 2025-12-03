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

//! Advent of Code 2025 Day 2: <https://adventofcode.com/2025/day/2>
//!
//! TODO

/// The solution for the day two challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y25d02::y25d02;
/// // probably read this from the input file...
/// let input = concat!(
/// );
/// assert_eq!(y25d02(input), 1);
/// ```
pub fn y25d02(input: &str, part: u32) -> u64 {
    let mut sum = 0;

    let ranges: Vec<_> = input.trim().split(',').collect();
    for range in ranges {
        let parts: Vec<_> = range.split('-').collect();
        let start: u64 = parts[0].parse().unwrap();
        let end: u64 = parts[1].parse().unwrap();

        for i in start..end+1 {
            if (part == 1 && is_invalid(i)) || (part == 2 && is_invalid_pt2(i)) {
                sum += i;
            }
        }
    }

    sum
}

/// TODO
fn is_invalid(num: u64) -> bool {
    let s = num.to_string();

    if s.len() % 2 != 0 {
        // numbers of an odd length _can't_ be invalid
        return false;
    }

    if s[0..s.len()/2] == s[s.len()/2..] {
        return true
    }

    false
}

/// TODO
/// This is not clever... it's just brute force but it completes in under a
/// second so it's good enough
fn is_invalid_pt2(num: u64) -> bool {
    let s = num.to_string();

    // println!("checking: {}", s);

    for window in 1..s.len()/2+1 {
        // println!("window size is: {}", window);

        if s.len() % window != 0 {
            // println!("window size doesn't divide equally\n");

            // this window size doesn't divide equally
            continue;
        }

        let mut invalid = true;
        let repeat = s[0..window].to_string();
        // println!("match string is {}", repeat);
        for i in 1..s.len()/window {
            if s[i*window..i*window+window] != repeat {
                // println!("mismatch {} != {}\n", s[i*window..i*window+window].to_string(), repeat);
                invalid = false;
                break;
            }
        }

        if invalid {
            // println!("invalid!\n\n");
            return true;
        }
    }

    // println!("valid!\n\n");
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_is_invalid() {
        assert_eq!(is_invalid(55), true);
        assert_eq!(is_invalid(6464), true);
        assert_eq!(is_invalid(123123), true);
        assert_eq!(is_invalid(101), false);
    }

    #[test]
    fn test_is_invalid_pt2() {
        assert_eq!(is_invalid_pt2(12341234), true);
        assert_eq!(is_invalid_pt2(123123123), true);
        assert_eq!(is_invalid_pt2(1212121212), true);
        assert_eq!(is_invalid_pt2(1111111), true);
        assert_eq!(is_invalid_pt2(101), false);
    }

    #[test]
    fn it_works() {
        let input = concat!(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,",
            "1698522-1698528,446443-446449,38593856-38593862,565653-565659,",
            "824824821-824824827,2121212118-2121212124",
        );

        assert_eq!(y25d02(input, 1), 1227775554);
        assert_eq!(y25d02(input, 2), 4174379265);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2025/day02.txt").unwrap();

        assert_eq!(y25d02(&contents, 1), 43952536386);
        assert_eq!(y25d02(&contents, 2), 54486209192);
    }
}
