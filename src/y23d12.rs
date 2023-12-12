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
/// let input = ".????..??#?. 3,2";
/// assert_eq!(y23d12(input, 1), 4);
/// assert_eq!(y23d12(input, 2), 1024);
/// ```
pub fn y23d12(input: &str, part: u32) -> usize {
    let mut sum = 0;
    let mut cache: HashMap<(Vec<char>, Vec<usize>, usize), usize> =
        HashMap::new();

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let times = if part == 1 { 1 } else { 5 };
        let (mut springs, sizes) = unfold_record(
            parts[0].chars().collect(),
            parts[1].split(',').map(|n| n.parse().unwrap()).collect(),
            times,
        );

        // this is the trick to determine end-of-input
        springs.push('.');

        sum += get_arrangements(springs, &sizes, 0, &mut cache);
    }

    sum
}

/// TODO
fn get_arrangements(
    springs: Vec<char>,
    sizes: &Vec<usize>,
    checked: usize,
    cache: &mut HashMap<(Vec<char>, Vec<usize>, usize), usize>,
) -> usize {
    if let Some(hit) = cache.get(&(springs.clone(), sizes.clone(), checked)) {
        return *hit;
    }

    let mut total = 0;

    if springs.is_empty() {
        if sizes.is_empty() && checked == 0 {
            return 1;
        } else {
            return 0;
        }
    }

    let possible = if springs[0] == '?' {
        ['.', '#'].to_vec()
    } else {
        [springs[0]].to_vec()
    };

    for c in possible {
        if c == '#' {
            total += get_arrangements(
                springs[1..].to_vec(),
                sizes,
                checked + 1,
                cache,
            );
        } else if checked != 0 {
            if !sizes.is_empty() && sizes[0] == checked {
                total += get_arrangements(
                    springs[1..].to_vec(),
                    &sizes[1..].to_vec(),
                    0,
                    cache,
                );
            }
        } else {
            total += get_arrangements(springs[1..].to_vec(), sizes, 0, cache);
        }
    }

    cache.insert((springs.clone(), sizes.clone(), checked), total);

    total
}

/// TODO
fn unfold_record(
    chars: Vec<char>,
    numbers: Vec<usize>,
    times: u32,
) -> (Vec<char>, Vec<usize>) {
    let mut new_chars = Vec::new();
    let mut new_numbers = Vec::new();

    for i in 0..times {
        for c in &chars {
            new_chars.push(*c);
        }

        if times != 1 && i != times - 1 {
            new_chars.push('?');
        }

        for n in &numbers {
            new_numbers.push(*n);
        }
    }

    (new_chars, new_numbers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_get_arrangements() {
        let mut input = "???.###.".chars().collect();
        let mut cache = HashMap::new();
        assert_eq!(get_arrangements(input, &vec![1, 1, 3], 0, &mut cache), 1);

        input = "???.###????.###????.###????.###????.###.".chars().collect();
        let mut cache = HashMap::new();
        assert_eq!(
            get_arrangements(
                input,
                &vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3],
                0,
                &mut cache
            ),
            1
        );
    }

    #[test]
    fn test_unfold_record() {
        let input: Vec<char> = ".#".chars().collect();

        assert_eq!(
            unfold_record(input.clone(), vec![1], 1),
            (vec!['.', '#'], vec![1])
        );

        assert_eq!(
            unfold_record(input.clone(), vec![1], 5),
            (
                vec![
                    '.', '#', '?', '.', '#', '?', '.', '#', '?', '.', '#', '?',
                    '.', '#'
                ],
                vec![1, 1, 1, 1, 1]
            )
        );
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

        assert_eq!(y23d12(input, 1), 21);
        assert_eq!(y23d12(input, 2), 525152);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day12.txt").unwrap();

        assert_eq!(y23d12(&contents, 1), 7163);
        assert_eq!(y23d12(&contents, 2), 17788038834112);
    }
}
