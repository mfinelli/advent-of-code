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

//! Advent of Code 2023 Day 10: <https://adventofcode.com/2023/day/10>
//!
//! TODO

use std::collections::HashMap;

/// The solution for the day ten challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y23d10::y23d10;
/// // probably read this from the input file...
/// let input = "";
/// assert_eq!(y23d10(input), 0);
/// ```
pub fn y23d10(input: &str) -> u32 {
    let mut grid: HashMap<(i32, i32), char> = HashMap::new();
    let mut start = None;

    for (y, line) in input.lines().enumerate() {
        let y: i32 = y.try_into().unwrap();

        for (x, pipe) in line.chars().enumerate() {
            let x: i32 = x.try_into().unwrap();
            grid.insert((x, y), pipe);

            if pipe == 'S' {
                start = Some((x, y));
            }
        }
    }

    let start = start.unwrap();

    // println!("{:?}", grid);
    // println!("{:?}", start);

    // let mut count = 0;
    let path_starts = vec![(start.0-1, start.1), (start.0+1, start.1), (start.0, start.1-1), (start.0,start.1+1)];

    for path_start in path_starts {
        let mut current = path_start;
        let mut count = 0;

        let mut dir = if current.0 == start.0 && current.1 == start.1-1 {
            'U'
        } else if current.0 == start.0 && current.1 == start.1+1 {
            'D'
        } else if current.0 == start.0-1 && current.1 == start.1 {
            'L'
        } else if current.0 == start.0+1 && current.1 == start.1 {
            'R'
        } else {
            panic!("invalid starting direction")
        };

        // println!("starting path check: {:?}, direction: {}", current, dir);

        loop {
            match grid.get(&current) {
                None => break,
                Some(pipe) => {
                    count += 1;
                    // println!("checking: {:?}, dir: {}", pipe, dir);

                    match pipe {
                        '.' => break,
                        'S' => return count/2,
                        '|' => {
                            if dir == 'U' {
                                current = (current.0, current.1-1);
                            } else if dir == 'D' {
                                current = (current.0, current.1+1);
                            } else {
                                // panic!("wrong direction");
                                break;
                            }
                        }
                        '-' => {
                            if dir == 'L' {
                                current = (current.0-1, current.1);
                            } else if dir == 'R' {
                                current = (current.0+1, current.1);
                            } else {
                                // panic!("wrong direction");
                                break;
                            }
                        }
                        'L' => {
                            if dir == 'L' {
                                dir = 'U';
                                current = (current.0, current.1-1);
                            } else if dir == 'D' {
                                dir = 'R';
                                current = (current.0+1, current.1);
                            } else {
                                // panic!("wrong direction");
                                break;
                            }
                        }
                        'J' => {
                            if dir == 'R' {
                                dir = 'U';
                                current = (current.0, current.1-1);
                            } else if dir == 'D' {
                                dir = 'L';
                                current = (current.0-1, current.1);
                            } else {
                                // panic!("wrong direction");
                                break;
                            }
                        }
                        '7' => {
                            if dir == 'R' {
                                dir = 'D';
                                current = (current.0, current.1+1);
                            } else if dir == 'U' {
                                dir = 'L';
                                current = (current.0-1,current.1);
                            } else {
                                // panic!("wrong direction");
                                break;
                            }
                        }
                        'F' => {
                            if dir == 'U' {
                                dir = 'R';
                                current = (current.0+1, current.1);
                            } else if dir == 'L' {
                                dir = 'D';
                                current = (current.0, current.1+1);
                            } else {
                                // panic!("wrong direction");
                                break;
                            }
                        }
                        _ => panic!("unrecognized pipe"),
                    };

                    // println!("new current: {:?}, new dir: {}", current, dir);
                }
            };
        }
    }

    // count
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let mut input = "-L|F7\n7S-7|\nL|7||\n-L-J|\nL|-JF\n";
        assert_eq!(y23d10(input), 4);

        input = "7-F7-\n.FJ|7\nSJLL7\n|F--J\nLJ.LJ\n";
        assert_eq!(y23d10(input), 8);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day10.txt").unwrap();

        assert_eq!(y23d10(&contents), 0);
    }
}
