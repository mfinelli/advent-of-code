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

//! Advent of Code 2023 Day 2: <https://adventofcode.com/2023/day/2>
//!
//! TODO

use regex::Regex;

/// TODO
#[derive(Debug)]
struct Draw {
    red: Option<u32>,
    blue: Option<u32>,
    green: Option<u32>,
}

/// TODO
#[derive(Debug)]
struct Game {
    number: u32,
    draws: Vec<Draw>,
}

/// The solution for the day two challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y23d02::y23d02;
/// // probably read this from the input file...
/// let input = "";
/// assert_eq!(y23d02(input), 0);
/// ```
pub fn y23d02(input: &str) -> u32 {
    let mut games = Vec::new();
    let red_regex = Regex::new(r"(\d+) red").unwrap();
    let blue_regex = Regex::new(r"(\d+) blue").unwrap();
    let green_regex = Regex::new(r"(\d+) green").unwrap();

    for line in input.lines() {
        let parts: Vec<_> = line.split(": ").collect();
        let number = parts[0].split_whitespace().collect::<Vec<_>>()[1].parse().unwrap();
        let mut draws = Vec::new();

        for d in parts[1].split("; ") {
            let mut draw = Draw {
                red: None,
                blue: None,
                green: None,
            };

            if let Some(r) = red_regex.captures(d) {
                draw.red = Some(r[1].parse().unwrap());
            }

            if let Some(b) = blue_regex.captures(d) {
                draw.blue = Some(b[1].parse().unwrap());
            }

            if let Some(g) = green_regex.captures(d) {
                draw.green = Some(g[1].parse().unwrap());
            }

            draws.push(draw);
        }

        games.push(Game {
            number: number,
            draws: draws,
        });
    }

    let mut sum = 0;
    for game in games {
        if is_possible(&game) {
            sum += game.number;
        }
    }

    sum
}

fn is_possible(game: &Game) -> bool {
    let red = 12;
    let blue = 14;
    let green = 13;

    for draw in &game.draws {
        if let Some(r) = draw.red {
            if r > red {
                return false;
            }
        }

        if let Some(b) = draw.blue {
            if b > blue {
                return false;
            }
        }

        if let Some(g) = draw.green {
            if g > green {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = concat!(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n",
        );

        assert_eq!(y23d02(input), 8);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day02.txt").unwrap();

        assert_eq!(y23d02(&contents), 2551);
    }
}
