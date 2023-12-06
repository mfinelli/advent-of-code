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

//! Advent of Code 2023 Day 6: <https://adventofcode.com/2023/day/6>
//!
//! Today's challenge was pretty easy. There aren't any tricks or gotchas, just
//! parse the input and do the first thing that comes to mind. Fortunately,
//! part two was a simple expansion and didn't increase the runtime
//! significantly with re-using the solution from part one.

/// The solution for the day six challenge.
///
/// As usual, we take the input as a string and an integer for whether we're
/// solving part `1` or part `2`. The difference in part just changes how we
/// parse the input. Either we have an array times and distances or we
/// concatenate the input to create a single race (which we then turn into a
/// vector of length one so that we can re-use the solution from part one
/// without any other modifications). Once we have our input arrays we `zip`
/// them together to create an array that matches the times and distances for
/// each race and then proceed to do the calculation as described in the
/// prompt. We start from `1` (holding the boat for `0` milliseconds causes it
/// to go nowhere) until the length of the race (minus one as just like not
/// holding it at all holding it for the entire race causes it to go nowhere).
/// With the remaining time and speed we calculate how far the boat will go and
/// if it's greater than the record for the race then we increment our counter
/// for that race. To get our final result we just multiply all of our counters
/// together (in the actual implementation we multiply them as we go).
///
/// # Example
/// ```rust
/// # use aoc::y23d06::y23d06;
/// // probably read this from the input file...
/// let input = "Time: 10 40\nDistance: 10 100";
/// assert_eq!(y23d06(input, 1), 245);
/// assert_eq!(y23d06(input, 2), 1021);
/// ```
pub fn y23d06(input: &str, part: u32) -> u64 {
    let lines: Vec<_> = input.lines().collect();

    let times: Vec<u64> = if part == 2 {
        vec![lines[0]
            .split_whitespace()
            .skip(1)
            .collect::<Vec<_>>()
            .join("")
            .parse()
            .unwrap()]
    } else {
        lines[0]
            .split_whitespace()
            .skip(1)
            .map(|d| d.parse().unwrap())
            .collect()
    };

    let distances: Vec<u64> = if part == 2 {
        vec![lines[1]
            .split_whitespace()
            .skip(1)
            .collect::<Vec<_>>()
            .join("")
            .parse()
            .unwrap()]
    } else {
        lines[1]
            .split_whitespace()
            .skip(1)
            .map(|d| d.parse().unwrap())
            .collect()
    };

    let mut result = 1;
    for (time, distance) in times.iter().zip(distances) {
        let mut count = 0;

        for t in 1..*time {
            let time_left = time - t;
            if time_left * t > distance {
                count += 1
            }
        }

        result *= count;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = "Time:      7  15   30\nDistance:  9  40  200\n";

        assert_eq!(y23d06(input, 1), 288);
        assert_eq!(y23d06(input, 2), 71503);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day06.txt").unwrap();

        assert_eq!(y23d06(&contents, 1), 316800);
        assert_eq!(y23d06(&contents, 2), 45647654);
    }
}
