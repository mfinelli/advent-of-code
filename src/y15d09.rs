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
//! This challenge turned out to be pretty easy. The list of inputs is small
//! so we don't need to worry about building any graphs and then trying to
//! find the shortest path or anything. We can just compute all of the possible
//! trips and their distances and then get the shortest/longest.

use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

/// The solution for the day nine challenge.
///
/// We take the input as a string and the problem part as an integer as usual.
/// We start by iterating over the lines and parsing the city names and the
/// distances. We maintain a [`std::collections::HashMap`] of the city names
/// with their values equal to another HashMap that contains the other cities
/// and their distances. This way we can lookup the distance from one city to
/// any other city. Then we get all of the permutations of the cities which
/// gives us all of the possible trips that we could take and then look at them
/// by two to give us each leg of the trip. Just get the distance for the trip
/// and then add it to the [`std::collections::BinaryHeap`]s that we've been
/// maintaining of the trip distances. If we're on part one then get the value
/// from the min-heap, otherwise the value from the max-heap.
///
/// # Example
/// ```rust
/// # use aoc::y15d09::y15d09;
/// // probably read this from the input file...
/// let input = "A to B = 10\nB to C = 15\nA to C = 25\n";
/// assert_eq!(y15d09(input, 1), 25);
/// assert_eq!(y15d09(input, 2), 40);
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

        let city = distances.entry(city1).or_default();
        city.insert(city2, distance);

        let city = distances.entry(city2).or_default();
        city.insert(city1, distance);
    }

    let paths = distances.keys().permutations(distances.keys().len());
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
