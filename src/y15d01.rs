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

//! Advent of Code 2015 Day 1: <https://adventofcode.com/2015/day/1>
//!
//! These challenges are extremely simple and basically amount to incrementing
//! or decrementing a counter based on observed characters in a string.
//!
//! We implement this as two distinct functions as the return type is
//! different because depending on the input it would technically be possible
//! in the second part to never reach the basement (negative counter). The
//! second part also returns a strictly positive number whereas in part one
//! it could be possible to return a negative number.

/// The solution for part one of the day one challenge.
///
/// Given the input as a string we loop through the characters and either add
/// one or subtract one from a counter that starts at zero depending on if the
/// character is a `(`, or a `)`, respectively.
///
/// # Example
/// ```rust
/// # use aoc::y15d01::y15d01p1;
/// let input = "(()))((\n"; // probably read this from the input file...
/// assert_eq!(y15d01p1(input), 1);
/// ```
pub fn y15d01p1(input: &str) -> i32 {
    let chars: Vec<_> = input.trim().chars().collect();
    let mut floor = 0;

    for c in chars {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
    }

    floor
}

/// The solution for part two of the day one challenge.
///
/// This solution is extremely similar to the part one solution [`y15d01p1`]
/// with the exception that after each character we do a check to see if we're
/// on a negative floor, and if stop and return the current index.
///
/// # Example
/// ```rust
/// # use aoc::y15d01::y15d01p2;
/// let input = "(()))((\n"; // probably read this from the input file...
/// assert_eq!(y15d01p2(input), Some(5));
/// ```
pub fn y15d01p2(input: &str) -> Option<u32> {
    let chars: Vec<_> = input.trim().chars().collect();
    let mut floor = 0;

    for (i, &c) in chars.iter().enumerate() {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }

        if floor < 0 {
            return Some((i + 1).try_into().unwrap());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let mut input = "(())";
        assert_eq!(y15d01p1(&input), 0);

        input = "()()\n";
        assert_eq!(y15d01p1(&input), 0);

        input = "(((";
        assert_eq!(y15d01p1(&input), 3);

        input = "(()(()(\n";
        assert_eq!(y15d01p1(&input), 3);

        input = "))(((((\n";
        assert_eq!(y15d01p1(&input), 3);

        input = "())";
        assert_eq!(y15d01p1(&input), -1);

        input = "))(\n";
        assert_eq!(y15d01p1(&input), -1);

        input = ")))";
        assert_eq!(y15d01p1(&input), -3);

        input = ")())())\n";
        assert_eq!(y15d01p1(&input), -3);

        input = ")";
        assert_eq!(y15d01p2(&input), Some(1));

        input = "()())\n";
        assert_eq!(y15d01p2(&input), Some(5));
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day01.txt").unwrap();

        assert_eq!(y15d01p1(&contents), 232);
        assert_eq!(y15d01p2(&contents).unwrap(), 1783);
    }
}
