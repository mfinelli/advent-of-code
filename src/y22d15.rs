/* Copyright 2022-2023 Mario Finelli
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

//! Advent of Code 2022 Day 15: <https://adventofcode.com/2022/day/15>
//!
//! Part one of this challenge is not too hard, we just need to find how many
//! places the beacon _can't_ be (for a single row). In part two, the challenge
//! is kind of reversed where we need to find the only place that the beacon
//! _can_ be.

use regex::Regex;
use std::collections::{HashMap, HashSet};

/// The solution for part one of the day fifteen challenge.
///
/// This is a somewhat naive and brute force approach to solving the problem
/// given the puzzle input as a string and an integer of the row that we want
/// to compute. We start by parsing the puzzle input to extract the locations
/// of the sensors and the beacon that they detect. Because a position that
/// already contains a beacon can't be a position that can't contain a beacon
/// we need to track any beacons that are on the row that we're interested in
/// so that we can substract them from the final total. We then need to find
/// the [Manhattan Distance](https://en.wikipedia.org/wiki/Taxicab_geometry)
/// and then we check each row (both above and below) from the sensor until
/// the end of the Manhattan distance. At each step we reduce the range along
/// the x-axis to account for being farther from the starting point. If we
/// match the row that we're interested in then we save the range for later.
/// After we've processed all of the sensors then we can loop through each
/// range and insert its x-coordinate into a tracking set. Finally, to get the
/// answer to the challenge we return the total number of x-coordinates that
/// we have minus the number of beacons on the row.
///
/// # Example
/// ```rust
/// # use aoc::y22d15::y22d15p1;
/// // probably read the from the input file...
/// let input = concat!(
///     "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n",
///     "Sensor at x=9, y=16: closest beacon is at x=10, y=16\n",
///     "Sensor at x=13, y=2: closest beacon is at x=15, y=3",
/// );
/// assert_eq!(y22d15p1(input, 15), 9);
/// ```
pub fn y22d15p1(input: &str, row: i32) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut rows: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    let mut beacons = HashSet::new();
    let r = Regex::new(concat!(
        r"^Sensor at x=(-?\d+), y=(-?\d+): ",
        r"closest beacon is at x=(-?\d+), y=(-?\d+)$"
    ))
    .unwrap();

    for line in lines {
        let captures = r.captures(line).unwrap();
        let sx: i32 = captures[1].parse().unwrap();
        let sy: i32 = captures[2].parse().unwrap();
        let bx: i32 = captures[3].parse().unwrap();
        let by: i32 = captures[4].parse().unwrap();

        // we'll need to subtract all of the beacons that might be on the row
        // that we're interested in from the final total as they obviously
        // don't count as "positions that can't contain a beacon"
        if by == row {
            beacons.insert((bx, by));
        }

        let md = (sx - bx).abs() + (sy - by).abs();

        for i in 0..md + 1 {
            let start_x = sx - md + i;
            let end_x = sx + md - i;
            let up_y = sy + i;
            let down_y = sy - i;

            if up_y == row {
                let up_ranges = rows.entry(up_y).or_default();
                up_ranges.push((start_x, end_x));
            }

            if i != 0 && down_y == row {
                let down_ranges = rows.entry(down_y).or_default();
                down_ranges.push((start_x, end_x));
            }
        }
    }

    let row = rows.get(&row).unwrap();
    let mut matches: HashSet<i32> = HashSet::new();
    for r in row {
        let (start, end) = r;
        for x in *start..*end + 1 {
            matches.insert(x);
        }
    }

    matches.len() as u32 - beacons.len() as u32
}

/// The solution for part two of the day fifteen challenge.
///
/// I originally solved this challenge by checking the perimeter of each
/// sensor (i.e., manhattan distance to beacon/range plus one) to see if it
/// was in range of any other sensors. If it was not, then we found the free
/// space and the missing beacon. However, after trying to solve a test that
/// would only fail occasionally I found a different method based on this
/// [comment](https://www.reddit.com/r/adventofcode/comments/zmcn64/comment/j0b90nr/)
/// on the Advent of Code subreddit. The strategy is to instead compute the
/// lines that surround the perimeter of each sensor then we can compute the
/// intersection and see if these reduced number of points to check is free
/// from any other scanners (just as I had implemented it before). As before,
/// if the intersection is outside of the range of all of the other sensors
/// then we've found the solution.
///
/// Given the input as a string and the upper bound of the search box as
/// function parameters we can start by parsing the input as in part one to
/// locate each of our sensors and their range based on the coordinates of the
/// beacon that they detect. The range, as before, is computed as the
/// Manhattan distance from the sensor to its beacon. Then we can compute the
/// four bounding diagonals. Then we can loop through each set of diagonals
/// and compute the intersection. If the intersection is within bounds then we
/// simply check it against the every sensor to see if its in range. Once we
/// have found an intersection that's _not_ in range then we're done and can
/// compute the answer based on the challenge prompt: the value of the x
/// coordinate times 4,000,000 plus the value of the y coordinate.
/// +
///
/// # Example
/// ```rust
/// # use aoc::y22d15::y22d15p2;
/// // probably read the from the input file...
/// let input = concat!(
///     "Sensor at x=0, y=0: closest beacon is at x=-2, y=0\n",
///     "Sensor at x=4, y=0: closest beacon is at x=6, y=0\n",
///     "Sensor at x=2, y=3: closest beacon is at x=0, y=3\n",
///     "Sensor at x=3, y=4: closest beacon is at x=3, y=6\n",
///     "Sensor at x=7, y=2: closest beacon is at x=9, y=2\n",
///     "Sensor at x=6, y=6: closest beacon is at x=6, y=8\n",
///     "Sensor at x=0, y=6: closest beacon is at x=0, y=8\n",
/// );
/// assert_eq!(y22d15p2(input, 6), 20000003);
/// ```
pub fn y22d15p2(input: &str, max: i32) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let r = Regex::new(concat!(
        r"^Sensor at x=(-?\d+), y=(-?\d+): ",
        r"closest beacon is at x=(-?\d+), y=(-?\d+)$"
    ))
    .unwrap();

    let mut sensors = HashMap::new();

    for line in lines {
        let captures = r.captures(line).unwrap();
        let sx: i32 = captures[1].parse().unwrap();
        let sy: i32 = captures[2].parse().unwrap();
        let bx: i32 = captures[3].parse().unwrap();
        let by: i32 = captures[4].parse().unwrap();

        let md = (sx - bx).abs() + (sy - by).abs();
        sensors.insert((sx, sy), md);
    }

    let mut a_diagonals = HashSet::new();
    let mut b_diagonals = HashSet::new();

    for ((x, y), range) in &sensors {
        a_diagonals.insert(y - x + range + 1);
        a_diagonals.insert(y - x - range - 1);
        b_diagonals.insert(x + y + range + 1);
        b_diagonals.insert(x + y - range - 1);
    }

    for a in &a_diagonals {
        for b in &b_diagonals {
            let intersection = (
                (((b - a) as f32) / 2.0).floor() as i32,
                (((a + b) as f32) / 2.0).floor() as i32,
            );
            let (x, y) = intersection;

            // only consider points that are in bounds
            if 0 <= x && x <= max && 0 <= y && y <= max {
                let mut in_range = false;
                for ((sx, sy), range) in &sensors {
                    let md = (x - *sx).abs() + (y - *sy).abs();
                    if md <= *range {
                        // this intersection is in-range of a sensor, we can
                        // stop checking and move on to the next one
                        in_range = true;
                        break;
                    }
                }

                if !in_range {
                    // we checked all of the sensors and were not in range of
                    // any of them -- we can return the solution!
                    return (x as i64 * 4000000 + y as i64).try_into().unwrap();
                }
            }
        }
    }

    // we shouldn't get here...
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = concat!(
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n",
            "Sensor at x=9, y=16: closest beacon is at x=10, y=16\n",
            "Sensor at x=13, y=2: closest beacon is at x=15, y=3\n",
            "Sensor at x=12, y=14: closest beacon is at x=10, y=16\n",
            "Sensor at x=10, y=20: closest beacon is at x=10, y=16\n",
            "Sensor at x=14, y=17: closest beacon is at x=10, y=16\n",
            "Sensor at x=8, y=7: closest beacon is at x=2, y=10\n",
            "Sensor at x=2, y=0: closest beacon is at x=2, y=10\n",
            "Sensor at x=0, y=11: closest beacon is at x=2, y=10\n",
            "Sensor at x=20, y=14: closest beacon is at x=25, y=17\n",
            "Sensor at x=17, y=20: closest beacon is at x=21, y=22\n",
            "Sensor at x=16, y=7: closest beacon is at x=15, y=3\n",
            "Sensor at x=14, y=3: closest beacon is at x=15, y=3\n",
            "Sensor at x=20, y=1: closest beacon is at x=15, y=3\n",
        );

        assert_eq!(y22d15p1(input, 10), 26);
        assert_eq!(y22d15p2(input, 20), 56000011);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2022/day15.txt").unwrap();

        assert_eq!(y22d15p1(&contents, 2000000), 4717631);
        assert_eq!(y22d15p2(&contents, 4000000), 13197439355220);
    }
}
