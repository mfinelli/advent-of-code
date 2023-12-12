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

//! Advent of Code 2023 Day 12: <https://adventofcode.com/2023/day/12>
//!
//! TODO

use std::collections::HashMap;

/// The solution for the day twelve challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y23d12::y23d12;
/// // probably read this from the input file...
/// let input = "";
/// assert_eq!(y23d12(input), 0);
/// ```
pub fn y23d12(input: &str) -> usize {
    let mut sum = 0;
    // let mut cache: HashMap<u64, u64> = HashMap::new();

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let mut check: Vec<char> = parts[0].chars().collect();
        let numbers: Vec<usize> = parts[1].split(",").map(|n| n.parse().unwrap()).collect();
        check.push('.');
        sum += get_arrangements(check, &numbers, 0);
        // sum += get_arrangements(check, &numbers, &mut cache);
    }

    sum
}

/// TODO
fn get_arrangements(check: Vec<char>, numbers: &Vec<usize>, done: usize) -> usize{
    let mut total = 0;

    println!("checking {:?} {:?}, done: {}", check, numbers, done);

    if check.is_empty() {
        if numbers.is_empty() && done == 0{
            println!("we did this");
            return 1;
        } else {
            println!("we did this instead");
            return 0;
        }
    }

    let possible = if check[0] == '?' {
        ['.', '#'].to_vec()
    } else {
        [check[0]].to_vec()
    };

    for c in possible {
        if c == '#' {
            total += get_arrangements(check[1..].to_vec(), numbers, done +1);
        } else {
            if done != 0 {
                if !numbers.is_empty() && numbers[0] == done {
                    total += get_arrangements(check[1..].to_vec(), &numbers[1..].to_vec(), 0);
                }
            } else {
                total += get_arrangements(check[1..].to_vec(), &numbers, 0);
            }
        }
    }




    //
// fn get_arrangements(check: Vec<char>, numbers: &Vec<u64>, cache: &mut HashMap<u64, u64>) -> u64 {
    // let mut total = 0;

    // if check.is_empty() {
    //     if numbers.len() == 0 {
    //         return 1;
    //     } else {
    //         return 0;
    //     }
    // }

    // if numbers.len() == 0 {
    //     if check.contains(&'#') {
    //         return 0;
    //     } else {
    //         return 1;
    //     }
    // }

    // if check[0] == '.' || check[0] == '?' {
    //     total += get_arrangements(check[1..].to_vec(), &numbers, cache);
    // }

    // if (check[0] == '#' || check[0] == '?') &&
    //     numbers[0] <= check.len().try_into().unwrap() &&
    //     !check[..numbers[0].try_into().unwrap()].contains(&'.') &&
    //     (numbers[0] as usize == check.len() || check[numbers[0] as usize] != '#')
    // {
    //     total += get_arrangements(check[(numbers[0] as usize + 1)..].to_vec(), &numbers[1..].to_vec(), cache);
    // }

    // // if check.len() != 0 {
    // //     total += get_arrangements(check, numbers, cache);
    // // }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_get_arrangements() {
        let mut input = "???.###.".chars().collect();
        assert_eq!(get_arrangements(input, &vec![1, 1, 3], 0), 1);
    }

    #[test]
    fn iit_works() {
        let input = concat!(
            "???.### 1,1,3\n",
            ".??..??...?##. 1,1,3\n",
            "?#?#?#?#?#?#?#? 1,3,1,6\n",
            "????.#...#... 4,1,1\n",
            "????.######..#####. 1,6,5\n",
            "?###???????? 3,2,1\n",
        );

        assert_eq!(y23d12(input), 21);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day12.txt").unwrap();

        assert_eq!(y23d12(&contents), 7163);
    }
}
