/* Copyright 2023-2024 Mario Finelli
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

//! Presentation functions and information for Advent of Code.

use std::io::Write;
use std::time::Duration;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

/// Determines if we also output the year/day combination (Because I can't
/// decide if I like how it looks or not).
const PRINT_DATE: bool = true;

/// Prints our Advent of Code header with the puzzle name.
pub fn print_title(stdout: &mut StandardStream, year: &str, day: &str) {
    let green = green_colorspec();
    let dimmed_white = dimmed_white_colorspec();
    let bold_white = bold_white_colorspec();
    let bold_red = bold_red_colorspec();

    let short_year = {
        let split_pos = year.char_indices().nth_back(1).unwrap().0;
        &year[split_pos..]
    };

    stdout.set_color(&green).unwrap();
    write!(stdout, "🎄 Advent of Code").unwrap();
    stdout.reset().unwrap();

    stdout.set_color(&dimmed_white).unwrap();
    write!(stdout, ": ").unwrap();
    stdout.reset().unwrap();

    stdout.set_color(&bold_white).unwrap();
    write!(stdout, "{}", title(&format!("y{}d{}", short_year, day))).unwrap();
    stdout.reset().unwrap();
    writeln!(stdout, " 🎄").unwrap();

    if PRINT_DATE {
        stdout.set_color(&bold_red).unwrap();
        write!(stdout, "                   🎅 {}", year).unwrap();
        stdout.reset().unwrap();

        stdout.set_color(&dimmed_white).unwrap();
        write!(stdout, " / ").unwrap();
        stdout.reset().unwrap();

        stdout.set_color(&bold_red).unwrap();
        writeln!(stdout, "{}", day).unwrap();
        stdout.reset().unwrap();
    }

    writeln!(stdout).unwrap();
}

/// Prints the answer to an Advent of Code puzzle.
pub fn print_answer(
    stdout: &mut StandardStream,
    part: u32,
    answer: &str,
    separate: bool,
) {
    let dimmed_white = dimmed_white_colorspec();
    let bold_white = bold_white_colorspec();

    stdout.set_color(&dimmed_white).unwrap();
    write!(stdout, "Part {}:", part).unwrap();
    stdout.reset().unwrap();

    if separate {
        writeln!(stdout).unwrap();
    } else {
        write!(stdout, " ").unwrap();
    }

    stdout.set_color(&bold_white).unwrap();
    write!(stdout, "{}", answer).unwrap();
    stdout.reset().unwrap();
    writeln!(stdout).unwrap();
}

/// Prints the solving stats (elapsed time and allocated memory).
pub fn print_stats(
    stdout: &mut StandardStream,
    duration: Duration,
    usage: usize,
) {
    let dimmed_red = dimmed_red_colorspec();
    let dimmed_white = dimmed_white_colorspec();
    let time = duration.as_micros();

    writeln!(stdout).unwrap();

    stdout.set_color(&dimmed_white).unwrap();
    write!(stdout, "Elapsed time: ").unwrap();
    stdout.reset().unwrap();
    stdout.set_color(&dimmed_red).unwrap();

    if time < 10000 {
        write!(stdout, "{}μs", time).unwrap();
    } else if time <= 5000000 {
        write!(stdout, "{}ms", duration.as_millis()).unwrap();
    } else if time <= 600000000 {
        write!(stdout, "{}s", duration.as_secs()).unwrap();
    } else {
        write!(stdout, "{}m", duration.as_secs() / 60).unwrap();
    }

    stdout.reset().unwrap();
    writeln!(stdout).unwrap();

    stdout.set_color(&dimmed_white).unwrap();
    write!(stdout, "Allocated memory: ").unwrap();
    stdout.reset().unwrap();
    stdout.set_color(&dimmed_red).unwrap();

    if usage <= 1500 {
        write!(stdout, "{}b", usage).unwrap();
    } else if usage <= 1048576 {
        write!(stdout, "{:.2}kb", usage as f32 / 1024.0).unwrap();
    } else if usage <= 1073741824 {
        write!(stdout, "{:.2}mb", usage as f32 / 1048576.0).unwrap();
    } else {
        write!(stdout, "{:.2}gb", usage as f32 / 1073741824.0).unwrap();
    }

    stdout.reset().unwrap();
    writeln!(stdout).unwrap();
}

/// Returns the title of the puzzle for the given year and day.
fn title(yd: &str) -> &str {
    match yd {
        // 2015
        "y15d01" => "Not Quite Lisp",
        "y15d02" => "I Was Told There Would Be No Math",
        "y15d03" => "Perfectly Spherical Houses in a Vacuum",
        "y15d04" => "The Ideal Stocking Stuffer",
        "y15d05" => "Doesn't He Have Intern-Elves For This?",
        "y15d06" => "Probably a Fire Hazard",
        "y15d07" => "Some Assembly Required",
        "y15d08" => "Matchsticks",
        "y15d09" => "All in a Single Night",
        "y15d10" => "Elves Look, Elves Say",
        "y15d11" => "Corporate Policy",
        "y15d12" => "JSAbacusFramework.io",
        "y15d13" => "Knights of the Dinner Table",
        "y15d14" => "Reindeer Olympics",
        "y15d15" => "Science for Hungry People",
        "y15d16" => "Aunt Sue",
        "y15d17" => "No Such Thing as Too Much",
        "y15d18" => "Like a GIF For Your Yard",
        "y15d19" => "Medicine for Rudolph",

        // 2022
        "y22d01" => "Calorie Counting",
        "y22d02" => "Rock Paper Scissors",
        "y22d03" => "Rucksack Reorganization",
        "y22d04" => "Camp Cleanup",
        "y22d05" => "Supply Stacks",
        "y22d06" => "Tuning Trouble",
        "y22d07" => "No Space Left On Device",
        "y22d08" => "Treetop Tree House",
        "y22d09" => "Rope Bridge",
        "y22d10" => "Cathode-Ray Tube",
        "y22d11" => "Monkey in the Middle",
        "y22d12" => "Hill Climbing Algorithm",
        "y22d13" => "Distress Signal",
        "y22d14" => "Regolith Reservoir",
        "y22d15" => "Beacon Exclusion Zone",
        "y22d16" => "Proboscidea Volcanium",
        "y22d17" => "Pyroclastic Flow",

        // 2023
        "y23d01" => "Trebuchet?!",
        "y23d02" => "Cube Conundrum",
        "y23d03" => "Gear Ratios",
        "y23d04" => "Scratchcards",
        "y23d05" => "If You Give A Seed A Fertilizer",
        "y23d06" => "Wait For It",
        "y23d07" => "Camel Cards",
        "y23d08" => "Haunted Wasteland",
        "y23d09" => "Mirage Maintenance",
        "y23d10" => "Pipe Maze",
        "y23d11" => "Cosmic Expansion",
        "y23d12" => "Hot Springs",
        "y23d13" => "Point of Incidence",
        "y23d14" => "Parabolic Reflector Dish",
        "y23d15" => "Lens Library",
        "y23d16" => "The Floor Will Be Lava",

        // 2024
        "y24d01" => "Historian Hysteria",
        "y24d02" => "Red-Nosed Reports",
        "y24d03" => "Mull It Over",
        "y24d04" => "Ceres Search",
        "y24d05" => "Print Queue",
        "y24d06" => "Guard Gallivant",

        // 2025
        "y25d01" => "Secret Entrance",
        "y25d02" => "Gift Shop",
        "y25d03" => "Lobby",
        "y25d04" => "Printing Department",

        _ => panic!("unknown puzzle title"),
    }
}

/// Returns a [`termcolor::ColorSpec`] for a bolded, red foreground.
fn bold_red_colorspec() -> ColorSpec {
    let mut spec = ColorSpec::new();
    spec.set_fg(Some(Color::Red));
    spec.set_bold(true);
    spec
}

/// Returns a [`termcolor::ColorSpec`] for a bolded, white foreground.
fn bold_white_colorspec() -> ColorSpec {
    let mut spec = ColorSpec::new();
    spec.set_fg(Some(Color::White));
    spec.set_bold(true);
    spec
}

/// Returns a [`termcolor::ColorSpec`] for a green foreground.
fn green_colorspec() -> ColorSpec {
    let mut spec = ColorSpec::new();
    spec.set_fg(Some(Color::Green));
    spec
}

/// Returns a [`termcolor::ColorSpec`] for a dimmed, yellow foreground.
fn dimmed_red_colorspec() -> ColorSpec {
    let mut spec = ColorSpec::new();
    spec.set_fg(Some(Color::Red));
    spec.set_dimmed(true);
    spec
}

/// Returns a [`termcolor::ColorSpec`] for a dimmed, white foreground.
fn dimmed_white_colorspec() -> ColorSpec {
    let mut spec = ColorSpec::new();
    spec.set_fg(Some(Color::White));
    spec.set_dimmed(true);
    spec
}
