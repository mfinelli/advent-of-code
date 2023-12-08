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
use crate::util;

/// The solution for the day eight challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y23d08::y23d08;
/// // probably read this from the input file...
/// let input = "";
/// assert_eq!(y23d08(input, 1), 0);
/// ```
pub fn y23d08(input: &str, part: u32) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let room_regex = Regex::new(r"^([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)$").unwrap();
    let mut instructions = lines[0].chars().cycle();
    let mut rooms = HashMap::new();
    // let mut steps = 0;


    for (i, line) in lines.iter().enumerate() {
        if i <= 1 {
            continue;
        }

        if let Some(captures) = room_regex.captures(line) {
            rooms.insert(captures[1].to_string(), (captures[2].to_string(), captures[3].to_string()));
        }
    }

    let mut starting_rooms: Vec<&String> = if part ==1 {
        // vec!["AAA".to_string()]
        rooms.keys().filter(|r| *r == "AAA").collect()
    } else {
        // rooms.keys().filter(|r| r.chars().next().unwrap() == 'A').collect()
        rooms.keys().filter(|r| r.chars().last().unwrap() == 'A').collect()
    };

    // println!("{:?}", instructions);
    // println!("{:?}", rooms);
    // println!("{:?}", current_rooms);
    // return 0;

    let mut all_steps = Vec::new();
    for starting_room in starting_rooms {
        let mut steps = 0;
        let mut current_room = starting_room;
    loop {
        if (part == 1 && current_room == "ZZZ") || (part == 2 && current_room.chars().last().unwrap() == 'Z') {
            break;
        }
        // if part == 1 && current_rooms[0] == "ZZZ" {
        //     break;
        // } else if part == 2 && current_rooms.iter().all(|r| r.chars().last().unwrap() == 'Z') {
        //     break;
        // }

        steps += 1;
        // let mut new_current_rooms = Vec::new();
        // let instruction = instructions.next().unwrap();
        // // for room in current_rooms {
        //     let (left, right) = rooms.get(room).unwrap();
        //     match instruction {
        //         'L' => new_current_rooms.push(left),
        //         'R' => new_current_rooms.push(right),
        //         _ => panic!("invalid instruction"),
        //     }
        // }
        let (left, right) = rooms.get(current_room).unwrap();
        match instructions.next().unwrap() {
            'L' => current_room = left,
            'R' => current_room = right,
            _ => panic!("invalid instruction"),
        };
        // current_rooms = new_current_rooms;
    }
    all_steps.push(steps);
    }

    // println!("{:?}", all_steps);

    let mut lcm = 1;
    for steps in all_steps {
        lcm = util::lcm(steps, lcm);
    }

    lcm
    // steps
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn tit_works() {
        // let mut input = concat!(
        //     "RL\n",
        //     "\n",
        //     "AAA = (BBB, CCC)\n",
        //     "BBB = (DDD, EEE)\n",
        //     "CCC = (ZZZ, GGG)\n",
        //     "DDD = (DDD, DDD)\n",
        //     "EEE = (EEE, EEE)\n",
        //     "GGG = (GGG, GGG)\n",
        //     "ZZZ = (ZZZ, ZZZ)\n",
        // );
        // assert_eq!(y23d08(input, 1), 2);

        // input = concat!(
        //     "LLR\n",
        //     "\n",
        //     "AAA = (BBB, BBB)\n",
        //     "BBB = (AAA, ZZZ)\n",
        //     "ZZZ = (ZZZ, ZZZ)\n",
        // );
        // assert_eq!(y23d08(input, 1), 6);

        // input = concat!(
        let input = concat!(
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
