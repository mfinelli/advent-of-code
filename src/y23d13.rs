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

//! Advent of Code 2023 Day 13: <https://adventofcode.com/2023/day/13>
//!
//! TODO

/// The solution for the day thirteen challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y23d13::y23d13;
/// // probably read this from the input file...
/// let input = "";
/// assert_eq!(y23d13(input), 0);
/// ```
pub fn y23d13(input: &str) -> u32 {
    let mut total = 0;

    for puzzle in input.split("\n\n") {
        let mut found = false;
        let puzzle = puzzle.trim();
        // println!("puzzle: {:?}", puzzle);
        let rows: Vec<Vec<char>> = puzzle.lines().map(|l| l.chars().collect()).collect();
        // println!("rows: {:?}", rows);
        let mut cols: Vec<Vec<char>> = Vec::new();
        for i in 0..rows[0].len() {
            let mut col = Vec::new();

            for j in 0..rows.len() {
                col.push(rows[j][i]);
            }

            cols.push(col);
        }

        let rows: Vec<String> = rows.iter().map(|r| r.iter().collect()).collect();
        let cols: Vec<String> = cols.iter().map(|c| c.iter().collect()).collect();

        // println!("puzzle:\n{}", puzzle);
        // println!("rows: {:?}", rows);
        // println!("cols: {:?}", cols);

        // for i in 1..rows.len() {
        //     // println!("checking: {:?}, {:?}", vec![rows[i]], vec![rows[i-1]]);
        //     let right = vec![&rows[i]];
        //     let left = vec![&rows[i-1]];


        //     println!("left: {:?}, right: {:?}", left, right);
        // }

        let half = rows.len()/2;
        let is_even = if rows.len()%2 == 0 { true } else { false};
        for i in 1..rows.len() {
            let len =  if i <= half { i*2} else { (rows.len()-i)*2};
            let offset = if is_even && i>half {
                (i-half)*2
            } else if !is_even && i>half {
                (i-half)*2-1
            } else {
                0
            };
            // let lstart = if i <= half { 0 } else { (i-half)*2 + offset};
            let lstart = offset;
            let lend = i;
            let rstart = i;
            let rend = if i > half { rows.len() } else { len };
            // if i == 6 || i == 7 || i ==8 {
            // println!("i: {}, len: {}, offset: {}, left: {}..{}, right: {}..{}", i, len, offset, lstart, lend, rstart, rend);
            // }
            //
            let left = rows[lstart..lend].to_vec();
            let right = rows[rstart..rend].to_vec();

            if left.len() != right.len() {
                println!("length mismatch on ^^");
            }

            // println!("comparing: {:?}, {:?}", left, right);
            if are_equal(&left, &right, false) {
                found = true;
                // println!("rows are equal at index: {}", i);
                let index: u32 = i.try_into().unwrap();
                total += index*100;
            }


        }

        let half = cols.len()/2;
        let is_even = if cols.len()%2 == 0 { true } else { false};
        for i in 1..cols.len() {
            let len =  if i <= half { i*2} else { (cols.len()-i)*2};
            let offset = if is_even && i>half {
                (i-half)*2
            } else if !is_even && i>half {
                (i-half)*2-1
            } else {
                0
            };
            // let lstart = if i <= half { 0 } else { (i-half)*2 + offset};
            let lstart = offset;
            let lend = i;
            let rstart = i;
            let rend = if i > half { cols.len() } else { len };
            // println!("i: {}, len: {}, offset: {}, left: {}..{}, right: {}..{}", i, len, offset, lstart, lend, rstart, rend);
            //
            let left = cols[lstart..lend].to_vec();
            let right = cols[rstart..rend].to_vec();

            // println!("comparing: {:?}, {:?}", left, right);
            if are_equal(&left, &right, false) {
                found = true;
                // println!("cols are equal at index: {}", i);
                let index: u32 = i.try_into().unwrap();
                total += index;
            }


        }

        // if !found {
        //     println!("didn't find anything:\n{}", puzzle);
        // }

        // println!("");


    }

    total
}

/// TODO
fn are_equal(left: &Vec<String>, right: &Vec<String>, debug: bool) -> bool {
    for i in 0..left.len() {
        if debug {
            println!("checking {:?} against {:?}", left[i], right[right.len()-1-i]);
        }
        if left[i] != right[right.len()-1-i] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_are_equal(){}

    #[test]
    fn tit_works() {
        let input = concat!(
            // "#.##..##..\n",
            // "..#.##.#..\n",
            // "##......#.\n",
            // "##......#.\n",
            // "..#.##.#..\n",
            // "..##..##..\n",
            // "#.#.##.#..\n",
            // "..........\n",



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

        assert_eq!(y23d13(input), 405);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day13.txt").unwrap();

        assert_eq!(y23d13(&contents), 32035);
    }
}
