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

//! Advent of Code 2023 Day 1: <https://adventofcode.com/2023/day/1>
//!
//! TODO

use regex::Regex;

/// The solution for part one of the day one challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y23d01::y23d01p1;
/// // probably read this from the input file...
/// let input = "";
/// assert_eq!(y23d01p1(input), 0);
/// ```
pub fn y23d01p1(input: &str) -> u32 {
    let mut total = 0;
    let numbers = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    for line in input.lines() {
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;

        for c in line.chars() {
            if numbers.contains(&c) {
                first = Some(c);
                break;
            }
        }

        for c in line.chars().rev() {
            if numbers.contains(&c) {
                last = Some(c);
                break;
            }
        }

        let number = format!("{}{}", first.unwrap(), last.unwrap());
        total += number.parse::<u32>().unwrap();
    }

    total
}

/// The solution for part two of the day one challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y23d01::y23d01p2;
/// // probably read this from the input file...
/// let input = "";
/// assert_eq!(y23d01p2(input), 0);
/// ```
pub fn y23d01p2(input: &str) -> u32 {
    let mut total = 0;
    let numbers = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    // let first_match = Regex::new(r"^(zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    // let last_match = Regex::new(r"^(orez|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();

    for line in input.lines() {
        let mut first: Option<String> = None;
        let mut last: Option<String> = None;

        let new = line.replace("zero", "zero0zero").replace("one", "one1one").replace("two", "two2two").replace("three", "three3three").replace("four", "four4four").replace("five", "five5five").replace("six", "six6six").replace("seven", "seven7seven").replace("eight", "eight8eight").replace("nine", "nine9nine");

        total += y23d01p1(&new);

        // println!("{:?}", line);
        // let window = if line.len() < 5 {
        //     line.len()-1
        // } else {
        //     5
        // };

        // for w in line.chars().collect::<Vec<char>>().windows(window) {
        //     // println!("{:?}", w);

        //     // println!("{:?}", w[0]);
        //     if numbers.contains(&w[0]) {
        //         // println!("numbers contains {}", w[0]);
        //         first = Some(format!("{}", w[0]));
        //         break;
        //     }

        //     // let s = format!("{}{}{}{}{}", w[0], w[1], w[2], w[3], w[4]);
        //     let s: String = w.iter().collect();
        //     // println!("{:?}", s);

        //     if let Some(capture) = first_match.captures(&s) {
        //         match &capture[1]{
        //             "one" => {
        //                 first = Some("1".to_string());
        //                 break;
        //             }
        //             "two" => {
        //                 first = Some("2".to_string());
        //                 break;
        //             }
        //             "three" => {
        //                 first = Some("3".to_string());
        //                 break;
        //             }
        //             "four" => {
        //                 first = Some("4".to_string());
        //                 break;
        //             }
        //             "five" => {
        //                 first = Some("5".to_string());
        //                 break;
        //             }
        //             "six" => {
        //                 first = Some("6".to_string());
        //                 break;
        //             }
        //             "seven" => {
        //                 first = Some("7".to_string());
        //                 break;
        //             }
        //             "eight" => {
        //                 first = Some("8".to_string());
        //                 break;
        //             }
        //             "nine" => {
        //                 first = Some("9".to_string());
        //                 break;
        //             }
        //             _ => panic!("unexpected match")
        //         }
        //     }
        //     // if first_match.is_match(&s) {
        //     //     // let captures = first_match.captures
        //     //     println!("{:?}", first_match.captures(&s));
        //     // }

            

        // }

        // for w in line.chars().rev().collect::<Vec<char>>().windows(window) {
        //     // println!("{:?}", w);

        //     if numbers.contains(&w[0]) {
        //         last = Some(format!("{}", w[0]));
        //         break;
        //     }

        //     // let s = format!("{}{}{}{}{}", w[0], w[1], w[2], w[3], w[4]);
        //     let s: String = w.iter().collect();
        //     // println!("{:?}", s);

        //     if let Some(capture) = last_match.captures(&s) {
        //         // println!("{:?}", capture);
        //         match &capture[1]{
        //             "eno" => {
        //                 last = Some("1".to_string());
        //                 break;
        //             }
        //             "owt" => {
        //                 last = Some("2".to_string());
        //                 break;
        //             }
        //             "eerht" => {
        //                 last = Some("3".to_string());
        //                 break;
        //             }
        //             "ruof" => {
        //                 last = Some("4".to_string());
        //                 break;
        //             }
        //             "evif" => {
        //                 last = Some("5".to_string());
        //                 break;
        //             }
        //             "xis" => {
        //                 last = Some("6".to_string());
        //                 break;
        //             }
        //             "neves" => {
        //                 last = Some("7".to_string());
        //                 break;
        //             }
        //             "thgie" => {
        //                 last = Some("8".to_string());
        //                 break;
        //             }
        //             "enin" => {
        //                 last = Some("9".to_string());
        //                 break;
        //             }
        //             _ => panic!("unexpected match")
        //         }
        //     }
        // }


        // // println!("{}", line);
        // // println!("{}{}", first.unwrap(), last.unwrap());


        // if first == None {
        //     // println!("{} has no first", line);
        //     // first = Some(line.chars().collect::<Vec<char>>()[line.len()-1].to_string());
        //     // println!("{} new first", first.clone().unwrap());
        //     for c in line.chars() {
        //         if numbers.contains(&c) {
        //             first = Some(format!("{}", c));
        //             break;
        //         }
        //     }
        // }

        // if last == None {
        //     // println!("{} has no last", line);
        //     // last = Some(line.chars().collect::<Vec<char>>()[0].to_string());
        //     // println!("{} new last", last.clone().unwrap());
        //     for c in line.chars().rev() {
        //         if numbers.contains(&c) {
        //             last = Some(format!("{}", c));
        //             break;
        //         }
        //     }
        // }

        // // println!("{}", line);
        // // println!("{}{}", first.clone().unwrap(), last.clone().unwrap());

        // let number = format!("{}{}", first.unwrap(), last.unwrap());
        // total += number.parse::<u32>().unwrap();
        // // break;

    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let mut input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n";
        assert_eq!(y23d01p1(input), 142);

        input = concat!(
            "two1nine\n",
            "eightwothree\n",
            "abcone2threexyz\n",
            "xtwone3four\n",
            "4nineeightseven2\n",
            "zoneight234\n",
            "7pqrstsixteen\n",
        );
        assert_eq!(y23d01p2(input), 281);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day01.txt").unwrap();

        assert_eq!(y23d01p1(&contents), 55712);
        assert_eq!(y23d01p2(&contents), 55413);
    }
}
