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

//! Advent of Code 2015 Day 18: <https://adventofcode.com/2015/day/18>
//!
//! A relatively easy challenge, we essentially keep track of all of the lights
//! as points on a grid ([`std::collections::HashMap`]) and then for each step
//! evaluate the neighbors to build a new grid and then replace the current
//! grid with the new grid in order to make one atomic option. Lights on the
//! edge should have their neighbors considered off which is easy with the
//! HashMap lookup: if we don't have a neighbor because it doesn't exist in the
//! grid then don't increment the turned-on-neighbor count. In part two we
//! just continually always set the corners to on.

use std::collections::HashMap;

/// The solution for the day eighteen challenge.
///
/// We take the input as a string as usual, and then accept the number of steps
/// to process and the part of the problem that we're solving (in part `2` the
/// corners are always on). We start by parsing the input to build a grid of
/// the lights and their state. If we're in part `2` then we turn on the lights
/// in the corner regardless of the initial input. Then for each step until we
/// reach the total number of steps we build a new grid. We process all of the
/// current lights and check their neighbors to see if they are on or not (if
/// we're in part two and on a corner we just skip as we always leave the
/// corner lights on). After we've counted the number of lights that are on we
/// turn the light on or off based on the constraints from the prompt: if the
/// light is currently on and it has two or three neighbors that are on then it
/// stays on otherwise it turns off, and if it's currently off and it has three
/// neighbors that are on then it turns on otherwise it stays off. If we're in
/// part two we turn on the corner lights before replacing the current grid
/// with the new grid that we've just built and moving on to the next step.
/// Finally, we count the number of lights that are on and return the result.
///
/// # Example
/// ```rust
/// # use aoc::y15d18::y15d18;
/// // probably read this from the input file...
/// let input = ".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..";
/// assert_eq!(y15d18(input, 1, 1), 11);
/// assert_eq!(y15d18(input, 3, 2), 18);
/// ```
pub fn y15d18(input: &str, steps: u32, part: u32) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let size: i32 = lines.len().try_into().unwrap();
    let mut grid = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, light) in line.chars().enumerate() {
            grid.insert((x as i32, y as i32), light == '#');
        }
    }

    if part == 2 {
        // in part two the corners are always on
        grid.insert((0, 0), true);
        grid.insert((size - 1, 0), true);
        grid.insert((0, size - 1), true);
        grid.insert((size - 1, size - 1), true);
    }

    for _ in 0..steps {
        let mut newgrid = HashMap::new();

        for ((x, y), light) in &grid {
            if part == 2
                && ((*x == size - 1 || *x == 0) && *y == 0
                    || (*x == 0 && *y == -1)
                    || (*x == size - 1 && *y == size - 1))
            {
                continue;
            }

            let neighbors = vec![
                (*x - 1, *y - 1),
                (*x, *y - 1),
                (*x + 1, *y - 1),
                (*x + 1, *y),
                (*x + 1, *y + 1),
                (*x, *y + 1),
                (*x - 1, *y + 1),
                (*x - 1, *y),
            ];
            let mut on = 0;

            for neighbor in neighbors {
                if let Some(switch) = grid.get(&neighbor) {
                    if *switch {
                        on += 1;
                    }
                }
            }

            if *light {
                if on == 2 || on == 3 {
                    newgrid.insert((*x, *y), true);
                } else {
                    newgrid.insert((*x, *y), false);
                }
            } else if on == 3 {
                newgrid.insert((*x, *y), true);
            } else {
                newgrid.insert((*x, *y), false);
            }
        }

        if part == 2 {
            // in part two the corners are always on
            newgrid.insert((0, 0), true);
            newgrid.insert((size - 1, 0), true);
            newgrid.insert((0, size - 1), true);
            newgrid.insert((size - 1, size - 1), true);
        }

        grid = newgrid;
    }

    grid.retain(|_, x| *x); // retain lights that are on
    grid.len().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = ".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..\n";
        assert_eq!(y15d18(input, 4, 1), 4);
        assert_eq!(y15d18(input, 5, 2), 17);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day18.txt").unwrap();

        assert_eq!(y15d18(&contents, 100, 1), 1061);
        assert_eq!(y15d18(&contents, 100, 2), 1006);
    }
}
