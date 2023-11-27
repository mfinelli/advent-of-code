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
//! This problem is extremely similar to the problem from day nine and so an
//! extremely similar approach was taken.

use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap};

/// The solution for the day thirteen challenge.
///
/// We take the input as a string and a boolean specifying if we're also
/// attending dinner or not. Then much like on day nine we start by parsing the
/// input to compute the happiness change of each guest sitting next to every
/// other guest. If we're attending dinner then we also add a `Me` entry with
/// happiness change of `0`. Then again like in day nine we compute all of the
/// possible seating arrangements (permutations) and then calculate the
/// happiness for each guest. Differently from day nine we use circular windows
/// to account for the first and last guest in the list sitting next to each
/// other. Like in day nine we add the total happiness to a
/// [`std::collections::BinaryHeap`] to make getting the final, largest value
/// easy.
///
/// # Example
/// ```rust
/// # use aoc::y15d13::y15d13;
/// // probably read this from the input file...
/// let input = concat!(
///     "Bob would gain 10 happiness units by sitting next to Alice.\n",
///     "Bob would gain 20 happiness units by sitting next to Jim.\n",
///     "Bob would gain 30 happiness units by sitting next to Andy.\n",
///     "Alice would lose 10 happiness units by sitting next to Bob.\n",
///     "Alice would lose 20 happiness units by sitting next to Jim.\n",
///     "Alice would lose 30 happiness units by sitting next to Andy.\n",
///     "Jim would gain 10 happiness units by sitting next to Alice.\n",
///     "Jim would gain 20 happiness units by sitting next to Bob.\n",
///     "Jim would gain 30 happiness units by sitting next to Andy.\n",
///     "Andy would lose 10 happiness units by sitting next to Alice.\n",
///     "Andy would lose 20 happiness units by sitting next to Jim.\n",
///     "Andy would lose 30 happiness units by sitting next to Bob.",
/// );
/// assert_eq!(y15d13(input, false), 10);
/// assert_eq!(y15d13(input, true), 50);
/// ```
pub fn y15d13(input: &str, me: bool) -> i32 {
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
        let guest_b = text[10].strip_suffix('.').unwrap();

        let mut newme = HashMap::new();
        if me {
            newme.insert("Me", 0);
        }

        let happiness = happinesses.entry(guest_a).or_insert(newme);
        happiness.insert(guest_b, amount);
    }

    if me {
        let mut guests = HashMap::new();
        for &guest in happinesses.keys() {
            guests.insert(guest, 0);
        }
        happinesses.insert("Me", guests);
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

        assert_eq!(y15d13(input, false), 330);
        assert_eq!(y15d13(input, true), 286);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day13.txt").unwrap();

        assert_eq!(y15d13(&contents, false), 709);
        assert_eq!(y15d13(&contents, true), 668);
    }
}
