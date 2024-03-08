/* Copyright 2023-2024 Mario Finelli
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

//! Advent of Code 2023 Day 5: <https://adventofcode.com/2023/day/5>
//!
//! TODO

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// The solution for the day five challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y23d05::y23d05;
/// // probably read this from the input file...
/// let input = "";
/// assert_eq!(y23d05(input, 1), 0);
/// assert_eq!(y23d05(input, 2), 0);
/// ```
pub fn y23d05(input: &str, part: u8) -> i64 {
    if part == 1 {
        return y23d05p1(input);
    } else {
        return y23d05p2(input);
    }
}

fn y23d05p1(input: &str) -> i64 {
    let mut locations = BinaryHeap::new();
    let mut seeds: Vec<i64> = Vec::new();

    let lines: Vec<_> = input.lines().collect();
    for number in lines[0].split_whitespace().skip(1) {
        seeds.push(number.parse().unwrap());
    }

    let mut conversions: Vec<Vec<(i64, i64, i64)>> = Vec::new();
    let mut current = Vec::new();
    for line in lines.iter().skip(3) {
        if line == &"" {
            continue;
        }

        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() != 3 {
            conversions.push(current);
            current = Vec::new();
            continue;
        }

        let destination: i64 = parts[0].parse().unwrap();
        let start: i64 = parts[1].parse().unwrap();
        let delta: i64 = parts[2].parse().unwrap();

        current.push((destination, start, delta));
    }
    conversions.push(current);

    for seed in seeds {
        let mut newseed = seed;
        for conversion in &conversions {
            for (destination, start, delta) in conversion {
                let range = *start..*start + *delta;
                if range.contains(&newseed) {
                    let diff = destination - start;
                    newseed += diff;
                    break;
                }
            }
        }

        locations.push(Reverse(newseed));
    }

    let Reverse(shortest) = locations.pop().unwrap();
    shortest
}

fn y23d05p2(input: &str) -> i64 {
    let mut locations = BinaryHeap::new();
    let mut intervals: Vec<(i64, i64, usize)> = Vec::new();

    let lines: Vec<_> = input.lines().collect();
    for pair in lines[0]
        .split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .chunks(2)
    {
        let x1: i64 = pair[0].parse().unwrap();
        let dx: i64 = pair[1].parse().unwrap();

        intervals.push((x1, x1 + dx, 1));
    }

    let mut conversions: Vec<Vec<(i64, i64, i64)>> = Vec::new();
    let mut current = Vec::new();
    for line in lines.iter().skip(3) {
        if line == &"" {
            continue;
        }

        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() != 3 {
            conversions.push(current);
            current = Vec::new();
            continue;
        }

        let destination: i64 = parts[0].parse().unwrap();
        let start: i64 = parts[1].parse().unwrap();
        let delta: i64 = parts[2].parse().unwrap();

        current.push((destination, start, delta));
    }
    conversions.push(current);

    loop {
        match intervals.pop() {
            Some(interval) => {
                let (mut x1, mut x2, level) = interval;
                if level == 8 {
                    locations.push(Reverse(x1));
                    continue;
                }

                let mut did_something = false;
                for conversion in &conversions[level - 1] {
                    let (z, y1, dy) = conversion;
                    let y2 = y1 + dy;
                    let diff = z - y1;

                    if x2 <= *y1 || y2 <= x1 {
                        // no overlap
                        continue;
                    }

                    if x1 < *y1 {
                        // split original interval at y1
                        intervals.push((x1, *y1, level));
                        x1 = *y1;
                    }

                    if y2 < x2 {
                        // split original interval at y2
                        intervals.push((y2, x2, level));
                        x2 = y2;
                    }

                    // perfect overlap -> make conversion and let pass to next
                    // level
                    intervals.push((x1 + diff, x2 + diff, level + 1));
                    did_something = true;
                    break;
                }

                if !did_something {
                    intervals.push((x1, x2, level + 1));
                }
            }

            None => {
                break;
            }
        }
    }

    let Reverse(shortest) = locations.pop().unwrap();
    shortest
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = concat!(
            "seeds: 79 14 55 13\n",
            "\n",
            "seed-to-soil map:\n",
            "50 98 2\n",
            "52 50 48\n",
            "\n",
            "soil-to-fertilizer map:\n",
            "0 15 37\n",
            "37 52 2\n",
            "39 0 15\n",
            "\n",
            "fertilizer-to-water map:\n",
            "49 53 8\n",
            "0 11 42\n",
            "42 0 7\n",
            "57 7 4\n",
            "\n",
            "water-to-light map:\n",
            "88 18 7\n",
            "18 25 70\n",
            "\n",
            "light-to-temperature map:\n",
            "45 77 23\n",
            "81 45 19\n",
            "68 64 13\n",
            "\n",
            "temperature-to-humidity map:\n",
            "0 69 1\n",
            "1 0 69\n",
            "\n",
            "humidity-to-location map:\n",
            "60 56 37\n",
            "56 93 4\n",
        );

        assert_eq!(y23d05(input, 1), 35);
        assert_eq!(y23d05(input, 2), 46);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day05.txt").unwrap();

        assert_eq!(y23d05(&contents, 1), 324724204);
        assert_eq!(y23d05(&contents, 2), 150985364);
    }
}
