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
//! I originally brute-forced this problem for part one simply iterating
//! the prompted number of times to compute the distance travelled but sure
//! that there was some math that I could do to calculate the answer much
//! faster. After seeing part two I decided to maintain my approach but refine
//! it to keep track of the state of all of the reindeer at each second so
//! that I could correctly award the points to the current winner.

use std::collections::BinaryHeap;

/// Reindeer tracks the state of each reindeer during the race.
#[derive(Debug, PartialEq)]
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
    /// Initializes a new reindeer object, given their name, race speed, how
    /// long they can race before needing to rest, and how long they need to
    /// rest before they can race again. Reindeer then start in the racing
    /// (flying phase) at distance `0` with the number of seconds that they can
    /// race set as the remaining time to spend in the current phase.
    fn new(
        name: &str,
        speed: u32,
        race_seconds: u32,
        rest_seconds: u32,
    ) -> Reindeer {
        Reindeer {
            name: name.to_string(),
            speed,
            race_seconds,
            rest_seconds,
            distance: 0,
            points: 0,
            is_flying: true,
            left_in_phase: race_seconds,
        }
    }

    /// A simple function to increment the current points that a reindeer has.
    fn award_point(&mut self) {
        self.points += 1;
    }

    /// This function computes the state of the reindeer for the next second.
    /// If the reindeer is currently in the racing (flying) phase then we
    /// increment its current distance by its speed. Then we decrement the
    /// phase counter. If we've reached the end of the phase (i.e., the counter
    /// is `0`) then we switch the phase and set the counter to the new phase
    /// time.
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
/// As usual we accept the input as a string. Then we take how many seconds of
/// race to process and whether we're doing part `1` (return the maximum
/// distance) or part `2` (return the maximum number of points). We start by
/// parsing the input and creating our `Reindeer` objects to track the state
/// as the race progresses. Then for the desired number or race seconds we
/// process each second one-at-a-time. For each second we create a new max-heap
/// (using the venerable [`std::collections::BinaryHeap`] to keep track of the
/// current maximum travelled distance. We process each reindeer and add their
/// distance to the heap. We then pop the heap to see what the maximum distance
/// is across all reindeer and then any reindeer who is currently at that
/// winning distance is awarded a point. To return the desired final answer we
/// create a new max-heap and then add each reindeer's distance for part one or
/// number of obtained points for part two. Popping the heap gives us the
/// answer.
///
/// # Example
/// ```rust
/// # use aoc::y15d14::y15d14;
/// // probably read this from the input file...
/// let input = concat!(
///     "Santa can fly 25 km/s for 10 seconds, but ",
///     "then must rest for 10 seconds."
/// );
/// assert_eq!(y15d14(input, 100, 1), 1250);
/// assert_eq!(y15d14(input, 100, 2), 100);
/// ```
pub fn y15d14(input: &str, seconds: u32, part: u32) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut reindeer = Vec::new();

    for line in lines {
        let text: Vec<&str> = line.split_whitespace().collect();
        let name = text[0];
        let speed = text[3].parse().unwrap();
        let race_seconds = text[6].parse().unwrap();
        let rest_seconds = text[13].parse().unwrap();

        reindeer.push(Reindeer::new(name, speed, race_seconds, rest_seconds));
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_reindeer_new() {
        assert_eq!(
            Reindeer::new("Test", 10, 15, 20),
            Reindeer {
                name: "Test".to_string(),
                speed: 10,
                race_seconds: 15,
                rest_seconds: 20,
                distance: 0,
                points: 0,
                is_flying: true,
                left_in_phase: 15,
            }
        );
    }

    #[test]
    fn test_reindeer_award_point() {
        let mut r = Reindeer::new("Test", 1, 1, 1);
        assert_eq!(r.points, 0);
        r.award_point();
        assert_eq!(r.points, 1);
    }

    #[test]
    fn test_reindeer_next() {
        let mut comet = Reindeer::new("Comet", 14, 10, 127);
        let mut dancer = Reindeer::new("Dancer", 16, 11, 162);

        for _ in 0..1000 {
            comet.next();
            dancer.next();
        }

        assert_eq!(comet.distance, 1120);
        assert_eq!(dancer.distance, 1056);
    }

    #[test]
    fn it_works() {
        let input = concat!(
            "Comet can fly 14 km/s for 10 seconds, but ",
            "then must rest for 127 seconds.\n",
            "Dancer can fly 16 km/s for 11 seconds, but ",
            "then must rest for 162 seconds.\n",
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
