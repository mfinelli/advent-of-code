/* Copyright 2022 Mario Finelli
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

//! Advent of Code 2015 Day 2: <https://adventofcode.com/2015/day/2>
//!
//! The solutions today are simple and just involve parsing numbers and
//! comparing them amongst each other. Originally this worked with some ugly
//! if/else statments and array sorting, but after solving the puzzle I
//! decided to switch it to use a [min
//! heap](https://en.wikipedia.org/wiki/Min-max_heap) which I think results
//! in a much cleaner and easier to understand/reason about solution.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// The solution for the day two challenge.
///
/// Given the input as a string we start by parsing all of the dimensions on
/// each line. Then if we're doing part `1` we calculate the area of each
/// side, add them to our heap and then pop the smallest value back. The
/// actual calculation comes directly from the challenge prompt:
/// `2*l*w + 2*w*h + 2*h*l` plus the extra (area of the smallest side that we
/// just got).
///
/// If we're doing part `2` then we calculate the cubic volume (`l * w * h`),
/// and then find the smallest two sides by adding the dimensions to the heap
/// and then popping the smallest two values back. The calculation again
/// comes directly from the prompt and is equal to the diameter of the
/// smallest side (two times the smallest two dimensions each) plus the volume
/// previously calculated.
///
/// # Example
/// ```rust
/// # use aoc::y15d02::y15d02;
/// let input = "1x2x3\n4x5x6\n"; // probably read this from the input file...
/// assert_eq!(y15d02(input, 1), 192);
/// assert_eq!(y15d02(input, 2), 150);
/// ```
///
/// # Min-Heap values
/// The rust documentation isn't exactly clear about how to work with a
/// min-heap which uses the same data structure as the max heap, but this
/// helpful answer on [stack overflow](https://stackoverflow.com/a/71121846)
/// provides the answer, reproduced here:
///
/// ```rust
/// # use std::cmp::Reverse;
/// let reversed = Reverse(42);
/// assert_eq!(reversed.0, 42);
/// // alternatively, using destructuring:
/// let Reverse(n) = reversed;
/// assert_eq!(n, 42);
/// ```
pub fn y15d02(input: &str, part: u32) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut total = 0;

    for line in lines {
        let dimensions: Vec<&str> = line.split('x').collect();
        let l: u32 = dimensions[0].parse().unwrap();
        let w: u32 = dimensions[1].parse().unwrap();
        let h: u32 = dimensions[2].parse().unwrap();

        if part == 1 {
            let lw = l * w;
            let wh = w * h;
            let hl = h * l;

            let mut heap =
                BinaryHeap::from([Reverse(lw), Reverse(wh), Reverse(hl)]);
            let Reverse(extra) = heap.pop().unwrap();

            total += 2 * lw + 2 * wh + 2 * hl + extra;
        } else {
            let volume = l * w * h;
            let mut heap =
                BinaryHeap::from([Reverse(l), Reverse(w), Reverse(h)]);

            let Reverse(s1) = heap.pop().unwrap();
            let Reverse(s2) = heap.pop().unwrap();

            total += 2 * s1 + 2 * s2 + volume;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let mut input = "2x3x4\n";
        assert_eq!(y15d02(input, 1), 58);
        assert_eq!(y15d02(input, 2), 34);

        input = "1x1x10";
        assert_eq!(y15d02(input, 1), 43);
        assert_eq!(y15d02(input, 2), 14);

        input = "2x3x4\n1x1x10\n";
        assert_eq!(y15d02(input, 1), 101);
        assert_eq!(y15d02(input, 2), 48);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day02.txt").unwrap();

        assert_eq!(y15d02(&contents, 1), 1606483);
        assert_eq!(y15d02(&contents, 2), 3842356);
    }
}
