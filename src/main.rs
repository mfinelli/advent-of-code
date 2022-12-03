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

use std::{env, fs};

use aoc::dayone;
use aoc::daytwo;
use aoc::daythree;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("usage: {} yXXdYY ./path/to/input", args[0]);
        return;
    }

    let input = fs::read_to_string(&args[2]).expect("Unable to open input.");

    match args[1].as_str() {
        "y22d01" => {
            println!("Part 1: {}", dayone::dayone(&input, 1));
            println!("Part 2: {}", dayone::dayone(&input, 3));
        }
        "y22d02" => {
            println!("Part 1: {}", daytwo::daytwo(&input, 1));
            println!("Part 2: {}", daytwo::daytwo(&input, 2));
        }
        "y22d03" => {
            println!("Part 1: {}", daythree::daythree(&input));
        }
        _ => panic!("Unable to find year/day match."),
    };
}
