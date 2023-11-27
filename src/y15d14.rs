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

//! Advent of Code 2015 Day 14: <https://adventofcode.com/2015/day/14>
//!
//! TODO

use std::collections::BinaryHeap;

/// The solution for the day fourteen challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y15d14::y15d14;
/// // probably read this from the input file...
/// let input = "";
/// assert_eq!(y15d13(input, 100), 10);
/// ```
pub fn y15d14(input: &str, seconds: u32) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut distances = BinaryHeap::new();

    for line in lines {
        let text: Vec<&str> = line.split_whitespace().collect();
        let speed = text[3].parse().unwrap();
        let race_seconds = text[6].parse().unwrap();
        let rest_seconds = text[13].parse().unwrap();

        distances.push(compute_distance(speed, race_seconds, rest_seconds, seconds));
    }

    distances.pop().unwrap()
}

/// TODO
pub fn compute_distance(speed: u32, flying: u32, rest: u32, seconds: u32) -> u32 {
    let mut is_flying = true;
    let mut phase = flying;
    let mut total = 0;

    for _ in 0..seconds {
        if is_flying {
            total += speed;
        }

        phase -= 1;
        
        if phase == 0 {
            if is_flying {
                phase = rest;
            } else {
                phase = flying;
            }

            is_flying = !is_flying;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_compute_distance() {
        assert_eq!(compute_distance(14, 10, 127, 1000), 1120);
        assert_eq!(compute_distance(16, 11, 162, 1000), 1056);
    }

    #[test]
    fn it_works() {
        let input = concat!(
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\n",
            "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
        );

        assert_eq!(y15d14(input, 1000), 1120);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day14.txt").unwrap();

        assert_eq!(y15d14(&contents, 2503), 2640);
    }
}
