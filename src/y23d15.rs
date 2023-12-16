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

//! Advent of Code 2023 Day 15: <https://adventofcode.com/2023/day/15>
//!
//! TODO

/// The solution for the day fifteen challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y23d15::y23d15;
/// // probably read this from the input file...
/// let input = "";
/// assert_eq!(y23d15(input), 0);
/// ```
pub fn y23d15(input: &str) -> u32 {
    let mut total = 0;

    for step in input.trim().split(",") {
        let mut val: u32 = 0;

        for c in step.chars() {
            let c: u8 = c.try_into().unwrap();
            val += u32::from(c);
            val *= 17;
            val %= 256;
        }

        total += val;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn iit_works() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\n";

        assert_eq!(y23d15(input), 1320);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day15.txt").unwrap();

        assert_eq!(y23d15(&contents), 517965);
    }
}
