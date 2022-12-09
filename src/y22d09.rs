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

//! Advent of Code 2022 Day 9: <https://adventofcode.com/2022/day/9>
//!
//! The logic in this solution is a bit verbose and could probably be
//! consolidated some, but it's easier for me to grok expanded out. I
//! generalized the solution to operate on an arbitrary number of knots, as
//! part one of the challenge can be thought of as a rope with only two knots.
//! Obviously, this doesn't work if the number of knots is less than two.
//!
//! To keep track of which positions the tail knot has visited we use a
//! [`std::collections::HashSet`] with tuples of `x`,`y` coordinates so that
//! at the end we can just return the size of the set to get the total number
//! of locations visited.

use std::collections::HashSet;

/// The solution for the day nine challenge.
///
/// We expect the input as a string and the number of knots in the rope (as
/// described above the minimum number of knots is `2`).
///
/// We start by initializing a vector that contains tuples of coordinates for
/// each knot in the rope; each knot starts at coordinates `(0, 0)`. We also
/// add the starting position to the visited set. Then we loop over the
/// instructions (lines). We do the following process `x` times where `x` is
/// the number of moves that the instruction specified. We loop through all of
/// the knots and if we're on the first knot then we apply the specified move.
/// Then for every knot _except_ the actual tail knot (and including the head
/// knot) we need to reconcile the position of the knot directly following the
/// current knot. As described in the challenge prompt the two knots must
/// always be touching (including diagonally) which amounts to doing some
/// simple checks on the positions of the two knots and adjusting the tail knot
/// accordingly. If we're on the final (tail) knot then we just need to insert
/// it's current position into the set. Finally, after looping through all of
/// the instructions we can return the length of the set to get our answer.
///
/// # Example
/// ```rust
/// # use aoc::y22d09::y22d09;
/// // probably read this from the input file...
/// let input = "U 2\nR 2\nU 2\nD 3\nL 4";
/// assert_eq!(y22d09(input, 3), 4);
/// ```
pub fn y22d09(input: &str, number_of_knots: u32) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut visited = HashSet::new();
    let mut knots = Vec::new();

    // set starting position
    for _ in 0..number_of_knots {
        knots.push((0, 0));
    }
    let tail_index = knots.len() - 1;
    visited.insert(knots[tail_index]);

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let steps: u32 = parts[1].parse().unwrap();

        for _ in 0..steps {
            for i in 0..knots.len() {
                let (mut hx, mut hy) = knots[i];

                if i == 0 {
                    match parts[0] {
                        "U" => hy += 1,
                        "D" => hy -= 1,
                        "L" => hx -= 1,
                        "R" => hx += 1,
                        _ => panic!("Invalid direction!"),
                    }

                    knots[i] = (hx, hy);
                }

                if i != tail_index {
                    let (mut tx, mut ty) = knots[i + 1];
                    (tx, ty) = reconcile_tail(hx, hy, tx, ty);
                    knots[i + 1] = (tx, ty);
                }

                if i == tail_index {
                    visited.insert(knots[i]);
                }
            }
        }
    }

    visited.len() as u32
}

/// Given an updated head position and the current tail position calculate if
/// tail needs to move and return its updated position.
fn reconcile_tail(hx: i32, hy: i32, tx: i32, ty: i32) -> (i32, i32) {
    if hx == tx && hy == ty {
        // head is overlapping tail; tail doesn't move
        return (tx, ty);
    }

    // head and tail share a row or column
    if hy == ty && hx > tx + 1 {
        // head has moved to the right
        return (tx + 1, ty);
    } else if hy == ty && hx < tx - 1 {
        // head has moved to the left
        return (tx - 1, ty);
    } else if hx == tx && hy > ty + 1 {
        // head has moved up
        return (tx, ty + 1);
    } else if hx == tx && hy < ty - 1 {
        // head has moved down
        return (tx, ty - 1);
    }

    // head and tail are on different rows and/or columns
    if hx > tx + 1 && hy > ty {
        // head is above tail and moved to the right;
        // tail moves up and to the right
        return (tx + 1, ty + 1);
    } else if hx > tx + 1 && hy < ty {
        // head is below tail and moved to the right;
        // tail moves down and to the right
        return (tx + 1, ty - 1);
    } else if hx < tx - 1 && hy > ty {
        // head is above tail and moved to the left;
        // tail moves up and to the left
        return (tx - 1, ty + 1);
    } else if hx < tx - 1 && hy < ty {
        // head is below tail and moved to the left;
        // tail moves down and to the left
        return (tx - 1, ty - 1);
    } else if hy > ty + 1 && hx > tx {
        // head is to the right of tail and moved up;
        // tail moves up and to the right
        return (tx + 1, ty + 1);
    } else if hy > ty + 1 && hx < tx {
        // head is to the left of tail and moved up;
        // tail moves up and to the left
        return (tx - 1, ty + 1);
    } else if hy < ty - 1 && hx > tx {
        // head is to the right of tail and moved down
        // tail moves down and to the right
        return (tx + 1, ty - 1);
    } else if hy < ty - 1 && hx < tx {
        // head is to the left of tail and moved down
        // tail moves down and to the left
        return (tx - 1, ty - 1);
    }

    // head and tail are still touching; tail doesn't move
    (tx, ty)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let mut input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n";
        assert_eq!(y22d09(input, 2), 13);
        assert_eq!(y22d09(input, 10), 1);

        input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";
        assert_eq!(y22d09(input, 10), 36);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2022/day09.txt").unwrap();

        assert_eq!(y22d09(&contents, 2), 6243);
        assert_eq!(y22d09(&contents, 10), 2630);
    }
}
