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
pub fn y15d07(input: &str, wire: &str) -> u16 {
    let lines: Vec<_> = input.lines().collect();
    let mut wires: HashMap<&str, u16> = HashMap::new();
    // let mut wires = HashMap::new();

    let r = Regex::new(r"^\d+ ").unwrap();

    for line in lines {
        let text: Vec<&str> = line.split_whitespace().collect();

        if r.is_match(line) {
            // it's a wire input, just add its value to the map

            let wire = text[2];
            let signal = text[0].parse().unwrap();
            wires.insert(wire, signal);
        } else {
            // it's an operation... so do the operation and then assign it

            if text[0] == "NOT" {
                let wire = text[3];
                let arg = text[1];

                wires.insert(wire, !wires.get(arg).unwrap());
            } else {
                let wire = text[4];
                let op = text[1];
                let arg1 = text[0];
                let arg2 = text[2];

                match op {
                    "AND" => {
                        wires.insert(wire, wires.get(arg1).unwrap() & wires.get(arg2).unwrap());
                    }
                    "OR" => {
                        wires.insert(wire, wires.get(arg1).unwrap() | wires.get(arg2).unwrap());
                    }
                    "LSHIFT" => {
                        let howmuch: u16 = arg2.parse().unwrap();
                        wires.insert(wire, wires.get(arg1).unwrap() << howmuch);
                    }
                    "RSHIFT" => {
                        let howmuch: u16 = arg2.parse().unwrap();
                        wires.insert(wire, wires.get(arg1).unwrap() >> howmuch);
                    }
                    _ => {
                        panic!("Unkown operation!");
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

        assert_eq!(y15d07(&contents, "a"), 0);
    }
}
