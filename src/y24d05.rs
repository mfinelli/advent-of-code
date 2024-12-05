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

//! Advent of Code 2024 Day 5: <https://adventofcode.com/2024/day/5>
//!
//! TODO

use std::cmp::Ordering;

/// The solution for the day five challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y24d05::y24d05;
/// // probably read this from the input file...
/// let input = concat!(
/// );
/// assert_eq!(y24d05(input, 1), 0);
/// ```
pub fn y24d05(input: &str, part: u32) -> usize {
    let mut rules: Vec<(usize, usize)> = Vec::new();
    let mut jobs: Vec<Vec<usize>> = Vec::new();
    let mut seen_newline = false;
    let mut sum = 0;

    for line in input.lines().into_iter() {
        if line == "" {
            seen_newline = true;
            continue;
        }

        if !seen_newline {
            let rule: Vec<_> = line
                .split('|')
                .into_iter()
                .map(|i| i.parse().unwrap())
                .collect();
            rules.push((rule[0], rule[1]));
        } else {
            jobs.push(
                line.split(',')
                    .into_iter()
                    .map(|i| i.parse().unwrap())
                    .collect(),
            );
        }
    }

    for mut job in jobs {
        // if is_safe(&rules, job) {
        // println!("{:?} is safe", job);
        // }
        // match is_safe(&rules, &job) {
        //     Some(middle) => sum += middle,
        //     None => continue,
        // }
        if job.is_sorted_by(|a, b| !rules.contains(&(*b, *a))) {
            if part == 1 {
                sum += job[job.len() / 2];
            }
        } else if part == 2 {
            job.sort_by(|a, b| (!rules.contains(&(*b, *a))).cmp(&true));
            sum += job[job.len() / 2];
        }
    }

    sum
}

/// TODO
fn is_safe(rules: &Vec<(usize, usize)>, job: &Vec<usize>) -> Option<usize> {
    for (left_rule, right_rule) in rules {
        match job.iter().position(|i| i == left_rule) {
            Some(left) => match job.iter().position(|i| i == right_rule) {
                Some(right) => {
                    if right < left {
                        return None;
                    }
                }
                None => continue,
            },
            None => continue,
        }
    }

    Some(job[job.len() / 2])
}

// fn cmp() -> Ordering {
//     Ordering::Less
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_is_safe() {}

    #[test]
    fn it_works() {
        let input = concat!(
            "47|53\n",
            "97|13\n",
            "97|61\n",
            "97|47\n",
            "75|29\n",
            "61|13\n",
            "75|53\n",
            "29|13\n",
            "97|29\n",
            "53|29\n",
            "61|53\n",
            "97|53\n",
            "61|29\n",
            "47|13\n",
            "75|47\n",
            "97|75\n",
            "47|61\n",
            "75|61\n",
            "47|29\n",
            "75|13\n",
            "53|13\n",
            "\n",
            "75,47,61,53,29\n",
            "97,61,53,29,13\n",
            "75,29,13\n",
            "75,97,47,61,53\n",
            "61,13,29\n",
            "97,13,75,29,47\n",
        );
        assert_eq!(y24d05(input, 1), 143);
        assert_eq!(y24d05(input, 2), 123);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2024/day05.txt").unwrap();

        assert_eq!(y24d05(&contents, 1), 5064);
        assert_eq!(y24d05(&contents, 2), 5152);
    }
}
