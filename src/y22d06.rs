/* Copyright 2022 Mario Finelli
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

//! Advent of Code 2022 Day 6: <https://adventofcode.com/2022/day/6>
//!
//! This challenge essentially boils down to checking if (sliding) windows of
//! `n` size are distinct. A start of packet/message starts when the preceding
//! `n` characters are distinct. The solution is therefore to look at each
//! window and add the elements to a [`std::collections::HashSet`]. If after
//! adding all of the elements the size of the set is the size of the window
//! then all elements are distinct and we've found the "start" and return the
//! current counter plus the size as an offset to account for the `size`
//! characters at the beginning of the string.

use std::collections::HashSet;

/// The solution for the day six challenge.
///
/// Given the input as a string it splits it into characters and then evaluates
/// each window of `size` characters while maintaining a counter of the current
/// window. Once all of the elements are distinct then we return the counter
/// plus the `size` offset.
///
/// # Example
/// ```rust
/// # use aoc::y22d06::y22d06;
/// let input = "abbccdefghij\n"; // probably read this from the input file...
/// assert_eq!(y22d06(&input, 4), 8);
/// ```
pub fn y22d06(input: &str, size: usize) -> u32 {
    let chars: Vec<_> = input.trim().chars().collect();
    for (i, window) in chars.windows(size).enumerate() {
        let mut set = HashSet::new();
        for c in window {
            set.insert(c);
        }
        if set.len() == size {
            return (i + size) as u32;
        }
    }

    // we didn't find a match; return an impossible value
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let mut input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(y22d06(input, 4), 5);
        assert_eq!(y22d06(input, 14), 23);

        input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(y22d06(input, 4), 6);
        assert_eq!(y22d06(input, 14), 23);

        input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(y22d06(input, 4), 10);
        assert_eq!(y22d06(input, 14), 29);

        input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(y22d06(input, 4), 11);
        assert_eq!(y22d06(input, 14), 26);

        input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(y22d06(input, 14), 19);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2022/day06.txt").unwrap();

        assert_eq!(y22d06(&contents, 4), 1802);
        assert_eq!(y22d06(&contents, 14), 3551);
    }
}
