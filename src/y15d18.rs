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
//! TODO

use std::collections::HashMap;

/// The solution for the day eighteen challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y15d18::y15d18;
/// // probably read this from the input file...
/// let input = ".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..";
/// assert_eq!(y15d18(input, 1, 1), 11);
/// ```
pub fn y15d18(input: &str, steps: u32, part: u32) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let size = lines.len();
    let mut grid = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, light) in line.chars().enumerate() {
            grid.insert((x as i32,y as i32), light == '#');
        }
    }

    if part == 2 {
        // in part two the corners are always on
        grid.insert((0, 0), true);
        grid.insert((size as i32 -1, 0), true);
        grid.insert((0, size as i32 -1), true);
        grid.insert((size as i32 -1, size as i32 -1), true);
    }

    for _ in 0..steps {
        print_grid(&grid, size);
        let mut newgrid = HashMap::new();

        for ((x,y), light) in &grid {
            if part == 2 {
                if (*x == 0 && *y == 0) || (*x == size as i32 -1 && *y == 0) || (*x == 0 && *y == size as i32 -1) || (*x == size as i32 -1 && *y == size as i32 -1){
                    continue;
                }
            }

            let neighbors = vec![(*x-1, *y-1), (*x, *y-1), (*x+1,*y-1),(*x+1,*y),(*x+1,*y+1),(*x,*y+1),(*x-1,*y+1),(*x-1,*y)];
            let mut on = 0;

            // if *x == 0 && *y == 0 {
            for neighbor in neighbors {
                if let Some(a) = grid.get(&neighbor) {
                    if *a {
                    // println!("{:?}", a);
                    on += 1;
                    }
                }
            }
            // println!("{:?}", on);
            // }

            // println!("step: {} - ({}, {}): {:?}", s, x, y, on);

            if *light {
                if on == 2 || on == 3 {
                    newgrid.insert((*x,*y), true);
                } else {
                    newgrid.insert((*x,*y), false);
                }
            } else {
                if on == 3 {
                    newgrid.insert((*x,*y), true);
                } else {
                    newgrid.insert((*x,*y), false);
                }
            }
        }

        if part == 2 {
            // in part two the corners are always on
            newgrid.insert((0, 0), true);
            newgrid.insert((size as i32 -1, 0), true);
            newgrid.insert((0, size as i32 -1), true);
            newgrid.insert((size as i32 -1, size as i32 -1), true);
        }

        grid = newgrid;
    }

    grid.retain(|_, x| *x); // retiain lights that are on
    grid.len().try_into().unwrap()
    // 0
}

fn print_grid(grid: &HashMap<(i32,i32), bool>, size: usize) {
    // println!("{:?}", grid);
    let mut s = "".to_string();
    for y in 0..size {
        for x in 0..size {
            if *grid.get(&(x as i32, y as i32)).unwrap() {
                s = format!("{}{}", s, '#');
            } else {
                s = format!("{}{}", s, '.');
            }
        }
        s = format!("{}{}", s, "\n");
    }

    println!("{}", s);
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
        // assert_eq!(y15d18(input, 2, 2), 17);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day18.txt").unwrap();

        assert_eq!(y15d18(&contents, 100, 1), 1061);
        assert_eq!(y15d18(&contents, 100, 2), 1006);
    }
}
