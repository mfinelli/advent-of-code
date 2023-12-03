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

use std::collections::HashMap;

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
    let lines: Vec<_> = input.lines().collect();
    let size = lines.len();
    let mut sum = 0;
    let mut grid = HashMap::new();

    let numbers = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
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

            if in_number {
                if numbers.contains(c) {
                    current_number = format!("{}{}", current_number, c);
                } else {
                    if symbol_adjacent(&grid, number_start, x, y) {
                        sum += current_number.parse::<u32>().unwrap();
                    }

                    in_number = false;
                    current_number = "".to_string();
                }
            } else if numbers.contains(c) {
                in_number = true;
                current_number = c.to_string();
                number_start = x;
            }
        }

        if in_number && symbol_adjacent(&grid, number_start, size as i32 - 1, y)
        {
            sum += current_number.parse::<u32>().unwrap();
        }
    }

    sum
}

/// TODO
pub fn y23d03p2(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let size: i32 = lines.len().try_into().unwrap();
    let mut sum = 0;
    let mut grid = HashMap::new();
    let mut stars = Vec::new();
    let mut numbers = Vec::new();

    // let numbers = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    for (y, line) in lines.iter().enumerate() {
        let mut in_number = false;
        let mut current_number = "".to_string();
        let mut number_start = 0;

        for (x, c) in line.chars().enumerate() {
            let x: i32 = x.try_into().unwrap();
            let y: i32 = y.try_into().unwrap();

            if in_number {
                if ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(&c) {
                    current_number = format!("{}{}", current_number, c);
                } else {
                    numbers.push((current_number.parse::<u32>().unwrap(), number_start, x-1, y));

                    in_number = false;
                    current_number = "".to_string();
                }
            } else if ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(&c) {
                in_number = true;
                current_number = c.to_string();
                number_start = x;
            }

            if c == '*' {
                stars.push((x,y));
            }

            grid.insert((x as i32, y as i32), c);
        }

        if in_number {
            numbers.push((current_number.parse::<u32>().unwrap(), number_start, size-1, y.try_into().unwrap()));
        }
    }

    // println!("{:?}", numbers);
    // println!("{:?}", stars);

    for star in stars {
        let mut neighbors = Vec::new();
        let (x, y) = star;

        for number in &numbers {
            let (num, start_x, end_x, num_y) = number;

            // if *num == 755 {
            //     println!("{:?}", number);
            //     println!("{:?}", star);

            //     println!("x: {}, start_x: {}, end_x: {}", x, start_x, end_x);
            //     println!("y: {}, y-1: {}, y+1: {}", y, num_y-1, num_y+1);
            // }

            if x >= *start_x -1 && x <= *end_x +1&& y >= num_y - 1 && y <= num_y +1 {
                neighbors.push(num);
            }
        }

        // println!("star at {},{} has neighbors {:?}", x, y, neighbors);

        if neighbors.len() == 2 {
            sum += neighbors[0] * neighbors[1];
        }

    }

    sum
}

/// TODO
fn symbol_adjacent(
    grid: &HashMap<(i32, i32), char>,
    start_x: i32,
    end_x: i32,
    y: i32,
) -> bool {
    let symbols = ['/', '%', '#', '&', '+', '@', '-', '$', '*', '='];

    for check_y in (y - 1)..(y + 2) {
        for check_x in start_x - 1..end_x + 1 {
            if let Some(check_c) = grid.get(&(check_x, check_y)) {
                if symbols.contains(check_c) {
                    return true;
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
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
        assert_eq!(y23d03p2(input), 467835);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day03.txt").unwrap();

        assert_eq!(y23d03(&contents), 527144);
        assert_eq!(y23d03p2(&contents), 81463996);
    }
}
