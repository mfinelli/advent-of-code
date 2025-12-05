/* Copyright 2025 Mario Finelli
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

//! Advent of Code 2025 Day 4: <https://adventofcode.com/2025/day/4>
//!
//! TODO

// use std::collections::HashMap;

/// The solution for the day four challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y25d04::y25d04;
/// // probably read this from the input file...
/// let input = concat!(
/// );
/// assert_eq!(y25d04(input), 1);
/// ```
pub fn y25d04(input: &str) -> u64 {
    let mut sum = 0;
    let mut grid = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, val) in line.chars().enumerate() {
            if val == '@' {
                grid.push((x, y));
            }
        }
    }

    for (x, y) in &grid {
        let mut adjacent = 0;

        if *x != 0 && *y != 0 && grid.contains(&(x-1, y-1)) { // upper left
            adjacent += 1;
        }

        if *y != 0 && grid.contains(&(*x, y-1)) { // above
            adjacent += 1;
        }

        if *y != 0 && grid.contains(&(x+1, y-1)) { // upper right
            adjacent += 1;
        }

        if grid.contains(&(x+1, *y)) { // right
            adjacent += 1;
        }

        if grid.contains(&(x+1, y+1)) { // lower right
            adjacent += 1;
        }

        if grid.contains(&(*x, y+1)) { // below
            adjacent += 1;
        }

        if *x != 0 && grid.contains(&(x-1, y+1)) { // lower left
            adjacent += 1;
        }

        if *x != 0 && grid.contains(&(x-1, *y)) { // left
            adjacent += 1;
        }

        if adjacent < 4 {
            sum += 1;
        }
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
            "..@@.@@@@.\n",
            "@@@.@.@.@@\n",
            "@@@@@.@.@@\n",
            "@.@@@@..@.\n",
            "@@.@@@@.@@\n",
            ".@@@@@@@.@\n",
            ".@.@.@.@@@\n",
            "@.@@@.@@@@\n",
            ".@@@@@@@@.\n",
            "@.@.@@@.@.\n",
        );

        assert_eq!(y25d04(input), 13);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2025/day04.txt").unwrap();

        // assert_eq!(y25d04(&contents), 1);
    }
}
