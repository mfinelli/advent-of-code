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

//! Advent of Code 2015 Day 16: <https://adventofcode.com/2015/day/16>
//!
//! TODO

use regex::Regex;

/// TODO
const CHILDREN: u32 = 3;

/// TODO
const CATS: u32 = 7;

/// TODO
const SAMOYEDS: u32 = 2;

/// TODO
const POMERANIANS: u32 = 3;

/// TODO
const AKITAS: u32 = 0;

/// TODO
const VIZSLAS: u32 = 0;

/// TODO
const GOLDFISH: u32 = 5;

/// TODO
const TREES: u32 = 3;

/// TODO
const CARS: u32 = 2;

/// TODO
const PERFUMES: u32 = 1;


/// TODO
#[derive(Debug)]
struct AuntSue {
    number: u32,
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

impl AuntSue {
    fn new(number: u32) -> AuntSue {
        AuntSue {
            number: number,
            children: None,
            cats: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None,
        }
    }
}

/// The solution for the day sixteen challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y15d16::y15d16;
/// // probably read this from the input file...
/// let input = "";
/// assert_eq!(y15d16(input), 0);
/// ```
pub fn y15d16(input: &str, part: u32) -> u32 {
    let lines: Vec<_> = input.lines().collect();

    let mut sues = Vec::new();
    let number_regex = Regex::new(r"^Sue (\d+):").unwrap();
    let children_regex = Regex::new(r"children: (\d+)").unwrap();
    let cats_regex = Regex::new(r"cats: (\d+)").unwrap();
    let samoyeds_regex = Regex::new(r"samoyeds: (\d+)").unwrap();
    let pomeranians_regex = Regex::new(r"pomeranians: (\d+)").unwrap();
    let akitas_regex = Regex::new(r"akitas: (\d+)").unwrap();
    let vizslas_regex = Regex::new(r"vizslas: (\d+)").unwrap();
    let goldfish_regex = Regex::new(r"goldfish: (\d+)").unwrap();
    let trees_regex = Regex::new(r"trees: (\d+)").unwrap();
    let cars_regex = Regex::new(r"cars: (\d+)").unwrap();
    let perfumes_regex = Regex::new(r"perfumes: (\d+)").unwrap();

    for line in lines {
        // println!("{:?}", line);
        let number = number_regex.captures(line).unwrap();
        // let children = children_regex.captures(line);
        // let goldfish = goldfish_regex.captures(line);

        // println!("{:?}", children);
        // println!("{:?}", goldfish);
        // println!("{:?}", num);
        // break;
        // sues.push(AuntSue {
        //     number: num.parse().unwrap(),
        // });
        // sues.push(AuntSue::new(num[1].parse().unwrap()));
        let mut sue = AuntSue::new(number[1].parse().unwrap());

        match children_regex.captures(line) {
            Some(m) => {
                sue.children = Some(m[1].parse().unwrap());
            }
            None => {}
        }

        match cats_regex.captures(line) {
            Some(m) => {
                sue.cats = Some(m[1].parse().unwrap());
            }
            None => {}
        }

        match samoyeds_regex.captures(line) {
            Some(m) => {
                sue.samoyeds = Some(m[1].parse().unwrap());
            }
            None => {}
        }

        match pomeranians_regex.captures(line) {
            Some(m) => {
                sue.pomeranians = Some(m[1].parse().unwrap());
            }
            None => {}
        }

        match akitas_regex.captures(line) {
            Some(m) => {
                sue.akitas = Some(m[1].parse().unwrap());
            }
            None => {}
        }

        match vizslas_regex.captures(line) {
            Some(m) => {
                sue.vizslas = Some(m[1].parse().unwrap());
            }
            None => {}
        }

        match goldfish_regex.captures(line) {
            Some(m) => {
                sue.goldfish = Some(m[1].parse().unwrap());
            }
            None => {}
        }

        match trees_regex.captures(line) {
            Some(m) => {
                sue.trees = Some(m[1].parse().unwrap());
            }
            None => {}
        }

        match cars_regex.captures(line) {
            Some(m) => {
                sue.cars = Some(m[1].parse().unwrap());
            }
            None => {}
        }

        match perfumes_regex.captures(line) {
            Some(m) => {
                sue.perfumes = Some(m[1].parse().unwrap());
            }
            None => {}
        }

        sues.push(sue);
    }

    // println!("{:?}", sues);
    //
    //
    for sue in sues {
        if the_real_aunt_sue(&sue, part) {
            return sue.number;
        }
    }


    // 1
    0
}

/// TODO
fn the_real_aunt_sue(sue: &AuntSue, part: u32) -> bool {
    match sue.children {
        Some(c) => {
            if c != CHILDREN {
                return false;
            }
        }
        None => {}
    }

    match sue.cats {
        Some(c) => {
            if (part == 2 && c <= CATS) || (part == 1 && c != CATS) {
                return false;
            }
        }
        None => {}
    }

    match sue.samoyeds {
        Some(s) => {
            if s != SAMOYEDS {
                return false;
            }
        }
        None => {}
    }

    match sue.pomeranians {
        Some(p) => {
            if (part == 2 && p >= POMERANIANS) || (part == 1 && p != POMERANIANS) {
                return false;
            }
        }
        None => {}
    }

    match sue.akitas {
        Some(a) => {
            if a != AKITAS {
                return false;
            }
        }
        None => {}
    }

    match sue.vizslas {
        Some(v) => {
            if v != VIZSLAS {
                return false;
            }
        }
        None => {}
    }

    match sue.goldfish {
        Some(g) => {
            if (part == 2 && g >= GOLDFISH) || (part == 1 && g != GOLDFISH) {
                return false;
            }
        }
        None => {}
    }

    match sue.trees {
        Some(t) => {
            if (part == 2 && t <= TREES) || (part == 1 && t != TREES) {
                return false;
            }
        }
        None => {}
    }

    match sue.cars {
        Some(c) => {
            if c != CARS {
                return false;
            }
        }
        None => {}
    }

    match sue.perfumes {
        Some(p) => {
            if p != PERFUMES {
                return false;
            }
        }
        None => {}
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn tthe_solution() {
        let contents = fs::read_to_string("input/2015/day16.txt").unwrap();

        assert_eq!(y15d16(&contents, 1), 40);
        assert_eq!(y15d16(&contents, 2), 241);
    }
}
