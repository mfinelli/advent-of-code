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

//! Advent of Code 2022 Day 10: <https://adventofcode.com/2022/day/10>
//!
//! Because the number of numbers and total states is so small I decided to
//! simply compute the state of the register at each cycle and then analyze it
//! afterwards for each part of the challenge. We basically start with the
//! register `x` at `1` and then if we see a `noop` push it into the cycles
//! vector as-is, otherwise compute the change that's being made and push it
//! once as-is, and then again with the adjustment (the instructions state
//! that the `addx` operation takes two cycles). This gives us a vector with
//! the state of the `x` register at each cycle during the "program"'s
//! execution.
//!
//! In part one we then need to calculate a sum based on the state of the
//! register at various points (cycle `20` and then every `40` cycles after).
//! And in part two we instead need to draw the output based on the state of
//! the register. It actually took me a while to figure out what the prompt
//! was actually asking for so I'll try to reproduce it here in a hopefully
//! easier way for me to understand.
//!
//! The drawing operation works like this: we draw from left to right and from
//! top to bottom one pixel at a time. We know whether to draw a `#` or a `.`
//! depending on the state of the register for that pixel (the index on the
//! cycles vector is `row * 40 + column`. The "sprite" is the value of the
//! register plus or minus one, and so if the current column falls into the
//! range of `x - 1` or `x + 1` then we draw the `#` character, otherwise the
//! `.` character.

use std::cmp::Ordering;

/// The solution for part one of the day ten challenge.
///
/// This is relatively straightforward after computing the state of the
/// register for each cycle. It just involves a little bit of math. The signal
/// strength is computed by the value of the register at cycle `20` and then
/// again every `40` cycles thereafter. So we start by calculating how many
/// additional checks we need to make (the length of the cycles vector minus
/// `20` to account for the first check divided by `40`). Then for each check
/// we add to the signal strength the value of the register times the cycle
/// (index/number).
///
/// # Example
/// ```rust
/// # use aoc::y22d10::y22d10p1;
/// // probably read this from the input file...
/// let input = "noop\naddx 3\naddx -5";
/// assert_eq!(y22d10p1(input), 0);
/// ```
pub fn y22d10p1(input: &str) -> i32 {
    let lines: Vec<_> = input.lines().collect();
    let cycles = compute_cycles(lines);

    // this isn't strictly necessary because the program input as provided by
    // the prompt should always have 240 cycles so that we can draw the output
    // in part two
    match cycles.len().cmp(&20) {
        Ordering::Less => return 0,
        Ordering::Equal => return cycles[19] * 20,
        Ordering::Greater => {}
    }

    // signal strength is calculated by the cycle at position 20 and then
    // again every 40 cycles thereafter
    let signal_strength_checks = (cycles.len() - 20) / 40;
    let mut signal_strength = cycles[19] * 20;

    for i in 0..signal_strength_checks {
        let cycle = (i + 1) * 40 + 20;
        signal_strength += cycles[cycle - 1] * cycle as i32;
    }

    signal_strength
}

/// The solution for part two of the day ten challenge.
///
/// I initially found the part two prompt somewhat challenging because I had
/// a hard time understanding exactly how the drawing process worked based on
/// the given instructions. After figuring it out it also became pretty
/// straightforward.
///
/// Once we compute the cycles (we need to have 240 of them for the `6x40`
/// size screen defined in the prompt) we commence the drawing process: go
/// pixel by pixel filling in each column left to right starting at the top
/// row and then proceeding to the next row after filling in all of the
/// columns. At each pixel we stop to consider the state of the register at
/// that point in time (each pixel corresponds to on cycle so rows > 0 mean
/// that we do the cycle lookup by multiplying the current row by `40`). If
/// the current column falls in the range of register +/-1 then we draw the
/// `#` character, otherwise the `.` character.
///
/// # Example
/// ```rust
/// # use aoc::y22d10::y22d10p2;
/// // probably read this from the input file...
/// let input = "noop\n".repeat(240);
/// assert_eq!(y22d10p2(&input),
///     "###.....................................\n".repeat(6).trim()
/// );
/// ```
pub fn y22d10p2(input: &str) -> String {
    let lines: Vec<_> = input.lines().collect();
    let cycles = compute_cycles(lines);

    if cycles.len() != 241 {
        panic!("Input is wrong size!");
    }

    let mut output = String::new();

    for row in 0..6 {
        for column in 0..40 {
            let x = cycles[row * 40 + column];
            if column as i32 >= x - 1 && column as i32 <= x + 1 {
                output += "#";
            } else {
                output += ".";
            }
        }
        output += "\n";
    }

    output.trim().to_string()
}

/// This function computes the state of the register `x` at each cycle as
/// described above and by the challenge prompt. Simply it starts at value `1`
/// and then depending on the instruction pushes either one or two states onto
/// the cycles vector. Add operations take two cycles so we push the original
/// state and the new state, otherwise (`noop` operation) we just push the
/// current state.
fn compute_cycles(lines: Vec<&str>) -> Vec<i32> {
    let mut x = 1;
    let mut cycles = Vec::new();
    cycles.push(x);

    for line in lines {
        if line == "noop" {
            cycles.push(x);
        } else {
            let parts: Vec<_> = line.split_whitespace().collect();
            let addx: i32 = parts[1].parse().unwrap();

            cycles.push(x);
            x += addx;
            cycles.push(x);
        }
    }

    cycles
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = concat!(
            "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\n",
            "addx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\n",
            "addx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\n",
            "addx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\n",
            "addx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\n",
            "addx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\n",
            "addx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\n",
            "addx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\n",
            "addx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\n",
            "addx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\n",
            "noop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\n",
            "addx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\n",
            "noop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\n",
            "addx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\n",
            "addx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\n",
            "noop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\n",
            "addx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\n",
            "addx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\n",
            "addx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop\n",
        );

        let p2output = concat!(
            "##..##..##..##..##..##..##..##..##..##..\n",
            "###...###...###...###...###...###...###.\n",
            "####....####....####....####....####....\n",
            "#####.....#####.....#####.....#####.....\n",
            "######......######......######......####\n",
            "#######.......#######.......#######.....",
        );

        assert_eq!(y22d10p1(input), 13140);
        assert_eq!(y22d10p2(input), p2output);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2022/day10.txt").unwrap();

        let p2output = concat!(
            "####.#..#.###..#..#.####.###..#..#.####.\n",
            "#....#.#..#..#.#..#.#....#..#.#..#....#.\n",
            "###..##...#..#.####.###..#..#.#..#...#..\n",
            "#....#.#..###..#..#.#....###..#..#..#...\n",
            "#....#.#..#.#..#..#.#....#....#..#.#....\n",
            "####.#..#.#..#.#..#.####.#.....##..####.",
        );

        assert_eq!(y22d10p1(&contents), 14560);
        assert_eq!(y22d10p2(&contents), p2output);
    }
}
