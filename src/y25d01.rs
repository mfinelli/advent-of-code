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

//! Advent of Code 2024 Day 1: <https://adventofcode.com/2024/day/1>
//!
//! TODO

/// The solution for the day one challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y25d01::y25d01;
/// // probably read this from the input file...
/// let input = concat!(
///     "R8\n",
///     "L58\n",
/// );
/// assert_eq!(y25d01(input), 1);
/// ```
pub fn y25d01(input: &str) -> u32 {
    let mut pwd = 0;
    let mut pos = 50; // dial starts at 50

    let lines: Vec<_> = input.lines().collect();
    for line in lines {
        println!("dial is starting at pos: {}", pos);

        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        let mut num: i32 = chars.collect::<String>().parse().unwrap();

        while num > 100 {
            num -= 100;
        }
        println!("moving {} {} places", dir, num);

        if dir == 'R' {
            pos += num;
            if pos > 99 {
                pos -= 100;
            }
        } else {
            pos -= num;
            if pos < 0 {
                pos += 100;
            }
        }

        println!("dial arrives at {}", pos);

        if pos == 0 {
            pwd += 1;
        }
    }

    pwd
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        assert_eq!(y25d01(input), 3);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2025/day01.txt").unwrap();

        assert_eq!(y25d01(&contents), 0);
    }
}
