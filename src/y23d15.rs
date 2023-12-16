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
//! Today is a very straightforward challenge that just requires following the
//! instructions exactly as they're defined in the prompt.

use std::collections::HashMap;

/// The solution for the day fifteen challenge.
///
/// We take the input as usual along with the normal integer to determine part
/// `1` or part `2`. In part one we just split the input by "," and then
/// calculate the hash for each string. For part two we instead build two
/// vectors to keep track of the current values of each label (using
/// [`std::collections::HashMap`]) as well as the orders of the labels in a
/// separate vector. This could probably be simplified to use just a single
/// vector with a tuple value. After we process the input step-by-step all that
/// is left is to calculate the power based on the prompt instructions.
///
/// # Example
/// ```rust
/// # use aoc::y23d15::y23d15;
/// // probably read this from the input file...
/// let input = "ab=1,cd=2,ef=3,cd-,ab=4";
/// assert_eq!(y23d15(input, 1), 746);
/// assert_eq!(y23d15(input, 2), 628);
/// ```
pub fn y23d15(input: &str, part: u32) -> u32 {
    let mut total = 0;

    if part == 1 {
        for step in input.trim().split(',') {
            total += hash(step);
        }

        return total;
    }

    let mut boxes = Vec::new();
    let mut orders = Vec::new();

    for _ in 0..256 {
        let o: Vec<&str> = Vec::new();
        let b = HashMap::new();

        boxes.push(b);
        orders.push(o);
    }

    for step in input.trim().split(',') {
        if step.contains('=') {
            let parts: Vec<_> = step.split('=').collect();
            let label = parts[0];
            let focal_length: u32 = parts[1].parse().unwrap();

            let b: usize = hash(label).try_into().unwrap();
            boxes[b].insert(label, focal_length);

            if !orders[b].contains(&label) {
                orders[b].push(label);
            }
        } else {
            let parts: Vec<_> = step.split('-').collect();
            let label = parts[0];

            let b: usize = hash(label).try_into().unwrap();
            orders[b].retain(|l| l != &label);
        }
    }

    for (b, order) in orders.iter().enumerate() {
        let i: u32 = b.try_into().unwrap();

        for (j, label) in order.iter().enumerate() {
            let j: u32 = j.try_into().unwrap();
            total += (i + 1) * (j + 1) * boxes[b].get(label).unwrap();
        }
    }

    total
}

/// This function computes the hash for a given string based on the
/// instructions in the prompt
fn hash(step: &str) -> u32 {
    let mut val = 0;

    for c in step.chars() {
        let c: u8 = c.try_into().unwrap();
        val += u32::from(c);
        val *= 17;
        val %= 256;
    }

    val
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn it_works() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\n";

        assert_eq!(y23d15(input, 1), 1320);
        assert_eq!(y23d15(input, 2), 145);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day15.txt").unwrap();

        assert_eq!(y23d15(&contents, 1), 517965);
        assert_eq!(y23d15(&contents, 2), 267372);
    }
}
