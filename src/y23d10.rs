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
/// assert_eq!(y23d10(input, 1), 0);
/// assert_eq!(y23d10(input, 2), 0);
/// ```
pub fn y23d10(input: &str, part: u32) -> u64 {
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

    let path = find_loop(&grid, start);
    if part == 1 {
        let len: u64 = path.len().try_into().unwrap();
        len / 2
    } else {
        let verticies: Vec<_> = path
            .iter()
            .filter(|p| {
                let pipe = grid.get(p).unwrap();
                // TODO: this assumes that S is a corner
                ['J', '7', 'L', 'F', 'S'].contains(pipe)
            })
            .collect();

        let area = shoelace(&verticies);
        let len = path.len() as f64;
        let points = (area * 2.0 - len + 2.0) / 2.0;
        if points.fract() != 0.0 {
            panic!("got non-whole-number");
        }

        points as u64
    }
}

fn find_loop(
    grid: &HashMap<(i32, i32), char>,
    start: (i32, i32),
) -> Vec<(i32, i32)> {
    let path_starts = vec![
        (start.0 - 1, start.1),
        (start.0 + 1, start.1),
        (start.0, start.1 - 1),
        (start.0, start.1 + 1),
    ];

    for path_start in path_starts {
        let mut current = path_start;
        let mut path = Vec::new();
        path.push(start);

        let mut dir = if current.0 == start.0 && current.1 == start.1 - 1 {
            'U'
        } else if current.0 == start.0 && current.1 == start.1 + 1 {
            'D'
        } else if current.0 == start.0 - 1 && current.1 == start.1 {
            'L'
        } else if current.0 == start.0 + 1 && current.1 == start.1 {
            'R'
        } else {
            panic!("invalid starting direction")
        };

        loop {
            match grid.get(&current) {
                None => break,
                Some(pipe) => {
                    if *pipe != 'S' {
                        path.push(current);
                    }

                    match pipe {
                        '.' => break,
                        'S' => return path,
                        '|' => {
                            if dir == 'U' {
                                current = (current.0, current.1 - 1);
                            } else if dir == 'D' {
                                current = (current.0, current.1 + 1);
                            } else {
                                break;
                            }
                        }
                        '-' => {
                            if dir == 'L' {
                                current = (current.0 - 1, current.1);
                            } else if dir == 'R' {
                                current = (current.0 + 1, current.1);
                            } else {
                                break;
                            }
                        }
                        'L' => {
                            if dir == 'L' {
                                dir = 'U';
                                current = (current.0, current.1 - 1);
                            } else if dir == 'D' {
                                dir = 'R';
                                current = (current.0 + 1, current.1);
                            } else {
                                break;
                            }
                        }
                        'J' => {
                            if dir == 'R' {
                                dir = 'U';
                                current = (current.0, current.1 - 1);
                            } else if dir == 'D' {
                                dir = 'L';
                                current = (current.0 - 1, current.1);
                            } else {
                                break;
                            }
                        }
                        '7' => {
                            if dir == 'R' {
                                dir = 'D';
                                current = (current.0, current.1 + 1);
                            } else if dir == 'U' {
                                dir = 'L';
                                current = (current.0 - 1, current.1);
                            } else {
                                break;
                            }
                        }
                        'F' => {
                            if dir == 'U' {
                                dir = 'R';
                                current = (current.0 + 1, current.1);
                            } else if dir == 'L' {
                                dir = 'D';
                                current = (current.0, current.1 + 1);
                            } else {
                                break;
                            }
                        }
                        _ => panic!("unrecognized pipe"),
                    };
                }
            };
        }
    }

    Vec::new()
}

fn shoelace(points: &Vec<&(i32, i32)>) -> f64 {
    let mut sum = 0.0;
    let mut p0 = points[points.len() - 1];

    for p1 in points {
        let p0x: f64 = p0.0.into();
        let p0y: f64 = p0.1.into();
        let p1x: f64 = p1.0.into();
        let p1y: f64 = p1.1.into();

        sum += p0y * p1x - p0x * p1y;
        p0 = *p1
    }

    (sum / 2.0).abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_find_loop() {}

    #[test]
    fn test_shoelace() {
        let points = vec![&(3, 4), &(5, 11), &(12, 8), &(9, 5), &(5, 6)];
        assert_eq!(shoelace(&points), 30.0);
    }

    #[test]
    fn it_works() {
        let mut input = "-L|F7\n7S-7|\nL|7||\n-L-J|\nL|-JF\n";
        assert_eq!(y23d10(input, 1), 4);

        input = "7-F7-\n.FJ|7\nSJLL7\n|F--J\nLJ.LJ\n";
        assert_eq!(y23d10(input, 1), 8);

        input = concat!(
            "...........\n",
            ".S-------7.\n",
            ".|F-----7|.\n",
            ".||.....||.\n",
            ".||.....||.\n",
            ".|L-7.F-J|.\n",
            ".|..|.|..|.\n",
            ".L--J.L--J.\n",
            "...........\n",
        );
        assert_eq!(y23d10(input, 2), 4);

        input = concat!(
            "..........\n",
            ".S------7.\n",
            ".|F----7|.\n",
            ".||....||.\n",
            ".||....||.\n",
            ".|L-7F-J|.\n",
            ".|..||..|.\n",
            ".L--JL--J.\n",
            "..........\n",
        );
        assert_eq!(y23d10(input, 2), 4);

        input = concat!(
            ".F----7F7F7F7F-7....\n",
            ".|F--7||||||||FJ....\n",
            ".||.FJ||||||||L7....\n",
            "FJL7L7LJLJ||LJ.L-7..\n",
            "L--J.L7...LJS7F-7L7.\n",
            "....F-J..F7FJ|L7L7L7\n",
            "....L7.F7||L7|.L7L7|\n",
            ".....|FJLJ|FJ|F7|.LJ\n",
            "....FJL-7.||.||||...\n",
            "....L---J.LJ.LJLJ...\n",
        );
        assert_eq!(y23d10(input, 2), 8);

        input = concat!(
            "FF7FSF7F7F7F7F7F---7\n",
            "L|LJ||||||||||||F--J\n",
            "FL-7LJLJ||||||LJL-77\n",
            "F--JF--7||LJLJ7F7FJ-\n",
            "L---JF-JLJ.||-FJLJJ7\n",
            "|F|F-JF---7F7-L7L|7|\n",
            "|FFJF7L7F-JF7|JL---7\n",
            "7-L-JL7||F7|L7F-7F7|\n",
            "L.L7LFJ|||||FJL7||LJ\n",
            "L7JLJL-JLJLJL--JLJ.L\n",
        );
        assert_eq!(y23d10(input, 2), 10);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day10.txt").unwrap();

        assert_eq!(y23d10(&contents, 1), 6927);
        assert_eq!(y23d10(&contents, 2), 467);
    }
}
