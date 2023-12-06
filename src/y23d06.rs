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
//! TODO

/// The solution for the day six challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y23d06::y23d06;
/// // probably read this from the input file...
/// let input = "";
/// assert_eq!(y23d06(input, 1), 0);
/// assert_eq!(y23d06(input, 2), 0);
/// ```
pub fn y23d06(input: &str, part: u32) -> u64 {
    let lines: Vec<_>= input.lines().collect();
    let times: Vec<u64> = if part == 2 {
        vec![lines[0].split_whitespace().skip(1).collect::<Vec<_>>().join("").parse().unwrap()]
    } else {
        lines[0].split_whitespace().skip(1).map(|d| d.parse().unwrap()).collect()
    };

    let distances: Vec<u64> = if part == 2 {
        vec![lines[1].split_whitespace().skip(1).collect::<Vec<_>>().join("").parse().unwrap()]
    } else {
        lines[1].split_whitespace().skip(1).map(|d| d.parse().unwrap()).collect()
    };

    // println!("{:?}", times);
    // println!("{:?}", distances);
    // println!("{:?}", times.iter().zip(distances).collect::<Vec<_>>());
    //

    let mut answer = 1;
    for (time, distance) in times.iter().zip(distances) {
        // println!("time: {}, distance: {}", time, distance);
        let mut count = 0;
        for t in 1..*time {
            let time_left = time-t;
            // println!("hold: {}, go: {}", t, time_left*t);
            if time_left*t > distance {
                count +=1
            }
        }

        answer *= count;
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn iit_works() {
        let input = "Time:      7  15   30\nDistance:  9  40  200\n";

        assert_eq!(y23d06(input, 1), 288);
        assert_eq!(y23d06(input, 2), 71503);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day06.txt").unwrap();

        assert_eq!(y23d06(&contents, 1), 316800);
        assert_eq!(y23d06(&contents, 2), 0);
    }
}
