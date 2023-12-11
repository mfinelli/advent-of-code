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
//! I originally wasn't sure how to handle this problem, and I considered
//! building a large grid to hold all of the galaxies (and the empty spaces)
//! but I was stuck on how to properly expand it for the empty rows/columns.
//! Part two would have then probably made the memory requirements outrageous
//! adding millions and millions of entries. Inspiration from the Advent of
//! Code subreddit instead led me down the path of instead just finding the
//! empty rows and columns and then adding them to the distance instead of
//! expanding the grid (or trying to modify the galaxy coordinates) and then
//! computing the Manhattan distance afterwards.

use itertools::Itertools;

/// The solution for the day eleven challenge.
///
/// We start with the input as a string and then the second parameter is for
/// how much the empty spaces are supposed to expand (which allows us to easily
/// test the answers given in the prompt's example as opposed to the usual
/// part one or two). We start by parsing the input to find the coordinates of
/// all of the galaxies. We also initialize vectors for the empty rows and
/// columns and fill them with all of the rows and columns to start. After
/// finding all of the galaxies we do a single pass over them to eliminate
/// their x and y coordinates from the lists of empty columns and rows. Then,
/// we generate the combinations for each galaxy to every other galaxy and
/// calculate the Manhattan distance. Then we generate ranges for the x any
/// y coordinates from one galaxy to the other and check to see if we pass over
/// any of the empty rows or columns. For each that we pass over we add our
/// modifier parameter. Then, we add the distance to our running sum, which we
/// return after processing all of the galaxies.
///
/// # Example
/// ```rust
/// # use aoc::y23d11::y23d11;
/// // probably read this from the input file...
/// let input = ".#...\n.....\n..#.#\n..#..\n#....";
/// assert_eq!(y23d11(input, 50), 428);
/// ```
pub fn y23d11(input: &str, modifier: i64) -> i64 {
    let mut galaxies: Vec<(i64, i64)> = Vec::new();
    let mut empty_rows: Vec<i64> = Vec::new();
    let mut empty_cols: Vec<i64> = Vec::new();
    let mut sum = 0;

    let modifier = if modifier == 1 { 1 } else { modifier - 1 };

    for (y, line) in input.lines().enumerate() {
        empty_rows.push(y.try_into().unwrap());

        for (x, c) in line.chars().enumerate() {
            if y == 0 {
                empty_cols.push(x.try_into().unwrap());
            }

            if c == '#' {
                galaxies.push((x.try_into().unwrap(), y.try_into().unwrap()));
            }
        }
    }

    for galaxy in &galaxies {
        let (x, y) = galaxy;

        empty_rows.retain(|r| r != y);
        empty_cols.retain(|c| c != x);
    }

    for combination in galaxies.iter().combinations(2) {
        let (ax, ay) = combination[0];
        let (bx, by) = combination[1];
        let mut md = (ax - bx).abs() + (ay - by).abs();

        let xrange = if ax <= bx { ax..bx } else { bx..ax };
        let yrange = if ay <= by { ay..by } else { by..ay };

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

        sum += md;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
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
