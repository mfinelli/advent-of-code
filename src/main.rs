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

use aoc::*;
use std::{env, fs, io, io::Read, time::Instant};

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

    let start = Instant::now();

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
        "y15d04" => {
            println!("Part 1: {}", y15d04::y15d04(input.clone(), 5).unwrap());
            println!("Part 2: {}", y15d04::y15d04(input, 6).unwrap());
        }
        "y15d05" => {
            println!("Part 1: {}", y15d05::y15d05(&input, 1));
            println!("Part 2: {}", y15d05::y15d05(&input, 2));
        }
        "y15d06" => {
            println!("Part 1: {}", y15d06::y15d06p1(&input));
            println!("Part 2: {}", y15d06::y15d06p2(&input));
        }
        "y15d07" => {
            println!("Part 1: {}", y15d07::y15d07(&input, "a", 1));
            println!("Part 2: {}", y15d07::y15d07(&input, "a", 2));
        }
        "y15d08" => {
            println!("Part 1: {}", y15d08::y15d08p1(&input));
            println!("Part 2: {}", y15d08::y15d08p2(&input));
        }
        "y15d09" => {
            println!("Part 1: {}", y15d09::y15d09(&input));
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
        "y22d10" => {
            println!("Part 1: {}", y22d10::y22d10p1(&input));
            println!("Part 2:\n{}", y22d10::y22d10p2(&input));
        }
        "y22d11" => {
            println!("Part 1: {}", y22d11::y22d11(&input, 20, true));
            println!("Part 2: {}", y22d11::y22d11(&input, 10000, false));
        }
        "y22d12" => {
            println!("Part 1: {}", y22d12::y22d12(&input, 1).unwrap());
            println!("Part 2: {}", y22d12::y22d12(&input, 2).unwrap());
        }
        "y22d13" => {
            println!("Part 1: {}", y22d13::y22d13(&input, 1));
            println!("Part 2: {}", y22d13::y22d13(&input, 2));
        }
        "y22d14" => {
            println!("Part 1: {}", y22d14::y22d14(&input, 1));
            println!("Part 2: {}", y22d14::y22d14(&input, 2));
        }
        "y22d15" => {
            println!("Part 1: {}", y22d15::y22d15p1(&input, 2000000));
            println!("Part 2: {}", y22d15::y22d15p2(&input, 4000000));
        }
        "y22d16" => {
            println!("Part 1: {}", y22d16::y22d16(&input, 1));
            println!("Part 2: {}", y22d16::y22d16(&input, 2));
        }
        _ => panic!("Unable to find year/day match."),
    };

    println!("\nRun time: {}μs", start.elapsed().as_micros());
}

fn read_from_stdin() -> String {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input
}
