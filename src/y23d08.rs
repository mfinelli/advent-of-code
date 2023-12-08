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

use crate::util;
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
/// let input = "LRR\n\n(AAA) = (ZZZ, XXX)";
/// assert_eq!(y23d08(input, 1), 1);
/// assert_eq!(y23d08(input, 2), 1);
/// ```
pub fn y23d08(input: &str, part: u32) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let room_regex =
        Regex::new(r"^([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)$")
            .unwrap();
    let mut instructions = lines[0].chars().cycle();
    let mut rooms = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        if i <= 1 {
            continue;
        }

        if let Some(captures) = room_regex.captures(line) {
            rooms.insert(
                captures[1].to_string(),
                (captures[2].to_string(), captures[3].to_string()),
            );
        }
    }

    let starting_rooms: Vec<&String> = if part == 1 {
        rooms.keys().filter(|r| *r == "AAA").collect()
    } else {
        rooms.keys().filter(|r| r.ends_with('A')).collect()
    };

    let mut all_steps = Vec::new();
    for starting_room in starting_rooms {
        let mut steps = 0;
        let mut current_room = starting_room;
        loop {
            if (part == 1 && current_room == "ZZZ")
                || (part == 2 && current_room.ends_with('Z'))
            {
                break;
            }

            steps += 1;
            let (left, right) = rooms.get(current_room).unwrap();
            match instructions.next().unwrap() {
                'L' => current_room = left,
                'R' => current_room = right,
                _ => panic!("invalid instruction"),
            };
        }
        all_steps.push(steps);
    }

    let mut lcm = 1;
    for steps in all_steps {
        lcm = util::lcm(steps, lcm);
    }

    lcm
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
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
        assert_eq!(y23d08(input, 1), 2);

        input = concat!(
            "LLR\n",
            "\n",
            "AAA = (BBB, BBB)\n",
            "BBB = (AAA, ZZZ)\n",
            "ZZZ = (ZZZ, ZZZ)\n",
        );
        assert_eq!(y23d08(input, 1), 6);

        input = concat!(
            "LR\n",
            "\n",
            "11A = (11B, XXX)\n",
            "11B = (XXX, 11Z)\n",
            "11Z = (11B, XXX)\n",
            "22A = (22B, XXX)\n",
            "22B = (22C, 22C)\n",
            "22C = (22Z, 22Z)\n",
            "22Z = (22B, 22B)\n",
            "XXX = (XXX, XXX)\n",
        );
        assert_eq!(y23d08(input, 2), 6);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day08.txt").unwrap();

        assert_eq!(y23d08(&contents, 1), 19783);
        assert_eq!(y23d08(&contents, 2), 9177460370549);
    }
}
