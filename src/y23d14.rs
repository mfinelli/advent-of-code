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

//! Advent of Code 2023 Day 14: <https://adventofcode.com/2023/day/14>
//!
//! TODO

use std::collections::HashMap;

/// The solution for the day fourteen challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y23d14::y23d14;
/// // probably read this from the input file...
/// let input = "";
/// assert_eq!(y23d14(input), 0);
/// ```
pub fn y23d14(input: &str) -> i32 {
    let mut total = 0;
    let mut map = HashMap::new();

    let lines: Vec<_> = input.lines().collect();
    let rows: i32 = lines.len().try_into().unwrap();
    let mut cols = 0;

    for (y, line) in lines.iter().enumerate() {
        let y: i32 = y.try_into().unwrap();

        for (x, c) in line.chars().enumerate() {
            if y == 0 {
                cols = x;
            }

            let x: i32 = x.try_into().unwrap();

            map.insert((x, y), c);
        }
    }

    cols += 1;

    for x in 0..cols {
        let x: i32 = x.try_into().unwrap();

        for y in 0..rows {
            // let y: i32 = y.try_into().unwrap();
            let c = map.get(&(x,y)).unwrap();

            if *c == 'O' {
                let mut current = y;

                loop {
                    match map.get(&(x, current-1)) {
                        None => break,
                        Some(above) => {
                            if *above == 'O' || *above == '#' {
                                break;
                            }

                            // if x ==0{
                            //     println!("we can move {},{} to {},{}", x, y, x, y-1);
                            // }

                            map.insert((x, current-1), 'O');
                            map.insert((x, current), '.');

                            current -=1;
                        }
                    }
                }
            }
        }
    }

    // println!("{}", map.get(&(0,0)).unwrap());
    // println!("{}", map.get(&(0,1)).unwrap());
    // println!("{}", map.get(&(0,2)).unwrap());
    // println!("{}", map.get(&(0,3)).unwrap());
    // println!("{}", map.get(&(0,4)).unwrap());
    // println!("{}", map.get(&(0,5)).unwrap());
    // println!("{}", map.get(&(0,6)).unwrap());
    // println!("{}", map.get(&(0,7)).unwrap());
    // println!("{}", map.get(&(0,8)).unwrap());
    // println!("{}", map.get(&(0,9)).unwrap());

    for ((_,y), c) in map {
        if c == 'O' {
            total += rows-y;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn iit_works() {
        let input = concat!(
            "O....#....\n",
            "O.OO#....#\n",
            ".....##...\n",
            "OO.#O....O\n",
            ".O.....O#.\n",
            "O.#..O.#.#\n",
            "..O..#O..O\n",
            ".......O..\n",
            "#....###..\n",
            "#OO..#....\n",
        );

        assert_eq!(y23d14(input), 136);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day14.txt").unwrap();

        assert_eq!(y23d14(&contents), 109098);
    }
}
