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
//! Today's challenge is pretty straightforward. We don't have all of the
//! information for the Aunt Sues but presumably, there is only one correct
//! answer in the list of Aunt Sues, and so each incorrect Sue will have some
//! disqualifying characteristic. The solution is therefore simple, check the
//! information that we have about each Sue, if we can't find anything that
//! disqualifies here then she's the one that we're looking for!

use regex::Regex;

/// How many children Aunt Sue has (provided by the prompt).
const CHILDREN: u32 = 3;

/// How many cats Aunt Sue has (provided by the prompt).
const CATS: u32 = 7;

/// How many samoyeds Aunt Sue has (provided by the prompt).
const SAMOYEDS: u32 = 2;

/// How many pomeranians Aunt Sue has (provided by the prompt).
const POMERANIANS: u32 = 3;

/// How many akitas Aunt Sue has (provided by the prompt).
const AKITAS: u32 = 0;

/// How many vizslas Aunt Sue has (provided by the prompt).
const VIZSLAS: u32 = 0;

/// How many goldfish Aunt Sue has (provided by the prompt).
const GOLDFISH: u32 = 5;

/// How many trees Aunt Sue has (provided by the prompt).
const TREES: u32 = 3;

/// How many cars Aunt Sue has (provided by the prompt).
const CARS: u32 = 2;

/// How many perfumes Aunt Sue has (provided by the prompt).
const PERFUMES: u32 = 1;

/// AuntSue represents one of the Aunt Sues from the input list. She has an
/// identifying number, and then optional information that we remember about
/// her.
#[derive(Debug, PartialEq)]
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
    /// Creates a new `AuntSue` with the given identifying number and no
    /// information remembered about her (to be filled in later).
    fn new(number: u32) -> AuntSue {
        AuntSue {
            number,
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
/// Starting with the input as a string and the part we're solving (which only
/// affects some of the comparisons that we do to find the real aunt sue) we
/// parse the input into `AuntSue` objects and then try to find out if we have
/// the one we're looking for. In theory we could optimize this further by not
/// creating objects but running the comparison as we try to check each field,
/// but I don't think that it's worth it. Leaving it as-is creates a cleaner,
/// easier-to-reason-about solution. If we get `0` back (which is not a valid
/// Aunt Sue) then it means that we exhausted the input list of Sues and were
/// unable to find the one that we were looking for.
///
/// # Example
/// ```rust
/// # use aoc::y15d16::y15d16;
/// // probably read this from the input file...
/// let input = concat!(
///     "Sue 1: goldfish: 5, cars: 2, samoyeds: 2\n",
///     "Sue 2: goldfish: 4, cars: 2, samoyeds: 2",
/// );
/// assert_eq!(y15d16(input, 1), 1);
/// assert_eq!(y15d16(input, 2), 2);
/// ```
pub fn y15d16(input: &str, part: u32) -> u32 {
    let lines: Vec<_> = input.lines().collect();

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
        let number = number_regex.captures(line).unwrap();
        let mut sue = AuntSue::new(number[1].parse().unwrap());

        if let Some(c) = children_regex.captures(line) {
            sue.children = Some(c[1].parse().unwrap());
        }

        if let Some(c) = cats_regex.captures(line) {
            sue.cats = Some(c[1].parse().unwrap());
        }

        if let Some(s) = samoyeds_regex.captures(line) {
            sue.samoyeds = Some(s[1].parse().unwrap());
        }

        if let Some(p) = pomeranians_regex.captures(line) {
            sue.pomeranians = Some(p[1].parse().unwrap());
        }

        if let Some(a) = akitas_regex.captures(line) {
            sue.akitas = Some(a[1].parse().unwrap());
        }

        if let Some(v) = vizslas_regex.captures(line) {
            sue.vizslas = Some(v[1].parse().unwrap());
        }

        if let Some(g) = goldfish_regex.captures(line) {
            sue.goldfish = Some(g[1].parse().unwrap());
        }

        if let Some(t) = trees_regex.captures(line) {
            sue.trees = Some(t[1].parse().unwrap());
        }

        if let Some(c) = cars_regex.captures(line) {
            sue.cars = Some(c[1].parse().unwrap());
        }

        if let Some(p) = perfumes_regex.captures(line) {
            sue.perfumes = Some(p[1].parse().unwrap());
        }

        if the_real_aunt_sue(&sue, part) {
            return sue.number;
        }
    }

    0
}

/// This function determines if the passed Aunt Sue is the _real_ Aunt Sue. We
/// check each of her characteristics and if we find one that doesn't match
/// the input we were given then we disqualify her. If we can't disqualify her
/// then we have found the one we're looking for.
fn the_real_aunt_sue(sue: &AuntSue, part: u32) -> bool {
    if let Some(c) = sue.children {
        if c != CHILDREN {
            return false;
        }
    }

    if let Some(c) = sue.cats {
        if (part == 2 && c <= CATS) || (part == 1 && c != CATS) {
            return false;
        }
    }

    if let Some(s) = sue.samoyeds {
        if s != SAMOYEDS {
            return false;
        }
    }

    if let Some(p) = sue.pomeranians {
        if (part == 2 && p >= POMERANIANS) || (part == 1 && p != POMERANIANS) {
            return false;
        }
    }

    if let Some(a) = sue.akitas {
        if a != AKITAS {
            return false;
        }
    }

    if let Some(v) = sue.vizslas {
        if v != VIZSLAS {
            return false;
        }
    }

    if let Some(g) = sue.goldfish {
        if (part == 2 && g >= GOLDFISH) || (part == 1 && g != GOLDFISH) {
            return false;
        }
    }

    if let Some(t) = sue.trees {
        if (part == 2 && t <= TREES) || (part == 1 && t != TREES) {
            return false;
        }
    }

    if let Some(c) = sue.cars {
        if c != CARS {
            return false;
        }
    }

    if let Some(p) = sue.perfumes {
        if p != PERFUMES {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_auntsue_new() {
        assert_eq!(
            AuntSue::new(10),
            AuntSue {
                number: 10,
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
        );
    }

    #[test]
    fn test_the_real_aunt_sue() {
        let mut input = AuntSue::new(10);
        input.children = Some(CHILDREN);
        input.cats = Some(CATS);
        input.samoyeds = Some(SAMOYEDS);
        input.pomeranians = Some(POMERANIANS);
        input.akitas = Some(AKITAS);
        input.vizslas = Some(VIZSLAS);
        input.goldfish = Some(GOLDFISH);
        input.trees = Some(TREES);
        input.cars = Some(CARS);
        input.perfumes = Some(PERFUMES);

        assert!(the_real_aunt_sue(&input, 1));

        input.children = Some(CHILDREN + 1);
        assert!(!the_real_aunt_sue(&input, 1));
        input.children = Some(CHILDREN);

        input.cats = Some(CATS + 1);
        assert!(!the_real_aunt_sue(&input, 1));
        input.cats = Some(CATS);

        input.samoyeds = Some(SAMOYEDS + 1);
        assert!(!the_real_aunt_sue(&input, 1));
        input.samoyeds = Some(SAMOYEDS);

        input.pomeranians = Some(POMERANIANS + 1);
        assert!(!the_real_aunt_sue(&input, 1));
        input.pomeranians = Some(POMERANIANS);

        input.akitas = Some(AKITAS + 1);
        assert!(!the_real_aunt_sue(&input, 1));
        input.akitas = Some(AKITAS);

        input.vizslas = Some(VIZSLAS + 1);
        assert!(!the_real_aunt_sue(&input, 1));
        input.vizslas = Some(VIZSLAS);

        input.goldfish = Some(GOLDFISH + 1);
        assert!(!the_real_aunt_sue(&input, 1));
        input.goldfish = Some(GOLDFISH);

        input.trees = Some(TREES + 1);
        assert!(!the_real_aunt_sue(&input, 1));
        input.trees = Some(TREES);

        input.cars = Some(CARS + 1);
        assert!(!the_real_aunt_sue(&input, 1));
        input.cars = Some(CARS);

        input.perfumes = Some(PERFUMES + 1);
        assert!(!the_real_aunt_sue(&input, 1));
        input.perfumes = Some(PERFUMES);

        input.cats = Some(CATS + 1);
        input.pomeranians = Some(POMERANIANS - 1);
        input.goldfish = Some(GOLDFISH - 1);
        input.trees = Some(TREES + 1);
        assert!(the_real_aunt_sue(&input, 2));

        input.cats = Some(CATS);
        assert!(!the_real_aunt_sue(&input, 2));
        input.cats = Some(CATS + 1);

        input.pomeranians = Some(POMERANIANS);
        assert!(!the_real_aunt_sue(&input, 2));
        input.pomeranians = Some(POMERANIANS - 1);

        input.goldfish = Some(GOLDFISH);
        assert!(!the_real_aunt_sue(&input, 2));
        input.goldfish = Some(GOLDFISH - 1);

        input.trees = Some(TREES);
        assert!(!the_real_aunt_sue(&input, 2));
        input.trees = Some(TREES + 1);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day16.txt").unwrap();

        assert_eq!(y15d16(&contents, 1), 40);
        assert_eq!(y15d16(&contents, 2), 241);
    }
}
