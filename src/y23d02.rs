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
use std::collections::BinaryHeap;

/// Draw repredents a handful of cubes in a game. Each color (red, blue, and
/// green) can have some or none.
#[derive(Debug)]
struct Draw {
    red: Option<u32>,
    blue: Option<u32>,
    green: Option<u32>,
}

/// Game represents each game from the input, it has a number and a variable
/// number of draws.
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
/// assert_eq!(y23d02(input, 1), 0);
/// assert_eq!(y23d02(input, 2), 0);
/// ```
pub fn y23d02(input: &str, part: u32) -> u32 {
    let mut sum = 0;
    let mut games = Vec::new();
    let red_regex = Regex::new(r"(\d+) red").unwrap();
    let blue_regex = Regex::new(r"(\d+) blue").unwrap();
    let green_regex = Regex::new(r"(\d+) green").unwrap();

    for line in input.lines() {
        let parts: Vec<_> = line.split(": ").collect();
        let number = parts[0].split_whitespace().collect::<Vec<_>>()[1]
            .parse()
            .unwrap();
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

        games.push(Game { number, draws });
    }

    if part == 1 {
        for game in games {
            if is_possible(&game) {
                sum += game.number;
            }
        }
    } else {
        for game in games {
            sum += compute_power(&game);
        }
    }

    sum
}

/// TODO
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

/// TODO
fn compute_power(game: &Game) -> u32 {
    let mut red = BinaryHeap::from([0]);
    let mut blue = BinaryHeap::from([0]);
    let mut green = BinaryHeap::from([0]);

    for draw in &game.draws {
        if let Some(r) = draw.red {
            red.push(r);
        }

        if let Some(b) = draw.blue {
            blue.push(b);
        }

        if let Some(g) = draw.green {
            green.push(g);
        }
    }

    red.pop().unwrap() * blue.pop().unwrap() * green.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_is_possible() {
        let game = Game {
            number: 1,
            draws: vec![
                Draw {
                    red: Some(4),
                    blue: Some(3),
                    green: None,
                },
                Draw {
                    red: Some(1),
                    blue: Some(6),
                    green: Some(2),
                },
                Draw {
                    red: None,
                    blue: None,
                    green: Some(2),
                },
            ],
        };

        assert!(is_possible(&game));

        let game = Game {
            number: 2,
            draws: vec![
                Draw {
                    red: None,
                    blue: Some(1),
                    green: Some(2),
                },
                Draw {
                    red: Some(1),
                    blue: Some(4),
                    green: Some(3),
                },
                Draw {
                    red: None,
                    blue: Some(1),
                    green: Some(1),
                },
            ],
        };

        assert!(is_possible(&game));

        let game = Game {
            number: 3,
            draws: vec![
                Draw {
                    red: Some(20),
                    blue: Some(6),
                    green: Some(8),
                },
                Draw {
                    red: Some(4),
                    blue: Some(5),
                    green: None,
                },
                Draw {
                    red: None,
                    blue: None,
                    green: Some(13),
                },
                Draw {
                    red: Some(1),
                    blue: None,
                    green: Some(5),
                },
            ],
        };

        assert!(!is_possible(&game));
    }

    #[test]
    fn test_compute_power() {
        let game = Game {
            number: 1,
            draws: vec![
                Draw {
                    red: Some(4),
                    blue: Some(3),
                    green: None,
                },
                Draw {
                    red: Some(1),
                    blue: Some(6),
                    green: Some(2),
                },
                Draw {
                    red: None,
                    blue: None,
                    green: Some(2),
                },
            ],
        };

        assert_eq!(compute_power(&game), 48);

        let game = Game {
            number: 2,
            draws: vec![
                Draw {
                    red: None,
                    blue: Some(1),
                    green: Some(2),
                },
                Draw {
                    red: Some(1),
                    blue: Some(4),
                    green: Some(3),
                },
                Draw {
                    red: None,
                    blue: Some(1),
                    green: Some(1),
                },
            ],
        };

        assert_eq!(compute_power(&game), 12);

        let game = Game {
            number: 3,
            draws: vec![
                Draw {
                    red: Some(20),
                    blue: Some(6),
                    green: Some(8),
                },
                Draw {
                    red: Some(4),
                    blue: Some(5),
                    green: None,
                },
                Draw {
                    red: None,
                    blue: None,
                    green: Some(13),
                },
                Draw {
                    red: Some(1),
                    blue: None,
                    green: Some(5),
                },
            ],
        };

        assert_eq!(compute_power(&game), 1560);
    }

    #[test]
    fn it_works() {
        let input = concat!(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, ",
            "1 blue\n",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; ",
            "5 green, 1 red\n",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 ",
            "blue, 14 red\n",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n",
        );

        assert_eq!(y23d02(input, 1), 8);
        assert_eq!(y23d02(input, 2), 2286);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day02.txt").unwrap();

        assert_eq!(y23d02(&contents, 1), 2551);
        assert_eq!(y23d02(&contents, 2), 62811);
    }
}
