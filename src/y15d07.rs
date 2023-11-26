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

//! Advent of Code 2015 Day 7: <https://adventofcode.com/2015/day/7>
//!
//! TODO

use regex::Regex;
use std::collections::HashMap;

/// The solution for the day seven challenge.
///
/// TODO
///
/// # Example
/// ```rust
///
/// ```
pub fn y15d07(input: &str, wire: &str, part: u32) -> u16 {
    let lines: Vec<_> = input.lines().collect();
    let mut wires: HashMap<&str, u16> = HashMap::new();

    if part == 2 {
        wires.insert("b", y15d07(input, "a", 1));
    }

    let r = Regex::new(r"^\d+ ").unwrap();
    let mut done = false;

    while !done {
        // presume that we finished...
        done = true;

        for line in &lines {
            let text: Vec<&str> = line.split_whitespace().collect();
            let wire = text.last().unwrap();

            match wires.get(wire) {
                Some(_) => continue,
                None => {
                    // we're making a change, so run through again
                    done = false;

                    if text.len() == 3 {
                        // simple assignment
                        if r.is_match(line) {
                            let signal = text[0].parse().unwrap();
                            wires.insert(wire, signal);
                        } else {
                            match wires.get(text[0]) {
                                Some(signal) => {
                                    wires.insert(wire, *signal);
                                }
                                None => continue,
                            }
                        }
                    } else if text[0] == "NOT" {
                        match wires.get(text[1]) {
                            Some(signal) => {
                                wires.insert(wire, !signal);
                            }
                            None => continue,
                        }
                    } else if text[1] == "AND" || text[1] == "OR" {
                        let lparse = &text[0].parse();
                        let leftarg = match lparse {
                            Ok(arg) => arg,
                            Err(_) => match wires.get(text[0]) {
                                Some(signal) => signal,
                                None => continue,
                            },
                        };

                        let rparse = &text[2].parse();
                        let rightarg = match rparse {
                            Ok(arg) => arg,
                            Err(_) => match wires.get(text[2]) {
                                Some(signal) => signal,
                                None => continue,
                            },
                        };

                        if text[1] == "AND" {
                            wires.insert(wire, leftarg & rightarg);
                        } else {
                            wires.insert(wire, leftarg | rightarg);
                        }
                    } else if text[1] == "LSHIFT" || text[1] == "RSHIFT" {
                        let by: u16 = text[2].parse().unwrap();

                        match wires.get(text[0]) {
                            Some(signal) => {
                                if text[1] == "LSHIFT" {
                                    wires.insert(wire, signal << by);
                                } else {
                                    wires.insert(wire, signal >> by);
                                }
                            }
                            None => continue,
                        }
                    } else {
                        panic!("Unsupported operation!");
                    }
                }
            }
        }
    }

    *wires.get(wire).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = concat!(
            "123 -> x\n",
            "456 -> y\n",
            "x AND y -> d\n",
            "x OR y -> e\n",
            "x LSHIFT 2 -> f\n",
            "y RSHIFT 2 -> g\n",
            "NOT x -> h\n",
            "NOT y -> i",
        );

        assert_eq!(y15d07(input, "d"), 72);
        assert_eq!(y15d07(input, "e"), 507);
        assert_eq!(y15d07(input, "f"), 492);
        assert_eq!(y15d07(input, "g"), 114);
        assert_eq!(y15d07(input, "h"), 65412);
        assert_eq!(y15d07(input, "i"), 65079);
        assert_eq!(y15d07(input, "x"), 123);
        assert_eq!(y15d07(input, "y"), 456);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day07.txt").unwrap();

        assert_eq!(y15d07(&contents, "a", 1), 16076);
        assert_eq!(y15d07(&contents, "a", 2), 2797);
    }
}
