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
/// assert_eq!(y24d06(input), 0);
/// ```
pub fn y24d06(input: &str) -> usize {
    let mut obstacles = Vec::new();
    let mut guard = (0,0);
    let mut dir = 'N';
    let max_y;
    let mut max_x = 0;
    let mut visited = HashSet::new();

    let lines: Vec<_> = input.lines().collect();
    max_y = lines.len();

    for (y, line) in input.lines().enumerate() {
        let chars: Vec<_> = line.chars().collect();

        if y == 0{
            max_x = chars.len();
        }

        for (x, c) in chars.iter().enumerate() {
            if *c == '#' {
                obstacles.push((x, y));
            }

            if *c == '^' {
                guard = (x, y);
                visited.insert((x, y));
            }
        }
    }

    // println!("{:?}", obstacles);
    // let mut i = 0;
    loop {
        // if i == 40 {
        //     break;
        // }

        let (x, y) = guard;
        visited.insert((x, y));
        // println!("guard: {},{} {}", x, y, dir);

        if dir == 'N' {
            if y == 0 {
                break;
            }

            if obstacles.contains(&(x, y-1)) {
                dir = 'E';
            } else {
                guard = (x, y-1);
            }
        } else if dir == 'E'{
            if x + 1 == max_x {
                break;
            }

            if obstacles.contains(&(x+1, y)) {
                dir = 'S';
            } else {
                guard = (x+1, y);
            }
        } else if dir == 'S'{
            if y+1 == max_y {
                break;
            }

            if obstacles.contains(&(x, y+1)) {
                dir = 'W';
            } else {
                guard = (x, y+1);
            }
        } else if dir == 'W' {
            if x == 0 {
                break;
            }

            if obstacles.contains(&(x-1, y)) {
                dir = 'N';
            } else {
                guard = (x-1, y);
            }
        }

        // i += 1;
    }


    visited.len()
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
        assert_eq!(y24d06(input), 41);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2024/day06.txt").unwrap();

        assert_eq!(y24d06(&contents), 5329);
    }
}
