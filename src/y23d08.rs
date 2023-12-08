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

//! Advent of Code 2023 Day 8: <https://adventofcode.com/2023/day/8>
//!
//! TODO

use regex::Regex;
use std::collections::HashMap;

/// The solution for the day eight challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y23d08::y23d08;
/// // probably read this from the input file...
/// let input = "";
/// assert_eq!(y23d08(input), 0);
/// ```
pub fn y23d08(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let room_regex = Regex::new(r"^([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)$").unwrap();
    let mut instructions = lines[0].chars().cycle();
    let mut rooms = HashMap::new();
    let mut steps = 0;
    let mut current_room = "AAA".to_string();

    for (i, line) in lines.iter().enumerate() {
        if i <= 1 {
            continue;
        }

        if let Some(captures) = room_regex.captures(line) {
            rooms.insert(captures[1].to_string(), (captures[2].to_string(), captures[3].to_string()));
        }
    }

    // println!("{:?}", instructions);
    // println!("{:?}", rooms);

    loop {
        if current_room == "ZZZ" {
            break;
        }

        steps += 1;
        let (left, right) = rooms.get(&current_room).unwrap();
        match instructions.next().unwrap() {
            'L' => current_room = left.clone(),
            'R' => current_room = right.clone(),
            _ => panic!("invalid instruction"),
        };
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn tit_works() {
        let mut input = concat!(
            "RL\n",
            "\n",
            "AAA = (BBB, CCC)\n",
            "BBB = (DDD, EEE)\n",
            "CCC = (ZZZ, GGG)\n",
            "DDD = (DDD, DDD)\n",
            "EEE = (EEE, EEE)\n",
            "GGG = (GGG, GGG)\n",
            "ZZZ = (ZZZ, ZZZ)\n",
        );

        assert_eq!(y23d08(input), 2);

        input = concat!(
            "LLR\n",
            "\n",
            "AAA = (BBB, BBB)\n",
            "BBB = (AAA, ZZZ)\n",
            "ZZZ = (ZZZ, ZZZ)\n",
        );

        assert_eq!(y23d08(input), 6);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day08.txt").unwrap();

        assert_eq!(y23d08(&contents), 19783);
    }
}
