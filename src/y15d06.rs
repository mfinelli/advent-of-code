/* Copyright 2022-2023 Mario Finelli
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

//! Advent of Code 2015 Day 6: <https://adventofcode.com/2015/day/6>
//!
//! TODO

use std::collections::HashMap;

/// Instruction is a representation of the kind of operation to take: toggle
/// the state of the light, turn if on (even if it's already on), or turn it
/// off (even if it's already off). In the case of turning on or off a light
/// that is already on or off this essentially results in a no-op.
#[derive(Debug, PartialEq)]
enum Instruction {
    Toggle,
    TurnOn,
    TurnOff,
}

/// The solution for part one of the day six challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y15d06::y15d06p1;
/// // probably read this from the input file...
/// let input = "";
/// // assert_eq!(y15d06p1(input), "TODO");
/// ```
pub fn y15d06p1(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut on = 0;

    let mut lights = [[false; 1000]; 1000];

    for line in lines {
        let (instruction, x1, y1, x2, y2) = parse_instruction(line);

        for light_row in lights.iter_mut().take(y2 + 1).skip(y1) {
            for light in light_row.iter_mut().take(x2 + 1).skip(x1) {
                if instruction == Instruction::Toggle {
                    if *light {
                        *light = false;
                    } else {
                        *light = true;
                    }
                } else if instruction == Instruction::TurnOff {
                    *light = false;
                } else {
                    *light = true;
                }
            }
        }
    }

    for light_row in &lights {
        for light in light_row.iter() {
            if *light {
                on += 1;
            }
        }
    }

    on
}

/// The solution part two of the day six challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y15d06::y15d06p2;
/// // probably read this from the input file...
/// let input = "";
/// //assert_eq!(y15d06p2(input), "TODO");
/// ```
pub fn y15d06p2(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let mut lights = HashMap::new();
    let mut total = 0;

    for line in lines {
        let (instruction, x1, y1, x2, y2) = parse_instruction(line);

        for y in y1..y2 + 1 {
            for x in x1..x2 + 1 {
                if instruction == Instruction::Toggle {
                    lights.entry((x, y)).and_modify(|v| *v += 2).or_insert(2);
                } else if instruction == Instruction::TurnOff {
                    let v = lights.entry((x, y)).or_insert(0);

                    if *v == 0 {
                        lights.remove(&(x, y));
                    } else {
                        *v -= 1;
                    }
                } else {
                    lights.entry((x, y)).and_modify(|v| *v += 1).or_insert(1);
                }
            }
        }
    }

    for value in lights.values() {
        total += value;
    }

    total
}

fn parse_instruction(line: &str) -> (Instruction, usize, usize, usize, usize) {
    let parts: Vec<_> = line.split_whitespace().collect();

    if line.starts_with("toggle") {
        let from: Vec<_> = parts[1].split(',').collect();
        let to: Vec<_> = parts[3].split(',').collect();
        let x1: usize = from[0].parse().unwrap();
        let x2: usize = to[0].parse().unwrap();
        let y1: usize = from[1].parse().unwrap();
        let y2: usize = to[1].parse().unwrap();

        (Instruction::Toggle, x1, y1, x2, y2)
    } else {
        let from: Vec<_> = parts[2].split(',').collect();
        let to: Vec<_> = parts[4].split(',').collect();
        let x1: usize = from[0].parse().unwrap();
        let x2: usize = to[0].parse().unwrap();
        let y1: usize = from[1].parse().unwrap();
        let y2: usize = to[1].parse().unwrap();

        if line.starts_with("turn off") {
            (Instruction::TurnOff, x1, y1, x2, y2)
        } else {
            (Instruction::TurnOn, x1, y1, x2, y2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_instruction() {
        let mut l = "toggle 1,2 through 10,20";
        assert_eq!(parse_instruction(l), (Instruction::Toggle, 1, 2, 10, 20));

        l = "turn on 3,4 through 30,40";
        assert_eq!(parse_instruction(l), (Instruction::TurnOn, 3, 4, 30, 40));

        l = "turn off 5,6 through 50,60";
        assert_eq!(parse_instruction(l), (Instruction::TurnOff, 5, 6, 50, 60));
    }

    #[test]
    fn it_works() {
        let mut input = "turn on 0,0 through 999,999";
        assert_eq!(y15d06p1(input), 1000000);

        input = "turn on 0,0 through 0,99\ntoggle 0,0 through 0,999\n";
        assert_eq!(y15d06p1(input), 900);

        input = concat!(
            "turn on 0,0 through 999,999\n",
            "toggle 0,0 through 999,0\n",
            "turn off 499,499 through 500,500\n"
        );
        assert_eq!(y15d06p1(input), 998996);

        input = "turn on 0,0 through 0,0\ntoggle 0,0 through 999,999";
        assert_eq!(y15d06p2(input), 2000001);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day06.txt").unwrap();

        assert_eq!(y15d06p1(&contents), 543903);
        assert_eq!(y15d06p2(&contents), 14687245);
    }
}
