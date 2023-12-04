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

//! Advent of Code 2015 Day 19: <https://adventofcode.com/2015/day/19>
//!
//! TODO

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;

/// The solution for part one of the day nineteen challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y15d19::y15d19p1;
/// // probably read this from the input file...
/// let input = "Al => ThF\nAl => ThRnFAr\nB => BCa\nB => TiB\n\nnPBP";
/// assert_eq!(y15d19p1(input), 2);
/// ```
pub fn y15d19p1(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let num_replacements = lines.len() - 2;
    let molecule = lines[lines.len() - 1];
    let mut set = HashSet::new();

    let mut replacements = Vec::new();
    for (i, line) in lines.into_iter().enumerate() {
        if i >= num_replacements {
            break;
        }

        let text: Vec<_> = line.split_whitespace().collect();
        replacements.push((text[0], text[2]));
    }

    for (find, replace) in replacements {
        for (i, check) in molecule
            .chars()
            .collect::<Vec<char>>()
            .windows(find.len())
            .enumerate()
        {
            let check: String = check.iter().collect();
            if check == find {
                let new = format!(
                    "{}{}{}",
                    substring(molecule, 0, i),
                    replace,
                    substring(molecule, i + find.len(), molecule.len())
                );
                set.insert(new);
            }
        }
    }

    set.len().try_into().unwrap()
}

/// The solution for part two of the day nineteen challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y15d19::y15d19p2;
/// // probably read this from the input file...
/// let input = "e => A\ne => B\nA => BB\nB => AA\nB => AB\n\nAB";
/// assert_eq!(y15d19p2(input), 2);
/// ```
pub fn y15d19p2(input: &str) -> u32 {
    let mut rng = thread_rng();
    let lines: Vec<_> = input.lines().collect();
    let num_replacements = lines.len() - 2;
    let molecule = lines[lines.len() - 1].to_string();
    let mut reduced = molecule.clone();
    let mut count = 0;

    let mut replacements = Vec::new();
    for (i, line) in lines.into_iter().enumerate() {
        if i >= num_replacements {
            break;
        }

        let text: Vec<_> = line.split_whitespace().collect();
        replacements.push((text[0], text[2]));
    }

    while reduced != "e" {
        let start_len = reduced.len();

        for (find, replace) in &replacements {
            loop {
                let new = reduced.replacen(replace, find, 1);
                if new == reduced {
                    break;
                }
                count += 1;
                reduced = new;
            }
        }

        if start_len == reduced.len() {
            replacements.shuffle(&mut rng);
            reduced = molecule.clone();
            count = 0;
        }
    }

    count
}

/// TODO
fn substring(string: &str, start: usize, end: usize) -> String {
    let s: String = string.chars().skip(start).take(end).collect();
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_substring() {}

    #[test]
    fn it_works() {
        let mut input = "H => HO\nH => OH\nO => HH\n\nHOHOHO\n";
        assert_eq!(y15d19p1(input), 7);

        input = "e => H\ne => O\nH => HO\nH => OH\nO => HH\n\nHOHOHO\n";
        assert_eq!(y15d19p2(input), 6);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day19.txt").unwrap();

        assert_eq!(y15d19p1(&contents), 509);
        assert_eq!(y15d19p2(&contents), 195);
    }
}
