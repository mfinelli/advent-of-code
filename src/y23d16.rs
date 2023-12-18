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

//! Advent of Code 2023 Day 16: <https://adventofcode.com/2023/day/16>
//!
//! Today's challenge wasn't very difficult conceptually, but was fairly
//! challenging to implement. I originally implemented a recursive solution
//! but it was too slow on the actual puzzle input (it recursed on each
//! individual tile) so I switched to a queuing implementation. It's not super
//! fast for the second part (where we need to try all of the possible starting
//! points and directions) but it's not so slow that it should be thrown away.

use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

/// The solution for the day sixteen challenge.
///
/// As usual we take the input as a string and an integer to determine the
/// part. We start by parsing the input into a [`std::collections::HashMap`]
/// grid. Then for part one we only have a single starting point so we can
/// return the result immediately from the solver function. For part two we
/// instead keep track of the energized tiles from each starting point and
/// direction in a [`std::collections::BinaryHeap`] and then simply return the
/// largest one by popping it after we've tried all of the combinations.
///
/// # Example
/// ```rust
/// # use aoc::y23d16::y23d16;
/// // probably read this from the input file...
/// let input = concat!(
///     "......|...\\..\\...\n",
///     "..../........|...\n",
///     "....\\.-.../......\n",
///     "......|....../...\n",
///     ".................",
/// );
/// assert_eq!(y23d16(input, 1), 41);
/// assert_eq!(y23d16(input, 2), 41);
/// ```
pub fn y23d16(input: &str, part: u32) -> u32 {
    let mut grid = HashMap::new();
    let lines: Vec<_> = input.lines().collect();
    let rows = lines.len();
    let mut heap = BinaryHeap::new();
    let mut cols = 0;

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if y == 0 {
                cols = x;
            }

            grid.insert((x, y), c);
        }
    }

    cols += 1;

    if part == 1 {
        return send_light((0, 0), 'R', &grid, rows, cols);
    }

    for x in 0..cols {
        heap.push(send_light((x, 0), 'D', &grid, rows, cols));
        heap.push(send_light((x, rows - 1), 'U', &grid, rows, cols));
    }

    for y in 0..rows {
        heap.push(send_light((0, y), 'R', &grid, rows, cols));
        heap.push(send_light((cols - 1, y), 'L', &grid, rows, cols));
    }

    heap.pop().unwrap()
}

/// This function is responsible for actually calculating the energized tiles
/// given a starting point and direction and the grid and details. It works by
/// using a [`std::collections::VecDeque`] to keep track of the tiles that it
/// still needs to visit while maintaining a [`std::collections::HashSet`] of
/// energized tiles and a vector of tiles that it has already visited (in order
/// to avoid infinite loops).
fn send_light(
    start: (usize, usize),
    direction: char,
    grid: &HashMap<(usize, usize), char>,
    rows: usize,
    cols: usize,
) -> u32 {
    let mut energized = HashSet::new();
    let mut queue = VecDeque::new();
    let mut visited = Vec::new();

    queue.push_back((start, direction));

    loop {
        match queue.pop_front() {
            None => break,
            Some(((x, y), direction)) => {
                let tile = grid.get(&(x, y)).unwrap();
                energized.insert((x, y));

                if x < cols - 1
                    && !visited.contains(&((x + 1, y), 'R'))
                    && ((direction == 'R' && (*tile == '.' || *tile == '-'))
                        || (direction == 'U' && (*tile == '/' || *tile == '-'))
                        || (direction == 'D'
                            && (*tile == '\\' || *tile == '-')))
                {
                    queue.push_back(((x + 1, y), 'R'));
                    visited.push(((x + 1, y), 'R'));
                }

                if y > 0
                    && !visited.contains(&((x, y - 1), 'U'))
                    && ((direction == 'U' && (*tile == '.' || *tile == '|'))
                        || (direction == 'R' && (*tile == '/' || *tile == '|'))
                        || (direction == 'L'
                            && (*tile == '\\' || *tile == '|')))
                {
                    queue.push_back(((x, y - 1), 'U'));
                    visited.push(((x, y - 1), 'U'));
                }

                if x > 0
                    && !visited.contains(&((x - 1, y), 'L'))
                    && ((direction == 'L' && (*tile == '.' || *tile == '-'))
                        || (direction == 'U'
                            && (*tile == '\\' || *tile == '-'))
                        || (direction == 'D' && (*tile == '/' || *tile == '-')))
                {
                    queue.push_back(((x - 1, y), 'L'));
                    visited.push(((x - 1, y), 'L'));
                }

                if y < rows - 1
                    && !visited.contains(&((x, y + 1), 'D'))
                    && ((direction == 'D' && (*tile == '.' || *tile == '|'))
                        || (direction == 'R'
                            && (*tile == '\\' || *tile == '|'))
                        || (direction == 'L' && (*tile == '/' || *tile == '|')))
                {
                    queue.push_back(((x, y + 1), 'D'));
                    visited.push(((x, y + 1), 'D'));
                }
            }
        }
    }

    energized.len().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_send_light() {
        // .|..-\.
        // /......
        // .-..|..
        // \.../\/
        let grid = HashMap::from([
            ((0, 0), '.'),
            ((1, 0), '|'),
            ((2, 0), '.'),
            ((3, 0), '.'),
            ((4, 0), '-'),
            ((5, 0), '\\'),
            ((6, 0), '.'),
            ((0, 1), '/'),
            ((1, 1), '.'),
            ((2, 1), '.'),
            ((3, 1), '.'),
            ((4, 1), '.'),
            ((5, 1), '.'),
            ((6, 1), '.'),
            ((0, 2), '.'),
            ((1, 2), '-'),
            ((2, 2), '.'),
            ((3, 2), '.'),
            ((4, 2), '|'),
            ((5, 2), '.'),
            ((6, 2), '.'),
            ((0, 3), '\\'),
            ((1, 3), '.'),
            ((2, 3), '.'),
            ((3, 3), '.'),
            ((4, 3), '/'),
            ((5, 3), '\\'),
            ((6, 3), '/'),
        ]);

        assert_eq!(send_light((0, 0), 'R', &grid, 4, 7), 28);
    }

    #[test]
    fn it_works() {
        let input = concat!(
            ".|...\\....\n",
            "|.-.\\.....\n",
            ".....|-...\n",
            "........|.\n",
            "..........\n",
            ".........\\\n",
            "..../.\\\\..\n",
            ".-.-/..|..\n",
            ".|....-|.\\\n",
            "..//.|....\n",
        );

        assert_eq!(y23d16(input, 1), 46);
        assert_eq!(y23d16(input, 2), 51);
    }

    #[test]
    #[ignore]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day16.txt").unwrap();

        assert_eq!(y23d16(&contents, 1), 7236);
        assert_eq!(y23d16(&contents, 2), 7521);
    }
}
