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
//! TODO

/// The solution for the day eight challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y15d08::y15d08;
/// // probably read this from the input file...
/// let input = "";
/// ```
pub fn y15d08(input: &str) -> u32 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn tit_works() {
        let input =
            concat!("\"\"\n", "\"abc\"\n", "\"aaa\\\"aaa\"\n", "\"\\x27\"\n",);

        assert_eq!(y15d08(input), 12);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day08.txt").unwrap();

        assert_eq!(y15d08(&contents), 0);
    }
}
