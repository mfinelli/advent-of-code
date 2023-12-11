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

//! Advent of Code 2023 Day 11: <https://adventofcode.com/2023/day/11>
//!
//! TODO

// use std::collections::HashMap;
use itertools::Itertools;

/// The solution for the day eleven challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y23d11::y23d11;
/// // probably read this from the input file...
/// let input = "";
/// assert_eq!(y23d11(input, 1), 0);
/// assert_eq!(y23d11(input, 10), 0);
/// ```
pub fn y23d11(input: &str, modifier: i64) -> i64 {
    let mut galaxies: Vec<(i64, i64)> = Vec::new();
    // let lines: Vec<_> = input.lines().collect();
    // let size = lines.len();
    let mut empty_rows: Vec<i64> = Vec::new();
    let mut empty_cols: Vec<i64> = Vec::new();
    let mut sum = 0;

    let modifier = if modifier == 1 {
        1
    } else {
        modifier - 1
    };

    for (y, line) in input.lines().enumerate() {
        empty_rows.push(y.try_into().unwrap());

        for (x, c) in line.chars().enumerate() {
            if y == 0 {
                empty_cols.push(x.try_into().unwrap());
            }

            if c == '#' {
                galaxies.push((x.try_into().unwrap(),y.try_into().unwrap()));
            }
        }
    }

    for galaxy in &galaxies {
        let (x, y) = galaxy;

        empty_rows.retain(|r| r != y);
        empty_cols.retain(|c| c != x);
    }

    // println!("galaxies: {:?}", galaxies);
    // println!("empty rows: {:?}", empty_rows);
    // println!("empty cols: {:?}", empty_cols);

    for combination in galaxies.iter().combinations(2) {
        // println!("{:?}", combination);
        let (ax, ay) = combination[0];
        let (bx, by) = combination[1];
        let mut md = (ax - bx).abs() + (ay - by).abs();

        let xrange = if ax <= bx {
            ax..bx
        } else {
            bx..ax
        };

        let yrange = if ay <= by {
            ay..by
        } else {
            by..ay
        };

        for row in &empty_rows {
            if yrange.contains(&row) {
                md += modifier;
            }
        }

        for col in &empty_cols {
            if xrange.contains(&col) {
                md += modifier;
            }
        }

        // println!("{:?} -> {:?}: {}", combination[0], combination[1], md);
        sum += md;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn iit_works() {
        let input = concat!(
            "...#......\n",
            ".......#..\n",
            "#.........\n",
            "..........\n",
            "......#...\n",
            ".#........\n",
            ".........#\n",
            "..........\n",
            ".......#..\n",
            "#...#.....\n",
        );

        assert_eq!(y23d11(input, 1), 374);
        assert_eq!(y23d11(input, 10), 1030);
        assert_eq!(y23d11(input, 100), 8410);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day11.txt").unwrap();

        assert_eq!(y23d11(&contents, 1), 9608724);
        assert_eq!(y23d11(&contents, 1000000), 904633799472);
    }
}
