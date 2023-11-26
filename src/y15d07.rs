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
//! Today's challenge was a little more difficult than I thought at the
//! beginning because it appears that the circuit can provide instructions
//! out-of-order. Once accounting for that possibility it was pretty
//! straightforward.

use regex::Regex;
use std::collections::HashMap;

/// The solution for the day seven challenge.
///
/// We take the usual input argument as a string as well as an argument for
/// the final wire signal that we're interested in. The final argument is for
/// part `1` (normal) or for part `2` (override the signal of wire `b` with
/// the signal of wire `a` calculated in part one).
///
/// We start by parsing our input and creating our
/// [`std::collections::HashMap`] that will store wires as keys and their
/// signals as the values. If we're in part two then we calculate the signal
/// for wire `a` in part one and then assign it to wire `b` immediately. This
/// works because we only set new values if they don't exist (implemented
/// during part one as it appeared that instructions could come out-of-order).
///
/// Now we start a loop that we'll use to run over the input until we've
/// accounted for all of the wires. Each iteration of the loop assumes that
/// we're done (so that it will exit when it's done) and then if we make _any_
/// changes (i.e., assign a signal to a wire) then we mark it as not done so
/// that we run the loop again. This lets us process the input even if there
/// are some wires that we can't calculate yet because we don't have all of
/// their input values.
///
/// From here it's pretty easy: figure out the wire to which we want to assign
/// a signal (always the final component of the instruction) and if we've
/// already assigned it a value then move on, otherwise figure out which
/// operation we want to perform and perform it. If we haven't calculated the
/// signal for any of the inputs then we skip it and try to calculate it again
/// when we come back around on the next loop iteration.
///
/// Finally, return the signal of the wire that we asked for as an argument.
///
/// **N.B.** the use of `u16` is important as it's specified in the prompt that
/// the integers are 16-bit which changes the values when performing bitwise
/// operations and shifts.
///
/// # Example
/// ```rust
/// # use aoc::y15d07::y15d07;
/// // probably read this from the input file...
/// let input = "12 -> c\n14 -> d\nc AND d -> b\nb -> a\n";
/// assert_eq!(y15d07(input, "a", 1), 12);
/// assert_eq!(y15d07(input, "a", 2), 12);
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

        assert_eq!(y15d07(input, "d", 1), 72);
        assert_eq!(y15d07(input, "e", 1), 507);
        assert_eq!(y15d07(input, "f", 1), 492);
        assert_eq!(y15d07(input, "g", 1), 114);
        assert_eq!(y15d07(input, "h", 1), 65412);
        assert_eq!(y15d07(input, "i", 1), 65079);
        assert_eq!(y15d07(input, "x", 1), 123);
        assert_eq!(y15d07(input, "y", 1), 456);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day07.txt").unwrap();

        assert_eq!(y15d07(&contents, "a", 1), 16076);
        assert_eq!(y15d07(&contents, "a", 2), 2797);
    }
}
