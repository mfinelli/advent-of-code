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

/// TODO
#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: u32,
    race_seconds: u32,
    rest_seconds: u32,
    distance: u32,
    points: u32,

    is_flying: bool,
    left_in_phase: u32,
}

impl Reindeer {
    /// TODO
    fn new(name: &str, speed: u32, race_seconds: u32, rest_seconds: u32) -> Reindeer {
        Reindeer {
            name: name.to_string(),
            speed: speed,
            race_seconds: race_seconds,
            rest_seconds: rest_seconds,
            distance: 0,
            points: 0,
            is_flying: true,
            left_in_phase: race_seconds,
        }
    }

    /// TODO
    fn award_point(&mut self) {
        self.points += 1;
    }

    /// TODO
    fn next(&mut self) {
        if self.is_flying {
            self.distance += self.speed;
        }

        self.left_in_phase -= 1;

        if self.left_in_phase == 0 {
            if self.is_flying {
                self.left_in_phase = self.rest_seconds;
            } else {
                self.left_in_phase = self.race_seconds;
            }

            self.is_flying = !self.is_flying;
        }
    }
}

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
pub fn y15d14(input: &str, seconds: u32, part: u32) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    // let mut distances = BinaryHeap::new();
    let mut reindeer = Vec::new();

    for line in lines {
        let text: Vec<&str> = line.split_whitespace().collect();
        let name = text[0];
        let speed = text[3].parse().unwrap();
        let race_seconds = text[6].parse().unwrap();
        let rest_seconds = text[13].parse().unwrap();

        reindeer.push(Reindeer::new(name, speed, race_seconds, rest_seconds));

        // reindeer.push(Reindeer {
        //     name: text[0].to_string(),
        //     speed: speed,
        //     race_seconds: race_seconds,
        //     rest_seconds: rest_seconds,
        //     distance: 0,
        //     points: 0,
        // });

        // distances.push(compute_distance(speed, race_seconds, rest_seconds, seconds));
    }

    for _ in 0..seconds {
        let mut winning_distance = BinaryHeap::new();

        for r in &mut reindeer {
            r.next();
            winning_distance.push(r.distance);
        }

        let current_winner = winning_distance.pop().unwrap();

        for r in &mut reindeer {
            if r.distance == current_winner {
                r.award_point();
            }
        }
    }

    // println!("{:?}", reindeer);

    if part == 1 {
        let mut distances = BinaryHeap::new();
        for r in &reindeer {
            distances.push(r.distance);
        }
        distances.pop().unwrap()
    } else {
        let mut points = BinaryHeap::new();
        for r in &reindeer {
            points.push(r.points);
        }
        points.pop().unwrap()
    }
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

        assert_eq!(y15d14(input, 1000, 1), 1120);
        assert_eq!(y15d14(input, 1000, 2), 689);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day14.txt").unwrap();

        assert_eq!(y15d14(&contents, 2503, 1), 2640);
        assert_eq!(y15d14(&contents, 2503, 2), 1102);
    }
}
