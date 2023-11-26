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

//! Advent of Code 2015 Day 8: <https://adventofcode.com/2015/day/8>
//!
//! Today's challenge was very simple. It essentially amounted to looping over
//! some strings with some counters and in a couple of cases changing how the
//! counters were incremented.

/// The solution for part one of the day eight challenge.
///
/// Given the input string we initialize the total sums of each type and then
/// loop over each string. The "code" representation we can find by simply
/// calling the `.len()` function (and converting it to a `u32`). To calculate
/// the "memory" representation we loop over all of the characters. If we have
/// previously encountered the escape character then we check if we have an `x`
/// which means a hexadecimal escape (and we need to skip the next two
/// characters) or not in which case we just move on without incrementing the
/// counter. If we have a skip value set then decrement it and move on without
/// incrementing the counter. If we encounter the `"` we don't increment the
/// counter because it's the start/end. Otherwise we increment the counter.
/// If we've encountered the escape character then trigger the behavior
/// described above for the next loop iteration. Finally, add the "memory" sum
/// to the running total and then return the "code" sum minus the "memory" sum
/// as requested by the prompt.
///
/// # Example
/// ```rust
/// # use aoc::y15d08::y15d08p1;
/// // probably read this from the input file...
/// let input = "\"a\"b\"\n\"\"\n\"\x23\"";
/// assert_eq!(y15d08p1(input), 7);
/// ```
pub fn y15d08p1(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut codesum = 0;
    let mut memsum = 0;

    for line in lines {
        let mut memlen: u32 = 0;
        let codelen: u32 = line.len().try_into().unwrap();
        codesum += codelen;

        let chars: Vec<_> = line.trim().chars().collect();
        let mut skip = 0;
        let mut escape = false;
        for c in chars {
            if escape {
                if c == 'x' {
                    // we got a hex escape, skip two more
                    skip = 2;
                    escape = false;
                    continue;
                } else {
                    escape = false;
                    continue;
                }
            }

            if skip > 0 {
                skip -= 1;
                continue;
            }

            if c == '"' {
                continue;
            }

            memlen += 1;

            if c == '\\' {
                escape = true;
            }
        }

        memsum += memlen;
    }

    codesum - memsum
}

/// The solution for part two of the day eight challenge.
///
/// Even easier than part one, we loop through all of the input lines
/// calculating the "code" sum the same as in part one (with the `.len()`
/// function). To calculate the "encoded" sum we start with `2` to account for
/// the new `"` that are added to the start and end of the string. Then we
/// loop through the characters and if we encounter the `"` or `\` characters
/// we increment by two to account for the escape otherwise we increment by
/// one as its standard. Add the tallies for the line to the totals and then
/// return the "encoded" sum minus the "code" sum as requested by the prompt.
///
/// # Example
/// ```rust
/// # use aoc::y15d08::y15d08p2;
/// // probably read this from the input file...
/// let input = "\"a\"b\"\n\"\"\n\"\x23\"";
/// assert_eq!(y15d08p2(input), 13);
/// ```
pub fn y15d08p2(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut codesum = 0;
    let mut encsum = 0;

    for line in lines {
        let mut enclen: u32 = 2;
        let codelen: u32 = line.len().try_into().unwrap();
        codesum += codelen;

        let chars: Vec<_> = line.trim().chars().collect();
        for c in chars {
            if c == '"' || c == '\\' {
                enclen += 2;
            } else {
                enclen += 1;
            }
        }

        encsum += enclen;
    }

    encsum - codesum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input =
            concat!("\"\"\n", "\"abc\"\n", "\"aaa\\\"aaa\"\n", "\"\\x27\"\n",);

        assert_eq!(y15d08p1(input), 12);
        assert_eq!(y15d08p2(input), 19);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day08.txt").unwrap();

        assert_eq!(y15d08p1(&contents), 1371);
        assert_eq!(y15d08p2(&contents), 2117);
    }
}
