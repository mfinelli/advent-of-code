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

use std::collections::HashMap;

/// The solution for the day fifteen challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y23d15::y23d15;
/// // probably read this from the input file...
/// let input = "";
/// assert_eq!(y23d15(input, 1), 0);
/// assert_eq!(y23d15(input, 2), 0);
/// ```
pub fn y23d15(input: &str, part: u32) -> u32 {
    let mut total = 0;

    if part == 1 {
        for step in input.trim().split(",") {
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

    for step in input.trim().split(",") {
        if step.contains("=") {
            let parts: Vec<_> = step.split("=").collect();
            let label = parts[0];
            let focal_length: u32 = parts[1].parse().unwrap();

            let b: usize = hash(label).try_into().unwrap();
            boxes[b].insert(label, focal_length);

            if !orders[b].contains(&label) {
                orders[b].push(label);
            }
        } else {
            let parts: Vec<_> = step.split("-").collect();
            let label = parts[0];

            let b: usize = hash(label).try_into().unwrap();
            orders[b].retain(|l| l != &label);
        }
    }

    // println!("{:?}", orders);
    // println!("{:?}", boxes);

    for (b, order) in orders.iter().enumerate() {
        let i: u32 = b.try_into().unwrap();

        // let mut power = i+1;
        for (j, label) in order.iter().enumerate() {
            let j: u32 = j.try_into().unwrap();
            // power *= j+1;
            // power *= boxes[b].get(label).unwrap();

            // println!("power for {} ({} * {} * {}): {}", label, i+1, j+1, boxes[b].get(label).unwrap(), power);
            total += (i+1) * (j+1) * boxes[b].get(label).unwrap();
        }

        // total += power;
    }

    total
}

/// TODO
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
