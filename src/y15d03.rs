/* Copyright 2022 Mario Finelli
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

//! Advent of Code 2015 Day 3: <https://adventofcode.com/2015/day/3>
//!
//! I originally misread the instructions for this challenge and thought that
//! the prompt wanted to know how many houses received _more than one_ present
//! but after getting the wrong answer realized that the prompt simply wanted
//! to know the total number of houses that received a present. I decided to
//! keep the initial implementation that tracks the number of presents that a
//! house receives, but it may be more efficient or performant to ignore how
//! many presents a house receives and just keep a list of coordinates that
//! have been visited using e.g., a [`std::collections::HashSet`].
//!
//! As it is I'm using a [`std::collections::HashMap`] with tuples of `x`,`y`
//! coordinates as the key to store the number of presents delivered to the
//! house at those coordinates. The answer to the challenge then comes from
//! counting the number of entries in the collection. More advanced answers
//! (e.g., the number of houses receiving more than one present) can be
//! determined by analyzing the same result collection.

use std::collections::HashMap;

/// The solution for the day three challenge.
///
/// The function takes the input as a string and the number of Santa's that are
/// delivering presents as the second argument assuming that the instructions
/// are consumed n-at-a-time where n is the number of Santa's (as described in
/// the challenge for `2` Santa's). The difference between part one and two of
/// the challenge is therefore the number of Santa's participating: `1` or `2`.
///
/// The solution is to split the input into number-of-santas-sized chunks and
/// then loop over each chunk. Then, for each chunk we loop through each Santa
/// and check the instruction that they have using their index as an index in
/// the given chunk. We adjust their position based on the character
/// (instruction that we found) and then increment (or initialize to `1`
/// present) the house corresponding to their position. Finally we return the
/// length of the houses collection as the response to the challenge.
///
/// A more naïve approach could have been to make the Santa loop the outer
/// loop and the chunk loop the inner loop i.e., use a single position tracker
/// (similar to the original solution to part one of the challenge) and then
/// reset it for each Santa and then use each Santa "i" as the index to pull
/// out of each chunk as we loop through the chunks collection -- but this
/// results in looping through the chunks loop santa-times instead of just
/// once through, though it hardly makes a difference for the small inputs and
/// number of Santa's that we actually have.
///
/// # Example
/// ```rust
/// # use aoc::y15d03::y15d03;
/// let input = "^<>^vv<>"; // probably read this from the input file...
/// assert_eq!(y15d03(input, 1), 5);
/// assert_eq!(y15d03(input, 2), 6);
/// ```
pub fn y15d03(input: &str, santas: u32) -> u32 {
    let chars: Vec<_> = input.trim().chars().collect();
    let mut houses = HashMap::new();
    houses.insert((0, 0), santas); // starting position gets a present

    // initialize each santa in their starting position
    let mut positions = Vec::new();
    for _ in 0..santas {
        positions.push((0, 0));
    }

    for chunk in chars.chunks(santas as usize) {
        for santa in 0..santas {
            let (mut x, mut y) = positions[santa as usize];
            let c = chunk[santa as usize];

            if c == '<' {
                x -= 1;
            } else if c == '^' {
                y += 1;
            } else if c == '>' {
                x += 1;
            } else {
                y -= 1;
            }

            // write santa's new position back to the position tracker
            positions[santa as usize] = (x, y);

            let count = houses.entry((x, y)).or_insert(0);
            *count += 1;
        }
    }

    houses.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let mut input = ">\n";
        assert_eq!(y15d03(input, 1), 2);

        input = "^>v<";
        assert_eq!(y15d03(input, 1), 4);
        assert_eq!(y15d03(input, 2), 3);

        input = "^v^v^v^v^v\n";
        assert_eq!(y15d03(input, 1), 2);
        assert_eq!(y15d03(input, 2), 11);

        input = "^v";
        assert_eq!(y15d03(input, 2), 3);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day03.txt").unwrap();

        assert_eq!(y15d03(&contents, 1), 2565);
        assert_eq!(y15d03(&contents, 2), 2639);
    }
}
