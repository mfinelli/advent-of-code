/* Copyright 2024 Mario Finelli
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

//! Advent of Code 2024 Day 4: <https://adventofcode.com/2024/day/4>
//!
//! TODO

/// The solution for the day four challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y24d04::y24d04;
/// // probably read this from the input file...
/// let input = concat!(
///     "..X...\n",
///     ".SAMX.\n",
///     ".A..A.\n",
///     "XMAS.S\n",
///     ".X....\n",
/// );
/// assert_eq!(y24d04(input), 0);
/// ```
pub fn y24d04(input: &str) -> u32 {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let lines: Vec<_> = input.lines().collect();
    for line in lines {
        let chars: Vec<_> = line.chars().collect();
        grid.push(chars);
    }

    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'X' {
                if y >= 3 && grid[y-1][x] == 'M' && grid[y-2][x] == 'A' && grid[y-3][x] == 'S' {
                    // from X straight up
                    sum += 1;
                }

                if y >=3 && x < grid[y].len() - 3 && grid[y-1][x+1] == 'M' && grid[y-2][x+2] == 'A' && grid[y-3][x+3] == 'S' {
                    // from X up and to the right
                    sum += 1;
                }

                if x < grid[y].len() - 3 && grid[y][x+1] == 'M' && grid[y][x+2] == 'A' && grid[y][x+3] == 'S' {
                    // from X to the right
                    sum += 1;
                }

                if y < grid.len() - 3 && x < grid[y].len() - 3 && grid[y+1][x+1] == 'M' && grid[y+2][x+2] == 'A' && grid[y+3][x+3] == 'S' {
                    // from x to the right and down
                    sum += 1;
                }

                if y < grid.len() -3 && grid[y+1][x] == 'M' && grid[y+2][x] == 'A' && grid[y+3][x] == 'S' {
                    // from X straight down
                    sum += 1;
                }

                if y < grid.len() - 3 && x >=3 && grid[y+1][x-1] == 'M' && grid[y+2][x-2] == 'A' && grid[y+3][x-3] == 'S' {
                    // from X down and to the left
                    sum += 1;
                }

                if x >= 3 && grid[y][x-1] == 'M' && grid[y][x-2] == 'A' && grid[y][x-3] == 'S' {
                    // from X to the left
                    sum += 1;
                }

                if y >= 3 && x >= 3 && grid[y-1][x-1] == 'M' && grid[y-2][x-2] == 'A' && grid[y-3][x-3] == 'S' {
                    // from X to the left and up
                    sum += 1;
                }
            }
        }
    }


    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = concat!(
            "MMMSXXMASM\n",
            "MSAMXMSMSA\n",
            "AMXSXMAAMM\n",
            "MSAMASMSMX\n",
            "XMASAMXAMM\n",
            "XXAMMXXAMA\n",
            "SMSMSASXSS\n",
            "SAXAMASAAA\n",
            "MAMMMXMMMM\n",
            "MXMXAXMASX",
        );
        // let input = concat!(
        //     "X...\n",
        //     ".M..\n",
        //     "..A.\n",
        //     "...S\n",
        // );
        assert_eq!(y24d04(input), 18);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2024/day04.txt").unwrap();

        assert_eq!(y24d04(&contents), 0);
    }
}
