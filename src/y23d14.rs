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

//! Advent of Code 2023 Day 14: <https://adventofcode.com/2023/day/14>
//!
//! Today's challenge was not terribly difficult. After a relatively easy part
//! one, part two is a classic Advent of Code problem where in order to avoid
//! needing to run one billion (expensive) iterations of a loop we can instead
//! find a cycle that we are sure that there will be because of the way that
//! the input is crafted.

use std::collections::HashMap;

/// The solution for the day fourteen challenge.
///
/// We take the input as a string and an integer for the part which changes
/// whether we do a single tilt or 1,000,000,000 cycles of tilt north, west,
/// south, and then east. We start by parsing the input into a
/// [`std::collections::HashMap`] grid of, cube-shaped rocks, rounded rocks,
/// and empty spaces. Then, in part one we just run a single north tilt and
/// then count the load. In part two we do one billion cycles but we keep track
/// of the end states that we've seen and at which count in the cycle so that
/// when we find the loop, we can short-circuit the loop and then we count the
/// load.
///
/// # Example
/// ```rust
/// # use aoc::y23d14::y23d14;
/// // probably read this from the input file...
/// let input = "...\n.#.\nO..";
/// assert_eq!(y23d14(input, 1), 3);
/// assert_eq!(y23d14(input, 2), 1);
/// ```
pub fn y23d14(input: &str, part: u32) -> i32 {
    let mut total = 0;
    let mut map = HashMap::new();
    let mut seen = HashMap::new();

    let lines: Vec<_> = input.lines().collect();
    let rows: i32 = lines.len().try_into().unwrap();
    let mut cols: i32 = 0;

    for (y, line) in lines.iter().enumerate() {
        let y: i32 = y.try_into().unwrap();

        for (x, c) in line.chars().enumerate() {
            if y == 0 {
                cols = x.try_into().unwrap();
            }

            let x: i32 = x.try_into().unwrap();

            map.insert((x, y), c);
        }
    }

    cols += 1;

    if part == 1 {
        tilt('N', rows, cols, &mut map);
    } else {
        let cycles = 1000000000;
        for cycle in 1..cycles + 1 {
            tilt('N', rows, cols, &mut map);
            tilt('W', rows, cols, &mut map);
            tilt('S', rows, cols, &mut map);
            tilt('E', rows, cols, &mut map);

            let grid = map_to_string(rows, cols, &map);
            if let Some(seen_at) = seen.insert(grid, cycle) {
                if (cycles - cycle) % (cycle - seen_at) == 0 {
                    break;
                }
            }
        }
    }

    for ((_, y), c) in map {
        if c == 'O' {
            total += rows - y;
        }
    }

    total
}

/// This function is responsible for modifying given map to its state after
/// applying a tilt in the desired direction. It basically works by going
/// row-by-row or column-by-column depending on the direction and then checking
/// each space until we find a boulder and then attempting to move it as far
/// as possible (either until we reach an edge, or another boulder) based on
/// what is currently in its adjacent tile.
fn tilt(dir: char, rows: i32, cols: i32, map: &mut HashMap<(i32, i32), char>) {
    if dir == 'N' {
        for x in 0..cols {
            for y in 0..rows {
                let c = map.get(&(x, y)).unwrap();

                if *c == 'O' {
                    let mut current = y;

                    loop {
                        match map.get(&(x, current - 1)) {
                            None => break,
                            Some(above) => {
                                if *above == 'O' || *above == '#' {
                                    break;
                                }

                                map.insert((x, current - 1), 'O');
                                map.insert((x, current), '.');

                                current -= 1;
                            }
                        }
                    }
                }
            }
        }
    } else if dir == 'S' {
        for x in 0..cols {
            for y in 0..rows {
                let y = rows - 1 - y;
                let c = map.get(&(x, y)).unwrap();

                if *c == 'O' {
                    let mut current = y;

                    loop {
                        match map.get(&(x, current + 1)) {
                            None => break,
                            Some(below) => {
                                if *below == 'O' || *below == '#' {
                                    break;
                                }

                                map.insert((x, current + 1), 'O');
                                map.insert((x, current), '.');

                                current += 1;
                            }
                        }
                    }
                }
            }
        }
    } else if dir == 'E' {
        for y in 0..rows {
            for x in 0..cols {
                let x = cols - 1 - x;
                let c = map.get(&(x, y)).unwrap();

                if *c == 'O' {
                    let mut current = x;

                    loop {
                        match map.get(&(current + 1, y)) {
                            None => break,
                            Some(right) => {
                                if *right == 'O' || *right == '#' {
                                    break;
                                }

                                map.insert((current + 1, y), 'O');
                                map.insert((current, y), '.');

                                current += 1;
                            }
                        }
                    }
                }
            }
        }
    } else {
        for y in 0..rows {
            for x in 0..cols {
                let c = map.get(&(x, y)).unwrap();

                if *c == 'O' {
                    let mut current = x;

                    loop {
                        match map.get(&(current - 1, y)) {
                            None => break,
                            Some(left) => {
                                if *left == 'O' || *left == '#' {
                                    break;
                                }

                                map.insert((current - 1, y), 'O');
                                map.insert((current, y), '.');

                                current -= 1;
                            }
                        }
                    }
                }
            }
        }
    }
}

/// This function returns the grid as its string representation which is useful
/// for debugging but also used to store in our list of seen states when we're
/// trying to find a cycle.
fn map_to_string(
    rows: i32,
    cols: i32,
    map: &HashMap<(i32, i32), char>,
) -> String {
    let mut s = "".to_string();

    for y in 0..rows {
        for x in 0..cols {
            s = format!("{}{}", s, map.get(&(x, y)).unwrap());
        }

        s = format!("{}\n", s);
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_map_to_string() {
        let grid = HashMap::from([
            ((0, 0), '.'),
            ((1, 0), '#'),
            ((2, 0), '#'),
            ((0, 1), '.'),
            ((1, 1), 'O'),
            ((2, 1), '.'),
            ((0, 2), '.'),
            ((1, 2), 'O'),
            ((2, 2), '.'),
        ]);
        assert_eq!(map_to_string(3, 3, &grid), ".##\n.O.\n.O.\n");
    }

    #[test]
    fn test_tilt() {
        let mut grid = HashMap::from([
            ((0, 0), '.'),
            ((1, 0), '#'),
            ((2, 0), '#'),
            ((0, 1), '.'),
            ((1, 1), '.'),
            ((2, 1), 'O'),
            ((0, 2), 'O'),
            ((1, 2), '.'),
            ((2, 2), '.'),
        ]);

        tilt('N', 3, 3, &mut grid);

        assert_eq!(*grid.get(&(0, 0)).unwrap(), 'O');
        assert_eq!(*grid.get(&(1, 0)).unwrap(), '#');
        assert_eq!(*grid.get(&(2, 0)).unwrap(), '#');
        assert_eq!(*grid.get(&(0, 1)).unwrap(), '.');
        assert_eq!(*grid.get(&(1, 1)).unwrap(), '.');
        assert_eq!(*grid.get(&(2, 1)).unwrap(), 'O');
        assert_eq!(*grid.get(&(0, 2)).unwrap(), '.');
        assert_eq!(*grid.get(&(1, 2)).unwrap(), '.');
        assert_eq!(*grid.get(&(2, 2)).unwrap(), '.');
    }

    #[test]
    fn it_works() {
        let input = concat!(
            "O....#....\n",
            "O.OO#....#\n",
            ".....##...\n",
            "OO.#O....O\n",
            ".O.....O#.\n",
            "O.#..O.#.#\n",
            "..O..#O..O\n",
            ".......O..\n",
            "#....###..\n",
            "#OO..#....\n",
        );

        assert_eq!(y23d14(input, 1), 136);
        assert_eq!(y23d14(input, 2), 64);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day14.txt").unwrap();

        assert_eq!(y23d14(&contents, 1), 109098);
        assert_eq!(y23d14(&contents, 2), 100064);
    }
}
