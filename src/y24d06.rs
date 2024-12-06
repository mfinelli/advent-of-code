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

//! Advent of Code 2024 Day 6: <https://adventofcode.com/2024/day/6>
//!
//! TODO

use std::collections::HashSet;

/// The solution for the day five challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y24d06::y24d06;
/// // probably read this from the input file...
/// let input = concat!(
/// );
/// assert_eq!(y24d06(input, 1), 0);
/// ```
pub fn y24d06(input: &str, part: u32) -> usize {
    let mut obstacles = Vec::new();
    let mut guard = (0, 0, 'N');
    let mut start = (0, 0);
    let max_y;
    let mut max_x = 0;
    let mut visited = HashSet::new();

    let lines: Vec<_> = input.lines().collect();
    max_y = lines.len();

    for (y, line) in input.lines().enumerate() {
        let chars: Vec<_> = line.chars().collect();

        if y == 0 {
            max_x = chars.len();
        }

        for (x, c) in chars.iter().enumerate() {
            if *c == '#' {
                obstacles.push((x, y));
            }

            if *c == '^' {
                start = (x, y);
                guard = (x, y, 'N');
                visited.insert((x, y));
            }
        }
    }

    loop {
        let (x, y, dir) = guard;
        visited.insert((x, y));

        if dir == 'N' {
            if y == 0 {
                break;
            }

            if obstacles.contains(&(x, y - 1)) {
                guard = (x, y, 'E');
            } else {
                guard = (x, y - 1, dir);
            }
        } else if dir == 'E' {
            if x + 1 == max_x {
                break;
            }

            if obstacles.contains(&(x + 1, y)) {
                guard = (x, y, 'S');
            } else {
                guard = (x + 1, y, dir);
            }
        } else if dir == 'S' {
            if y + 1 == max_y {
                break;
            }

            if obstacles.contains(&(x, y + 1)) {
                guard = (x, y, 'W');
            } else {
                guard = (x, y + 1, dir);
            }
        } else if dir == 'W' {
            if x == 0 {
                break;
            }

            if obstacles.contains(&(x - 1, y)) {
                guard = (x, y, 'N');
            } else {
                guard = (x - 1, y, dir);
            }
        }
    }

    if part == 1 {
        return visited.len();
    }

    let mut sum = 0;
    for loc in visited {
        if loc == start {
            continue;
        }

        let mut newobstacles = obstacles.clone();
        newobstacles.push(loc);
        let mut p2visited = Vec::new();
        guard = (start.0, start.1, 'N');

        loop {
            let (x, y, dir) = guard;
            if p2visited.contains(&guard) {
                sum += 1;
                break;
            }

            p2visited.push(guard);

            if dir == 'N' {
                if y == 0 {
                    break;
                }

                if newobstacles.contains(&(x, y - 1)) {
                    guard = (x, y, 'E');
                } else {
                    guard = (x, y - 1, dir);
                }
            } else if dir == 'E' {
                if x + 1 == max_x {
                    break;
                }

                if newobstacles.contains(&(x + 1, y)) {
                    guard = (x, y, 'S');
                } else {
                    guard = (x + 1, y, dir);
                }
            } else if dir == 'S' {
                if y + 1 == max_y {
                    break;
                }

                if newobstacles.contains(&(x, y + 1)) {
                    guard = (x, y, 'W');
                } else {
                    guard = (x, y + 1, dir);
                }
            } else if dir == 'W' {
                if x == 0 {
                    break;
                }

                if newobstacles.contains(&(x - 1, y)) {
                    guard = (x, y, 'N');
                } else {
                    guard = (x - 1, y, dir);
                }
            }
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
            "....#.....\n",
            ".........#\n",
            "..........\n",
            "..#.......\n",
            ".......#..\n",
            "..........\n",
            ".#..^.....\n",
            "........#.\n",
            "#.........\n",
            "......#...\n",
        );
        assert_eq!(y24d06(input, 1), 41);
        assert_eq!(y24d06(input, 2), 6);
    }

    #[test]
    #[ignore]
    fn the_solution() {
        let contents = fs::read_to_string("input/2024/day06.txt").unwrap();

        assert_eq!(y24d06(&contents, 1), 5329);
        assert_eq!(y24d06(&contents, 2), 2162);
    }
}
