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

//! Advent of Code 2015 Day 9: <https://adventofcode.com/2015/day/9>
//!
//! TODO

use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

/// The solution for the day nine challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y15d09::y15d09;
/// // probably read this from the input file...
/// let input = "";
/// assert_eq!(y15d09(input), 0);
/// ```
pub fn y15d09(input: &str, part: u32) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut distances: HashMap<&str, HashMap<&str, u32>> = HashMap::new();
    let mut shortest_trips = BinaryHeap::new();
    let mut longest_trips = BinaryHeap::new();

    for line in lines {
        let text: Vec<&str> = line.split_whitespace().collect();
        let city1 = text[0];
        let city2 = text[2];
        let distance: u32 = text[4].parse().unwrap();

        let city = distances.entry(city1).or_insert(HashMap::new());
        city.insert(city2, distance);

        let city = distances.entry(city2).or_insert(HashMap::new());
        city.insert(city1, distance);
    }

    let paths = distances
        .keys()
        .permutations(distances.keys().len())
        .into_iter();
    for path in paths {
        let mut distance = 0;

        for trip in path.windows(2) {
            distance += distances.get(trip[0]).unwrap().get(trip[1]).unwrap();
        }

        shortest_trips.push(Reverse(distance));
        longest_trips.push(distance);
    }

    if part == 1 {
        let Reverse(shortest) = shortest_trips.pop().unwrap();
        shortest
    } else {
        longest_trips.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = concat!(
            "London to Dublin = 464\n",
            "London to Belfast = 518\n",
            "Dublin to Belfast = 141",
        );

        assert_eq!(y15d09(input, 1), 605);
        assert_eq!(y15d09(input, 2), 982);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day09.txt").unwrap();

        assert_eq!(y15d09(&contents, 1), 207);
        assert_eq!(y15d09(&contents, 2), 804);
    }
}
