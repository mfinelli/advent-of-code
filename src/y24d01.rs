/* Copyright 2024 Mario Finelli
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

//! Advent of Code 2024 Day 1: <https://adventofcode.com/2024/day/1>
//!
//! A nice, straightforward challenge to start the year's puzzles. In part one
//! we just maintain two min-heaps with each of the numbers from the two lists
//! and then pop entries from both lists to find their distance. In part two
//! we need to find how many time each number from the left list appears in the
//! right one. To do this as we parse the numbers we create maintain a
//! [`std::collections::HashMap`] with each number on the left as the key and
//! then for each number on the right we increase the number of times we've
//! seen that number. To get the final answer we just need to loop through all
//! of the left numbers in the map and perform the required multiplication.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

/// The solution for part one of the day one challenge.
///
/// We start by parsing the input: for each line separate the numbers by their
/// whitespace and then parse them into integers and add them to the respective
/// min-heaps. Then we just pop from the heap while there are still values
/// (we do the check on the left heap and then just unconditionally pop from
/// the right heap because the number of items in each heap is the same) and
/// add the difference to the total sum.
///
/// # Example
/// ```rust
/// # use aoc::y24d01::y24d01p1;
/// // probably read this from the input file...
/// let input = concat!(
///     "1   2\n",
///     "3   4",
/// );
/// assert_eq!(y24d01p1(input), 2);
/// ```
pub fn y24d01p1(input: &str) -> i32 {
    let mut left = BinaryHeap::new();
    let mut right = BinaryHeap::new();
    let mut sum = 0;

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();

        let l: i32 = parts[0].parse().unwrap();
        let r: i32 = parts[1].parse().unwrap();

        left.push(Reverse(l));
        right.push(Reverse(r));
    }

    while let Some(Reverse(l)) = left.pop() {
        let Reverse(r) = right.pop().unwrap();
        sum += i32::abs(l - r);
    }

    sum
}

/// The solution for part two of the day one challenge.
///
/// We start by parsing the input as in part one, but this time we use the
/// left number as the key in a [`std::collections::HashMap`] (which we
/// initialize as `0` if we haven't seen it yet) and then the right number if
/// we haven't seen yet we initialize it as `1` or otherwise we add one to the
/// count. We keep track of which numbers appear in the left list (otherwise
/// if we just use the keys we would use numbers that were only on the right
/// that have count 1 but were never in the left list) so that we can then loop
/// through them and multiply the key by the count for that key to get the
/// final answer.
///
/// # Example
/// ```rust
/// # use aoc::y24d01::y24d01p2;
/// // probably read this from the input file...
/// let input = concat!(
///     "1   2\n",
///     "3   4\n",
///     "2   2\n",
/// );
/// assert_eq!(y24d01p2(input), 4);
/// ```
pub fn y24d01p2(input: &str) -> u32 {
    let mut counts = HashMap::new();
    let mut lefts = Vec::new();
    let mut sum = 0;

    let lines: Vec<_> = input.lines().collect();
    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();

        let l: u32 = parts[0].parse().unwrap();
        let r: u32 = parts[1].parse().unwrap();

        counts.entry(l).or_insert(0);
        counts.entry(r).and_modify(|e| *e += 1).or_insert(1);
        lefts.push(l);
    }

    for left in lefts {
        sum += left * counts.get(&left).unwrap();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n";
        assert_eq!(y24d01p1(input), 11);
        assert_eq!(y24d01p2(input), 31);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2024/day01.txt").unwrap();

        assert_eq!(y24d01p1(&contents), 2176849);
        assert_eq!(y24d01p2(&contents), 23384288);
    }
}
