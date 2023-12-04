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

//! Advent of Code 2023 Day 4: <https://adventofcode.com/2023/day/4>
//!
//! TODO

use std::collections::HashSet;
use std::collections::HashMap;

/// The solution for the day four challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y23d04::y23d04;
/// // probably read this from the input file...
/// let input = ""
/// assert_eq!(y23d04(input), 0);
/// ```
pub fn y23d04(input: &str, part: u32) -> u32 {
    let base: u32 = 2;
    let lines: Vec<_> = input.lines().collect();
    let mut cards = HashMap::new();
    let mut sum = 0;

    if part == 2 {
        for card in 0..lines.len() {
            let card: u32 = card.try_into().unwrap();
            cards.insert(card+1, 1);
        }
    }

    for line in lines {
        let parts: Vec<_> = line.split(": ").collect();
        let card_number = parts[0].split_whitespace().collect::<Vec<_>>()[1].parse::<u32>().unwrap();

        let numbers: Vec<_> = parts[1].split("|").collect();
        let winning: HashSet<u32> = numbers[0].split_whitespace().map(|n| n.parse().unwrap()).collect();
        let have: HashSet<u32> = numbers[1].split_whitespace().map(|n| n.parse().unwrap()).collect();
        let matches: Vec<_> = winning.intersection(&have).collect();
        let matches_len: u32 = matches.len().try_into().unwrap();

        if part == 1 {
            if matches_len > 0 {
                sum += base.pow(matches_len -1);
            }
        } else {
            let how_many_we_have = *cards.get(&card_number).unwrap();
            for card in card_number..card_number + matches_len {
                cards.entry(card+1).and_modify(|c| *c+= how_many_we_have);
            }
        }
    }

    if part == 2 {
        return cards.values().sum();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = concat!(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n",
        );

        assert_eq!(y23d04(input, 1), 13);
        assert_eq!(y23d04(input, 2), 30);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day04.txt").unwrap();

        assert_eq!(y23d04(&contents, 1), 24706);
        assert_eq!(y23d04(&contents, 2), 13114317);
    }
}
