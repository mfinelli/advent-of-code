/* Copyright 2022 Mario Finelli
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
//! TODO

use regex::Regex;
use std::collections::{HashMap, HashSet};

/// The solution for part one of the day fifteen challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y22d15::y22d15p1;
/// ```
pub fn y22d15p1(input: &str, row: i32) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut rows: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    let mut beacons = HashSet::new();
    let r = Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();

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
            let start_x = sx - md + i as i32;
            let end_x = sx + md - i as i32;
            let up_y = sy + i as i32;
            let down_y = sy - i as i32;

            if up_y == row {
                let mut up_ranges = rows.entry(up_y).or_insert(Vec::new());
                up_ranges.push((start_x, end_x));
            }

            if i != 0 && down_y == row {
                let mut down_ranges = rows.entry(down_y).or_insert(Vec::new());
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
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y22d15::y22d15p2;
/// ```
pub fn y22d15p2(input: &str, max: i32) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let r = Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();

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

    for ((sx, sy), md) in &sensors {
        // use manhattan distance plus two to account for being one _larger_
        // than the border (plus one to make it inclusive, plus one for the
        // larger size)
        for i in 0..md + 2 {
            let start_x = sx - md + i as i32;
            let end_x = sx + md - i as i32;
            let up_y = sy + i as i32;
            let down_y = sy - i as i32;

            let checks: [(i32, i32); 4] = [
                (start_x - 1, up_y),
                (end_x + 1, up_y),
                (start_x - 1, down_y),
                (end_x + 1, down_y),
            ];

            for (x, y) in checks {
                if x < 0 || x > max || y < 0 || y > max {
                    continue;
                }

                let mut in_range = false;
                for ((ox, oy), omd) in &sensors {
                    if ox == sx && oy == sy {
                        continue;
                    }

                    if (ox - x).abs() + (y - oy).abs() < *omd {
                        in_range = true;
                        break;
                    }
                }

                if !in_range {
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
        // TODO: there's an ordering problem here somewhere... this test
        // occasionally fails
        assert_eq!(y22d15p2(input, 20), 56000011);
    }

    #[test]
    #[ignore]
    fn the_solution() {
        let contents = fs::read_to_string("input/2022/day15.txt").unwrap();

        assert_eq!(y22d15p1(&contents, 2000000), 4717631);
        assert_eq!(y22d15p2(&contents, 4000000), 13197439355220);
    }
}
