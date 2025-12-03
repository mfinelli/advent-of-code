/* Copyright 2022-2024 Mario Finelli
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
use peak_alloc::PeakAlloc;
use std::{env, fs, io, io::IsTerminal, io::Read, time::Instant};
use termcolor::{ColorChoice, StandardStream};

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("usage: {} YYYY DD ./path/to/input", args[0]);
        return;
    }

    let input = if args[3] == "-" {
        read_from_stdin()
    } else {
        fs::read_to_string(&args[3]).unwrap()
    };

    let color_choice = if !io::stdout().is_terminal() {
        ColorChoice::Never
    } else {
        ColorChoice::Auto
    };

    let mut stdout = StandardStream::stdout(color_choice);
    let day: u8 = args[2].parse().unwrap();
    title::print_title(&mut stdout, &args[1], &format!("{:02}", day));

    let part1: String;
    let part2: String;
    let part1_sep = false;
    let mut part2_sep = false;

    let problem_lookup = {
        let split_pos = args[1].char_indices().nth_back(1).unwrap().0;
        let yr = &args[1][split_pos..];
        format!("y{}d{:02}", yr, day)
    };

    match problem_lookup.as_str() {
        // 2015
        "y15d01" => {
            part1 = format!("{}", y15d01::y15d01p1(&input));
            part2 = format!("{}", y15d01::y15d01p2(&input).unwrap());
        }
        "y15d02" => {
            part1 = format!("{}", y15d02::y15d02(&input, 1));
            part2 = format!("{}", y15d02::y15d02(&input, 2));
        }
        "y15d03" => {
            part1 = format!("{}", y15d03::y15d03(&input, 1));
            part2 = format!("{}", y15d03::y15d03(&input, 2));
        }
        "y15d04" => {
            part1 = format!("{}", y15d04::y15d04(input.clone(), 5).unwrap());
            part2 = format!("{}", y15d04::y15d04(input, 6).unwrap());
        }
        "y15d05" => {
            part1 = format!("{}", y15d05::y15d05(&input, 1));
            part2 = format!("{}", y15d05::y15d05(&input, 2));
        }
        "y15d06" => {
            part1 = format!("{}", y15d06::y15d06p1(&input));
            part2 = format!("{}", y15d06::y15d06p2(&input));
        }
        "y15d07" => {
            part1 = format!("{}", y15d07::y15d07(&input, "a", 1));
            part2 = format!("{}", y15d07::y15d07(&input, "a", 2));
        }
        "y15d08" => {
            part1 = format!("{}", y15d08::y15d08p1(&input));
            part2 = format!("{}", y15d08::y15d08p2(&input));
        }
        "y15d09" => {
            part1 = format!("{}", y15d09::y15d09(&input, 1));
            part2 = format!("{}", y15d09::y15d09(&input, 2));
        }
        "y15d10" => {
            part1 = format!("{}", y15d10::y15d10(&input, 40));
            part2 = format!("{}", y15d10::y15d10(&input, 50));
        }
        "y15d11" => {
            part1 = y15d11::y15d11(&input, 1).to_string();
            part2 = y15d11::y15d11(&input, 2).to_string();
        }
        "y15d12" => {
            part1 = format!("{}", y15d12::y15d12(&input, false));
            part2 = format!("{}", y15d12::y15d12(&input, true));
        }
        "y15d13" => {
            part1 = format!("{}", y15d13::y15d13(&input, false));
            part2 = format!("{}", y15d13::y15d13(&input, true));
        }
        "y15d14" => {
            part1 = format!("{}", y15d14::y15d14(&input, 2503, 1));
            part2 = format!("{}", y15d14::y15d14(&input, 2503, 2));
        }
        "y15d15" => {
            part1 = format!("{}", y15d15::y15d15(&input, 1));
            part2 = format!("{}", y15d15::y15d15(&input, 2));
        }
        "y15d16" => {
            part1 = format!("{}", y15d16::y15d16(&input, 1));
            part2 = format!("{}", y15d16::y15d16(&input, 2));
        }
        "y15d17" => {
            part1 = format!("{}", y15d17::y15d17(&input, 150, 1));
            part2 = format!("{}", y15d17::y15d17(&input, 150, 2));
        }
        "y15d18" => {
            part1 = format!("{}", y15d18::y15d18(&input, 100, 1));
            part2 = format!("{}", y15d18::y15d18(&input, 100, 2));
        }
        "y15d19" => {
            part1 = format!("{}", y15d19::y15d19p1(&input));
            part2 = format!("{}", y15d19::y15d19p2(&input));
        }

        // 2022
        "y22d01" => {
            part1 = format!("{}", y22d01::y22d01(&input, 1));
            part2 = format!("{}", y22d01::y22d01(&input, 3));
        }
        "y22d02" => {
            part1 = format!("{}", y22d02::y22d02(&input, 1));
            part2 = format!("{}", y22d02::y22d02(&input, 2));
        }
        "y22d03" => {
            part1 = format!("{}", y22d03::y22d03(&input, 1));
            part2 = format!("{}", y22d03::y22d03(&input, 2));
        }
        "y22d04" => {
            part1 = format!("{}", y22d04::y22d04(&input, 1));
            part2 = format!("{}", y22d04::y22d04(&input, 2));
        }
        "y22d05" => {
            part1 = y22d05::y22d05(&input, 1).to_string();
            part2 = y22d05::y22d05(&input, 2).to_string();
        }
        "y22d06" => {
            part1 = format!("{}", y22d06::y22d06(&input, 4).unwrap());
            part2 = format!("{}", y22d06::y22d06(&input, 14).unwrap());
        }
        "y22d07" => {
            part1 = format!("{}", y22d07::y22d07(&input, 1));
            part2 = format!("{}", y22d07::y22d07(&input, 2));
        }
        "y22d08" => {
            part1 = format!("{}", y22d08::y22d08(&input, 1));
            part2 = format!("{}", y22d08::y22d08(&input, 2));
        }
        "y22d09" => {
            part1 = format!("{}", y22d09::y22d09(&input, 2));
            part2 = format!("{}", y22d09::y22d09(&input, 10));
        }
        "y22d10" => {
            part1 = format!("{}", y22d10::y22d10p1(&input));
            part2 = y22d10::y22d10p2(&input).to_string();
            part2_sep = true;
        }
        "y22d11" => {
            part1 = format!("{}", y22d11::y22d11(&input, 20, true));
            part2 = format!("{}", y22d11::y22d11(&input, 10000, false));
        }
        "y22d12" => {
            part1 = format!("{}", y22d12::y22d12(&input, 1).unwrap());
            part2 = format!("{}", y22d12::y22d12(&input, 2).unwrap());
        }
        "y22d13" => {
            part1 = format!("{}", y22d13::y22d13(&input, 1));
            part2 = format!("{}", y22d13::y22d13(&input, 2));
        }
        "y22d14" => {
            part1 = format!("{}", y22d14::y22d14(&input, 1));
            part2 = format!("{}", y22d14::y22d14(&input, 2));
        }
        "y22d15" => {
            part1 = format!("{}", y22d15::y22d15p1(&input, 2000000));
            part2 = format!("{}", y22d15::y22d15p2(&input, 4000000));
        }
        "y22d16" => {
            part1 = format!("{}", y22d16::y22d16(&input, 1));
            part2 = format!("{}", y22d16::y22d16(&input, 2));
        }

        // 2023
        "y23d01" => {
            part1 = format!("{}", y23d01::y23d01(&input, 1));
            part2 = format!("{}", y23d01::y23d01(&input, 2));
        }
        "y23d02" => {
            part1 = format!("{}", y23d02::y23d02(&input, 1));
            part2 = format!("{}", y23d02::y23d02(&input, 2));
        }
        "y23d03" => {
            part1 = format!("{}", y23d03::y23d03p1(&input));
            part2 = format!("{}", y23d03::y23d03p2(&input));
        }
        "y23d04" => {
            part1 = format!("{}", y23d04::y23d04(&input, 1));
            part2 = format!("{}", y23d04::y23d04(&input, 2));
        }
        "y23d05" => {
            part1 = format!("{}", y23d05::y23d05(&input, 1));
            part2 = format!("{}", y23d05::y23d05(&input, 2));
        }
        "y23d06" => {
            part1 = format!("{}", y23d06::y23d06(&input, 1));
            part2 = format!("{}", y23d06::y23d06(&input, 2));
        }
        "y23d07" => {
            part1 = format!("{}", y23d07::y23d07(&input, 1));
            part2 = format!("{}", y23d07::y23d07(&input, 2));
        }
        "y23d08" => {
            part1 = format!("{}", y23d08::y23d08(&input, 1));
            part2 = format!("{}", y23d08::y23d08(&input, 2));
        }
        "y23d09" => {
            part1 = format!("{}", y23d09::y23d09(&input, 1));
            part2 = format!("{}", y23d09::y23d09(&input, 2));
        }
        "y23d10" => {
            part1 = format!("{}", y23d10::y23d10(&input, 1));
            part2 = format!("{}", y23d10::y23d10(&input, 2));
        }
        "y23d11" => {
            part1 = format!("{}", y23d11::y23d11(&input, 1));
            part2 = format!("{}", y23d11::y23d11(&input, 1000000));
        }
        "y23d12" => {
            part1 = format!("{}", y23d12::y23d12(&input, 1));
            part2 = format!("{}", y23d12::y23d12(&input, 5));
        }
        "y23d13" => {
            part1 = format!("{}", y23d13::y23d13(&input, 1));
            part2 = format!("{}", y23d13::y23d13(&input, 2));
        }
        "y23d14" => {
            part1 = format!("{}", y23d14::y23d14(&input, 1));
            part2 = format!("{}", y23d14::y23d14(&input, 2));
        }
        "y23d15" => {
            part1 = format!("{}", y23d15::y23d15(&input, 1));
            part2 = format!("{}", y23d15::y23d15(&input, 2));
        }
        "y23d16" => {
            part1 = format!("{}", y23d16::y23d16(&input, 1));
            part2 = format!("{}", y23d16::y23d16(&input, 2));
        }

        // 2024
        "y24d01" => {
            part1 = format!("{}", y24d01::y24d01p1(&input));
            part2 = format!("{}", y24d01::y24d01p2(&input));
        }
        "y24d02" => {
            part1 = format!("{}", y24d02::y24d02(&input, 1));
            part2 = format!("{}", y24d02::y24d02(&input, 2));
        }
        "y24d03" => {
            part1 = format!("{}", y24d03::y24d03(&input, 1));
            part2 = format!("{}", y24d03::y24d03(&input, 2));
        }
        "y24d04" => {
            part1 = format!("{}", y24d04::y24d04p1(&input));
            part2 = format!("{}", y24d04::y24d04p2(&input));
        }
        "y24d05" => {
            part1 = format!("{}", y24d05::y24d05(&input, 1));
            part2 = format!("{}", y24d05::y24d05(&input, 2));
        }
        "y24d06" => {
            part1 = format!("{}", y24d06::y24d06(&input, 1));
            part2 = format!("{}", y24d06::y24d06(&input, 2));
        }

        // 2025
        "y25d01" => {
            part1 = format!("{}", y25d01::y25d01(&input, 1));
            part2 = format!("{}", y25d01::y25d01(&input, 2));
        }
        "y25d02" => {
            part1 = format!("{}", y25d02::y25d02(&input, 1));
            part2 = format!("{}", y25d02::y25d02(&input, 2));
        }
        "y25d03" => {
            part1 = format!("{}", y25d03::y25d03(&input));
            part2 = "".to_string();
        }

        _ => panic!("Unable to find year/day match."),
    };

    title::print_answer(&mut stdout, 1, &part1, part1_sep);
    title::print_answer(&mut stdout, 2, &part2, part2_sep);

    let elapsed = start.elapsed();
    let peak_usage = PEAK_ALLOC.peak_usage();
    title::print_stats(&mut stdout, elapsed, peak_usage);
}

fn read_from_stdin() -> String {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input
}
