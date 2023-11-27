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

//! Advent of Code 2015 Day 13: <https://adventofcode.com/2015/day/13>
//!
//! TODO

use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap};

/// The solution for the day thirteen challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y15d13::y15d13;
/// // probably read this from the input file...
/// //let input = "";
/// //assert_eq!(y15d13(input), 0);
/// ```
pub fn y15d13(input: &str) -> i32 {
    let lines: Vec<_> = input.lines().collect();
    let mut happinesses: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    let mut totals = BinaryHeap::new();

    for line in lines {
        let text: Vec<&str> = line.split_whitespace().collect();

        let guest_a = text[0];
        let effect = text[2];
        let amount: i32 = if effect == "gain" {
            text[3].parse().unwrap()
        } else {
            -(text[3].parse::<i32>().unwrap())
        };
        let guest_b = text[10].strip_suffix(".").unwrap();

        let happiness = happinesses.entry(guest_a).or_default();
        happiness.insert(guest_b, amount);
    }

    for seating in happinesses.keys().permutations(happinesses.keys().len()) {
        let mut total = 0;

        for (a, b) in seating.into_iter().circular_tuple_windows() {
            total += happinesses.get(a).unwrap().get(b).unwrap();
            total += happinesses.get(b).unwrap().get(a).unwrap();
        }

        totals.push(total);
    }

    totals.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = concat!(
            "Alice would gain 54 happiness units by sitting next to Bob.\n",
            "Alice would lose 79 happiness units by sitting next to Carol.\n",
            "Alice would lose 2 happiness units by sitting next to David.\n",
            "Bob would gain 83 happiness units by sitting next to Alice.\n",
            "Bob would lose 7 happiness units by sitting next to Carol.\n",
            "Bob would lose 63 happiness units by sitting next to David.\n",
            "Carol would lose 62 happiness units by sitting next to Alice.\n",
            "Carol would gain 60 happiness units by sitting next to Bob.\n",
            "Carol would gain 55 happiness units by sitting next to David.\n",
            "David would gain 46 happiness units by sitting next to Alice.\n",
            "David would lose 7 happiness units by sitting next to Bob.\n",
            "David would gain 41 happiness units by sitting next to Carol.\n",
        );

        assert_eq!(y15d13(input), 330);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day13.txt").unwrap();

        assert_eq!(y15d13(&contents), 709);
    }
}
