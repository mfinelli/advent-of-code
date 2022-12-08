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

//! Advent of Code 2022 Day 8: <https://adventofcode.com/2022/day/8>
//!
//! Overall this solution is a little bit ugly and a lot of brute force; there
//! are probably better ways to solve this. We start by parsing the input (we
//! do an initial pass `n*n` complexity) to convert the strings in the input
//! into a vector of integer vectors which saves us having to do the parse for
//! every comparison. Then we look at interior trees only. In part one we can
//! do a simple calculation based on the dimensions to account for them
//! without needing to loop through and account for them if x/y==0/len and in
//! part two we can ignore them because as edge trees they have one side with
//! a viewing distance of zero which when multiplied by any other viewing
//! distance will obviously result in a scenic score of zero too. For part two
//! we use the usual [`std::collections::BinaryHeap`] to keep track of the
//! tree with the highest scenic score value.

use std::collections::BinaryHeap;

/// The solution for the day eight challenge.
///
/// Given the input as a string and the part as an integer where `1` returns
/// the number of trees that are visible and `2` returns the highest possible
/// scenic score (viewing distance in all 4 directions multiplied together)
/// we compute the answer for both parts one and two at the same time and then
/// just return the desired answer.
///
/// We start by parsing the input into the vector of integer vectors as
/// described above. We then loop through all of the interior trees one at a
/// time. We look out in each direction (note the range reverse for looking
/// left and up) to determine how many trees we can see until we get a tree
/// that is taller or equal to the tree that we are inspecting (or the edge).
/// If we reach the edge then the tree is visible from that direction and we
/// can increase our visible tree counter. In part one, we could move on to
/// the next tree directly (`continue`) but in part two we need to calculate
/// the viewing distance from all directions. As we look at each other tree
/// we increase the viewing distance for that direction until we get to a
/// view-blocking tree or the edge. As described in the prompt the scenic
/// score is calculated by multiplying the four viewing distances together.
/// After inspecting all four directions we calculate the score and add it to
/// the scenic score heap. If we want the highest scenic score (part `2`) then
/// we pop the heap, otherwise we return the total visible trees.
///
/// # Example
/// ```rust
/// # use aoc::y22d08::y22d08;
/// // probably read this from the input file...
/// let input = "33333\n34223\n32123\n32223\n33333\n";
/// assert_eq!(y22d08(input, 1), 17);
/// assert_eq!(y22d08(input, 2), 9);
pub fn y22d08(input: &str, part: u32) -> u32 {
    let grid = parse_input(input);
    let mut scenic_scores = BinaryHeap::new();

    // calculate the outer edge which is always visible
    let mut total = 2 * grid.len() as u32 + 2 * (grid[0].len() as u32 - 2);

    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            // consider tree in position x,y: grid[y][x]

            let mut visible_from_left = true;
            let mut visible_to_left = 0;
            for left in (0..x).rev() {
                // compare to the left: left,y (grid[y][left])

                visible_to_left += 1;
                if grid[y][left] >= grid[y][x] {
                    visible_from_left = false;
                    break;
                }
            }

            if visible_from_left {
                total += 1;
            }

            let mut visible_from_right = true;
            let mut visible_to_right = 0;
            for right in x + 1..grid[0].len() {
                // compare to the right: right,y (grid[y][right])

                visible_to_right += 1;
                if grid[y][right] >= grid[y][x] {
                    visible_from_right = false;
                    break;
                }
            }

            // don't double count so check previous visibility
            if !visible_from_left && visible_from_right {
                total += 1;
            }

            let mut visible_from_top = true;
            let mut visible_to_top = 0;
            for top in (0..y).rev() {
                // compare to the top: x, top (grid[top][x])

                visible_to_top += 1;
                if grid[top][x] >= grid[y][x] {
                    visible_from_top = false;
                    break;
                }
            }

            // don't double count so check previous visibilities
            if !visible_from_left && !visible_from_right && visible_from_top {
                total += 1;
            }

            let mut visible_from_bottom = true;
            let mut visible_to_bottom = 0;
            for bottom in y + 1..grid.len() {
                // compare to the bottom: x, bottom (grid[bottom][x])

                visible_to_bottom += 1;
                if grid[bottom][x] >= grid[y][x] {
                    visible_from_bottom = false;
                    break;
                }
            }

            // don't double count so check previous visibilities
            if !visible_from_left
                && !visible_from_right
                && !visible_from_top
                && visible_from_bottom
            {
                total += 1;
            }

            let scenic_score = visible_to_left
                * visible_to_right
                * visible_to_top
                * visible_to_bottom;
            scenic_scores.push(scenic_score);
        }
    }

    if part == 1 {
        total
    } else {
        scenic_scores.pop().unwrap()
    }
}

/// Parsing the input requires an extra n^2 time complexity since we need to
/// loop through every element, but I think it should save time overall as
/// otherwise we need to parse the character every time we do a check.
fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let mut grid = Vec::new();
    let lines: Vec<_> = input.lines().collect();
    for line in lines {
        let mut trees = Vec::new();
        let chars: Vec<_> = line.chars().collect();

        for c in chars {
            let height: u32 = c.to_string().parse().unwrap();
            trees.push(height);
        }

        grid.push(trees);
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = "30373\n25512\n65332\n33549\n35390";
        assert_eq!(y22d08(input, 1), 21);
        assert_eq!(y22d08(input, 2), 8);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2022/day08.txt").unwrap();

        assert_eq!(y22d08(&contents, 1), 1807);
        assert_eq!(y22d08(&contents, 2), 480000);
    }
}
