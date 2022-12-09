/* Copyright 2022 Mario Finelli
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

use aoc::*;
use std::{env, fs, io, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("usage: {} yXXdYY ./path/to/input", args[0]);
        return;
    }

    let input = if args[2] == "-" {
        read_from_stdin()
    } else {
        fs::read_to_string(&args[2]).unwrap()
    };

    match args[1].as_str() {
        "y15d01" => {
            println!("Part 1: {}", y15d01::y15d01p1(&input));
            println!("Part 2: {}", y15d01::y15d01p2(&input).unwrap());
        }
        "y15d02" => {
            println!("Part 1: {}", y15d02::y15d02(&input, 1));
            println!("Part 2: {}", y15d02::y15d02(&input, 2));
        }
        "y15d03" => {
            println!("Part 1: {}", y15d03::y15d03(&input, 1));
            println!("Part 2: {}", y15d03::y15d03(&input, 2));
        }
        "y22d01" => {
            println!("Part 1: {}", y22d01::y22d01(&input, 1));
            println!("Part 2: {}", y22d01::y22d01(&input, 3));
        }
        "y22d02" => {
            println!("Part 1: {}", y22d02::y22d02(&input, 1));
            println!("Part 2: {}", y22d02::y22d02(&input, 2));
        }
        "y22d03" => {
            println!("Part 1: {}", y22d03::y22d03(&input, 1));
            println!("Part 2: {}", y22d03::y22d03(&input, 2));
        }
        "y22d04" => {
            println!("Part 1: {}", y22d04::y22d04(&input, 1));
            println!("Part 2: {}", y22d04::y22d04(&input, 2));
        }
        "y22d05" => {
            println!("Part 1: {}", y22d05::y22d05(&input, 1));
            println!("Part 2: {}", y22d05::y22d05(&input, 2));
        }
        "y22d06" => {
            println!("Part 1: {}", y22d06::y22d06(&input, 4).unwrap());
            println!("Part 2: {}", y22d06::y22d06(&input, 14).unwrap());
        }
        "y22d07" => {
            println!("Part 1: {}", y22d07::y22d07(&input, 1));
            println!("Part 2: {}", y22d07::y22d07(&input, 2));
        }
        "y22d08" => {
            println!("Part 1: {}", y22d08::y22d08(&input, 1));
            println!("Part 2: {}", y22d08::y22d08(&input, 2));
        }
        "y22d09" => {
            println!("Part 1: {}", y22d09::y22d09(&input, 2));
            println!("Part 2: {}", y22d09::y22d09(&input, 10));
        }
        _ => panic!("Unable to find year/day match."),
    };
}

fn read_from_stdin() -> String {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input
}
