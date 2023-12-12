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
//! Today's problem was very challenging. The solution ends up using recursion
//! to try all of the possibilities, and then in part two to avoid the runtime
//! exploding maintains a cache of seen values to check against instead of
//! recomputing each time (we actually then use this in part one too, but it's
//! not necessary). The key hint for me comes from a comment on the
//! [subreddit](https://www.reddit.com/r/adventofcode/comments/18ge41g/comment/kd0dw9e/)
//! which says to add a final, trailing '.' to each input string to help find
//! the end of each line of input.

use std::collections::HashMap;

/// The solution for the day twelve challenge.
///
/// We take the input as a string as usual and then the second parameter
/// specifies how many times we're supposed to expand the record: just once in
/// part one (i.e., the original input as-is) and five times in part two. We
/// then call our recursive counter for each of the records adding them to the
/// final sum and then we're done.
///
/// # Example
/// ```rust
/// # use aoc::y23d12::y23d12;
/// // probably read this from the input file...
/// let input = ".????..??#?. 3,2";
/// assert_eq!(y23d12(input, 1), 4);
/// assert_eq!(y23d12(input, 5), 1024);
/// ```
pub fn y23d12(input: &str, times: u32) -> usize {
    let mut sum = 0;
    let mut cache: HashMap<(Vec<char>, Vec<usize>, usize), usize> =
        HashMap::new();

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
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

/// This is the recursive function that actually solves the problem. We start
/// by returning the cache hit if we have one. Otherwise if we've reached the
/// end of the input we're done. If we haven't checked any numbers then we've
/// arrived at a single solution, otherwise we don't return any. If we haven't
/// reached the end of the input then we we either explode a given `?` into
/// its two possibilities or otherwise take it as-is. For each of the
/// possibilities (i.e., either the known character or both of the options for
/// an unknown character) we first check to see if we have a (potential)
/// damaged spring. If we do then we increment our checked counter and recurse
/// again skipping the character. If we don't have a damaged spring and we've
/// checked at least once possibility and we still have sizes to check and we
/// haven't checked enough numbers for the current size then we recurse again
/// for the next character and the next size. Otherwise, we recurse to the
/// next character for all of the given sizes. Finally, we insert an entry into
/// the cache for all of our inputs and then return the total.
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

/// This function unfolds the input the given number of times, inserting a `?`
/// between them as specified by the prompt.
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

        if i != times - 1 {
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
    fn it_works() {
        let input = concat!(
            "???.### 1,1,3\n",
            ".??..??...?##. 1,1,3\n",
            "?#?#?#?#?#?#?#? 1,3,1,6\n",
            "????.#...#... 4,1,1\n",
            "????.######..#####. 1,6,5\n",
            "?###???????? 3,2,1\n",
        );

        assert_eq!(y23d12(input, 1), 21);
        assert_eq!(y23d12(input, 5), 525152);
    }

    #[test]
    #[ignore]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day12.txt").unwrap();

        assert_eq!(y23d12(&contents, 1), 7163);
        assert_eq!(y23d12(&contents, 5), 17788038834112);
    }
}
