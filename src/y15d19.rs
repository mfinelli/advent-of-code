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

//! Advent of Code 2015 Day 19: <https://adventofcode.com/2015/day/19>
//!
//! Today's challenge was quite hard. It took me a while to even figure out
//! exactly what I was supposed to be doing in the problem. Eventually, after
//! reading the prompt several more times and looking at some solutions on the
//! subreddit I was able to figure out enough to be able to implement my
//! solution.

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;

/// The solution for part one of the day nineteen challenge.
///
/// As usual we take the problem input as a string and then start parsing it.
/// The input contains all of the replacements followed by an empty line and
/// then the molecule. So we parse the replacements until we get to the empty
/// line and then the molecule is the final line. Once we have our replacements
/// we loop through them and then we loop and for each of them we loop through
/// the windows of the molecule of the size of the replacement so that we can
/// see if we can do a replacement. If we can then we build a new molecule
/// using the replacement and then add it to our tracking
/// [`std::collections::HashSet`] (we're only supposed to find distinct
/// molecules). Once we've checked all of the possibilities for all of the
/// replacements we just return the length of our set.
///
/// # Example
/// ```rust
/// # use aoc::y15d19::y15d19p1;
/// // probably read this from the input file...
/// let input = "Al => ThF\nAl => ThRnFAr\nB => BCa\nB => TiB\n\nnPBP";
/// assert_eq!(y15d19p1(input), 2);
/// ```
pub fn y15d19p1(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let num_replacements = lines.len() - 2;
    let molecule = lines[lines.len() - 1];
    let mut set = HashSet::new();

    let mut replacements = Vec::new();
    for (i, line) in lines.into_iter().enumerate() {
        if i >= num_replacements {
            break;
        }

        let text: Vec<_> = line.split_whitespace().collect();
        replacements.push((text[0], text[2]));
    }

    for (find, replace) in replacements {
        for (i, check) in molecule
            .chars()
            .collect::<Vec<char>>()
            .windows(find.len())
            .enumerate()
        {
            let check: String = check.iter().collect();
            if check == find {
                let new = format!(
                    "{}{}{}",
                    substring(molecule, 0, i),
                    replace,
                    substring(molecule, i + find.len(), molecule.len())
                );
                set.insert(new);
            }
        }
    }

    set.len().try_into().unwrap()
}

/// The solution for part two of the day nineteen challenge.
///
/// This was very challenging and I had to rely on inspiration from the
/// subreddit to figure out exactly how to solve this problem. As in part one
/// we start by parsing the problem input string to get our replacements and
/// molecule. Then we start a loop that we run until we reach `e`. The strategy
/// here is to work backwards from the end molecule to the start instead of
/// vice-versa as we'd need to implement a breadth-first-search or similar
/// which would quickly explode in how many checks that we need to do and how
/// long that it would take. Then we loop through all of the replacements
/// keeping track of how many times we're able to make a substitution. When we
/// can't make any more substitutions for that replacement (new == original)
/// then we move on to the next one. If, after processing all of the
/// replacements, the length of our "new" molecule after the replacements is
/// the same as the length of our "old" molecule before the replacements (i.e.,
/// we weren't able to do any replacements) then we reset our molecule
/// replacement string back to the original, reset our substitution counter,
/// shuffle the replacements array, and try again. This is the insight that I
/// was able to get from the subreddit, it seems that many people took a
/// similar approach, but as I understand it, it is technically possible to get
/// a non-optimal solution however, it seems that based on many peoples' inputs
/// there is actually only one path from `e` to the molecule so it should be
/// okay. Once we've reached `e` the loop ends and we can return the number of
/// substitutions that we made.
///
/// # Example
/// ```rust
/// # use aoc::y15d19::y15d19p2;
/// // probably read this from the input file...
/// let input = "e => A\ne => B\nA => BB\nB => AA\nB => AB\n\nAB";
/// assert_eq!(y15d19p2(input), 2);
/// ```
pub fn y15d19p2(input: &str) -> u32 {
    let mut rng = thread_rng();
    let lines: Vec<_> = input.lines().collect();
    let num_replacements = lines.len() - 2;
    let molecule = lines[lines.len() - 1].to_string();
    let mut reduced = molecule.clone();
    let mut count = 0;

    let mut replacements = Vec::new();
    for (i, line) in lines.into_iter().enumerate() {
        if i >= num_replacements {
            break;
        }

        let text: Vec<_> = line.split_whitespace().collect();
        replacements.push((text[0], text[2]));
    }

    while reduced != "e" {
        let start_len = reduced.len();

        for (find, replace) in &replacements {
            loop {
                let new = reduced.replacen(replace, find, 1);
                if new == reduced {
                    break;
                }
                count += 1;
                reduced = new;
            }
        }

        if start_len == reduced.len() {
            replacements.shuffle(&mut rng);
            reduced = molecule.clone();
            count = 0;
        }
    }

    count
}

/// This function simply returns a substring of the given size from the given
/// starting point as exists in the standard library of many other languages.
fn substring(string: &str, start: usize, end: usize) -> String {
    let s: String = string.chars().skip(start).take(end).collect();
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_substring() {
        let input = "ABCDE";
        assert_eq!(substring(input, 1, 2), "BC");
        assert_eq!(substring(input, 2, 2), "CD");
    }

    #[test]
    fn it_works() {
        let mut input = "H => HO\nH => OH\nO => HH\n\nHOHOHO\n";
        assert_eq!(y15d19p1(input), 7);

        input = "e => H\ne => O\nH => HO\nH => OH\nO => HH\n\nHOHOHO\n";
        assert_eq!(y15d19p2(input), 6);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day19.txt").unwrap();

        assert_eq!(y15d19p1(&contents), 509);
        assert_eq!(y15d19p2(&contents), 195);
    }
}
