/* Copyright 2023-2024 Mario Finelli
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

//! Advent of Code 2023 Day 13: <https://adventofcode.com/2023/day/13>
//!
//! Today's problem was fairly challenging and I spent quite a bit of time
//! figuring out exactly how to loop through all of the possible valid windows.
//! In part two the key insight is that compared to part one when checking the
//! equality of the rows/columns where there should be zero differences in
//! part two there should be exactly one (which means that we _shouldn't_ count
//! the old path from part one).

/// The solution for the day thirteen challenge.
///
/// We take the input as a string and a variable for the part that we're
/// solving (which we ultimately transform into the number of expected
/// differences when checking to see if we've found the path: `0` in part `1`
/// and `1` in part `2`). Then we split the input for each puzzle and parse it
/// into a vector of its rows and columns. We then loop through each set of
/// valid rows (i.e., that makes a valid path) and calculate all of the
/// parameters necessary to split the puzzle and compare the equality along the
/// dividing line. If we find the path then we increase the total accordingly.
/// Then we do the same process again for the columns.
///
/// # Example
/// ```rust
/// # use aoc::y23d13::y23d13;
/// // probably read this from the input file...
/// let input = concat!(
///     ".#.##.#.#\n",
///     ".##..##..\n",
///     ".#.##.#..\n",
///     "#......##\n",
///     "#......##\n",
///     ".#.##.#..\n",
///     ".##..##.#\n",
/// );
/// assert_eq!(y23d13(input, 1), 4);
/// assert_eq!(y23d13(input, 2), 400);
/// ```
pub fn y23d13(input: &str, part: u32) -> u32 {
    let mut total = 0;

    for puzzle in input.split("\n\n") {
        let puzzle = puzzle.trim();
        let rows: Vec<Vec<char>> =
            puzzle.lines().map(|l| l.chars().collect()).collect();
        let mut cols: Vec<Vec<char>> = Vec::new();

        for i in 0..rows[0].len() {
            let mut col = Vec::new();

            for row in &rows {
                col.push(row[i]);
            }

            cols.push(col);
        }

        let rows: Vec<String> =
            rows.iter().map(|r| r.iter().collect()).collect();
        let cols: Vec<String> =
            cols.iter().map(|c| c.iter().collect()).collect();

        let half = rows.len() / 2;
        let is_even = rows.len() % 2 == 0;

        for i in 1..rows.len() {
            let len = if i <= half {
                i * 2
            } else {
                (rows.len() - i) * 2
            };

            let offset = if is_even && i > half {
                (i - half) * 2
            } else if !is_even && i > half {
                (i - half) * 2 - 1
            } else {
                0
            };

            let lstart = offset;
            let lend = i;
            let rstart = i;
            let rend = if i > half { rows.len() } else { len };
            let left = rows[lstart..lend].to_vec();
            let right = rows[rstart..rend].to_vec();

            if are_equal(&left, &right, part - 1) {
                let index: u32 = i.try_into().unwrap();
                total += index * 100;
            }
        }

        let half = cols.len() / 2;
        let is_even = cols.len() % 2 == 0;

        for i in 1..cols.len() {
            let len = if i <= half {
                i * 2
            } else {
                (cols.len() - i) * 2
            };

            let offset = if is_even && i > half {
                (i - half) * 2
            } else if !is_even && i > half {
                (i - half) * 2 - 1
            } else {
                0
            };

            let lstart = offset;
            let lend = i;
            let rstart = i;
            let rend = if i > half { cols.len() } else { len };
            let left = cols[lstart..lend].to_vec();
            let right = cols[rstart..rend].to_vec();

            if are_equal(&left, &right, part - 1) {
                let index: u32 = i.try_into().unwrap();
                total += index;
            }
        }
    }

    total
}

/// This function takes the left and right halves (or top and bottom) of the
/// puzzle at the divided line and compares them to see if they're equal or
/// not. Because we need to find the smudge on the mirror in part two we
/// actually count the number of differences that we find if a particular line
/// is not equal and then return the equality based on the total number of
/// differences that we found.
fn are_equal(left: &[String], right: &[String], allowed: u32) -> bool {
    let mut diffs = 0;

    for i in 0..left.len() {
        if left[i] != right[right.len() - 1 - i] {
            let lchars: Vec<char> = left[i].chars().collect();
            let rchars: Vec<char> =
                right[right.len() - 1 - i].chars().collect();

            for j in 0..left[i].len() {
                if lchars[j] != rchars[j] {
                    diffs += 1;
                }
            }
        }

        if diffs > allowed {
            return false;
        }
    }

    diffs == allowed
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_are_equal() {
        let l = vec!["#...##..#".to_string()];
        let r = vec!["#....#..#".to_string()];
        assert!(!are_equal(&l, &r, 0));
        assert!(are_equal(&l, &r, 1));
    }

    #[test]
    fn it_works() {
        let input = concat!(
            "#.##..##.\n",
            "..#.##.#.\n",
            "##......#\n",
            "##......#\n",
            "..#.##.#.\n",
            "..##..##.\n",
            "#.#.##.#.\n",
            "\n",
            "#...##..#\n",
            "#....#..#\n",
            "..##..###\n",
            "#####.##.\n",
            "#####.##.\n",
            "..##..###\n",
            "#....#..#\n",
        );

        assert_eq!(y23d13(input, 1), 405);
        assert_eq!(y23d13(input, 2), 400);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day13.txt").unwrap();

        assert_eq!(y23d13(&contents, 1), 32035);
        assert_eq!(y23d13(&contents, 2), 24847);
    }
}
