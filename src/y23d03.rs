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

//! Advent of Code 2023 Day 3: <https://adventofcode.com/2023/day/3>
//!
//! TODO

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

/// The solution for the day three challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y23d03::y23d03;
/// // probably read this from the input file...
/// let input = "";
/// assert_eq!(y23d03(input), 0);
/// ```
pub fn y23d03(input: &str) -> u32 {
    // println!("{}", input);
    let lines: Vec<_> = input.lines().collect();
    let size = lines.len();
    let mut sum = 0;
    let mut grid = HashMap::new();
    // let mut s = HashSet::new();
    // let r = Regex::new(r"(\d+").unwrap();

    let numbers = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let symbols = ['/', '%', '#', '&', '+', '@', '-', '$', '*', '='];

    for (y, line) in lines.iter().enumerate() {
        // if let Some(cap) = r.captures(line) {
        //     println!("{:?}", cap);
        // }

        for (x, c) in line.chars().enumerate() {
            // s.insert(c);
            grid.insert((x as i32, y as i32), c);
        }
    }

    for i in 0..size {
        let y: i32 = i.try_into().unwrap();

        let mut in_number = false;
        let mut current_number = "".to_string();
        let mut number_start = 0;

        for j in 0..size {
            let x: i32 = j.try_into().unwrap();
            let c = grid.get(&(x, y)).unwrap();

            // println!("{}", c);

            if in_number {
                // println!("in number");
                if numbers.contains(c)  {
                    current_number = format!("{}{}", current_number, c);
                } else {
                    in_number = false;
                    // do the checks...
                    // println!("{:?}", current_number);

                    // for check_y in y-1..y+1 {
                    //     for check_x in number_start - 1..x+1 {
                    //         if let Some(check_c) = grid.get(&(check_x, check_y)) {

                    //         }
                    //     }
                    // }
                    // println!("{}", current_number);
                    if symbol_adjacent(&grid, number_start, x, y) {
                        // println!("{} is symbol adjacent", current_number);
                        sum += current_number.parse::<u32>().unwrap();
                    } else {
                        // println!("{} is not symbol-adjacent", current_number);
                    }

                    current_number = "".to_string();
                }
            } else {
                // println!("not in number");
                if numbers.contains(c) {
                    in_number = true;
                    current_number = c.to_string();
                    number_start = x;
                } else {
                    // println!("got: {}", c);
                }
            }
        }

        if in_number {
                    // println!("{}", current_number);
                    if symbol_adjacent(&grid, number_start, size as i32-1, y) {
                        sum += current_number.parse::<u32>().unwrap();
                    }
        }

        // break;
        // println!("");
    }
    // technically we should do one more check in case we finish on a number,
    // but I know that we don't by inspecting my input file (and the example)
    // println!("{:?}", s);

    sum
}

/// TODO
fn symbol_adjacent(grid: &HashMap<(i32, i32), char>, start_x: i32, end_x: i32, y: i32) -> bool {
                    // for check_y in y-1..y+1 {
                    //     for check_x in number_start - 1..x+1 {
                    //         if let Some(check_c) = grid.get(&(check_x, check_y)) {

                    //         }
                    //     }
    let symbols = ['/', '%', '#', '&', '+', '@', '-', '$', '*', '='];
                    // }
    // println!("given y: {}, y-1: {}, y+1: {}", y, y-1, y+1);
    for check_y in (y-1)..(y+2) {
        // println!("checking y: {}", check_y);
        for check_x in start_x-1..end_x+1 {
            if let Some(check_c) = grid.get(&(check_x, check_y)) {
                // println!("checking {}, {}: {}", check_x, check_y, check_c);
                if symbols.contains(check_c) {
                    return true;
                }
            }

        }
    }

    let mut p = "".to_string();
    for check_y in (y-1)..(y+2) {
        // println!("checking y: {}", check_y);
        for check_x in start_x-1..end_x+1 {
            match grid.get(&(check_x, check_y)) {
                // println!("checking {}, {}: {}", check_x, check_y, check_c);
                // if symbols.contains(check_c) {
                //     return true;
                // }
                Some(c) => {p = format!("{}{}", p, c);}
                None => {p=format!("{}{}", p, "X");}
            }

        }

        p = format!("{}{}", p, "\n");
    }
    // println!("{}", p);



    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn tit_works() {
        let input = concat!(
            "467..114..\n",
            "...*......\n",
            "..35..633.\n",
            "......#...\n",
            "617*......\n",
            ".....+.58.\n",
            "..592.....\n",
            "......755.\n",
            "...$.*....\n",
            ".664.598..\n",
        );

        assert_eq!(y23d03(input), 4361);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day03.txt").unwrap();

        assert_eq!(y23d03(&contents), 527144);
    }
}
