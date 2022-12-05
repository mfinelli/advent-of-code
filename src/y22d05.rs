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

//! Advent of Code 2022 Day 5: <https://adventofcode.com/2022/day/5>
//!
//! The day five challenge mostly revolves around vector manipulation (my
//! chosen methodology for managing the state of the crates). The most
//! challenging part was to parse the input text into the initial state
//! representation.
//!
//! Once the input has been parsed part one has us move crates one at a time
//! from the top (end) of one stack onto another i.e., [`std::vec::Vec::pop`]
//! from one vector and then [`std::vec::Vec::push`] onto another. The second
//! part requires maintaining the order of multiple crates as they are moved
//! from one stack to another, so we just move them one at a time into a
//! temporary vector, reverse the order of that and then play them back into
//! the destination vector.

/// The solution for the day five challenge.
///
/// As the arguments correspond to the input text and the method of operation:
/// the reordering of crates (part `1`) or the maintaining of the order (part
/// `2`).
///
/// # Parsing the Initial State
/// To parse the initial state we first need to get all of the lines before
/// the first empty line which we can treat as the delimiter between the state
/// representation and the moves that we will need to make. The last non-empty
/// line of the state representation gives us a numbering of the columns
/// (which we can use to determine the total number of columns) that needs to
/// be discarded as it is not actually a part of the internal state
/// representation. Once we have the lines that represent the state we can
/// loop through them (in reverse order so that the items on the bottom of the
/// stacks are the first items in the vector) checking each column to see if
/// there is an element defined there or not (e.g, empty space or end-of-input)
/// and if there is then add it to the appropriate column vector.
///
/// # Example
/// ```rust
/// # use aoc::y22d05::y22d05;
/// // probably read this in from the input file...
/// let input = concat![
///     "    [D]    \n",
///     "[N] [C]    \n",
///     "[Z] [M] [P]\n",
///     " 1   2   3 \n",
///     "\n",
///     "move 1 from 2 to 1\n",
///     "move 3 from 1 to 3\n",
///     "move 2 from 2 to 1\n",
///     "move 1 from 1 to 2\n",
/// ];
/// assert_eq!(y22d05(&input, 1), "CMZ");
/// assert_eq!(y22d05(&input, 2), "MCD");
/// ```
pub fn y22d05(input: &str, part: u32) -> String {
    let lines: Vec<_> = input.lines().collect();

    let mut state = parse_initial_state(&lines);
    let mut in_moves = false; // track if we've hit the state/moves separator
    let mut output = String::new();

    for line in lines {
        if in_moves {
            // "move X from Y to Z": X is index 0, Y is index 3, Z is index 5
            let text: Vec<&str> = line.split_whitespace().collect();
            let how_many_to_move = text[1].parse().unwrap();
            let from_index: u32 = text[3].parse().unwrap();
            let to_index: u32 = text[5].parse().unwrap();

            if part == 1 {
                for _ in 0..how_many_to_move {
                    let from: &mut Vec<String> =
                        &mut state[(from_index - 1) as usize];
                    let to_move = from.pop().unwrap();
                    let to: &mut Vec<String> =
                        &mut state[(to_index - 1) as usize];
                    to.push(to_move);
                }
            } else {
                let mut holding: Vec<String> = Vec::new();
                let from: &mut Vec<String> =
                    &mut state[(from_index - 1) as usize];
                for _ in 0..how_many_to_move {
                    let to_move = from.pop().unwrap();
                    holding.push(to_move);
                }

                let to: &mut Vec<String> = &mut state[(to_index - 1) as usize];
                for to_move in holding.into_iter().rev() {
                    to.push(to_move);
                }
            }
        } else if line.is_empty() {
            // we've hit the empty line, switch "modes"
            in_moves = true;
        } // else continue
    }

    for mut stack in state {
        // TODO: add test for empty column
        output += match &stack.pop() {
            Some(item) => item,
            None => " ",
        };
    }

    output
}

/// The process of processing the initial state is described in [`y22d05`]
/// under the "Parsing the Initial State" header.
fn parse_initial_state(lines: &Vec<&str>) -> Vec<Vec<String>> {
    let mut state_lines: Vec<&str> = Vec::new();
    let mut state: Vec<Vec<String>> = Vec::new();

    for line in lines {
        // once we find the empty line we _remove_ (pop) the previous line
        // that we saw because it's not actually part of the state
        // representation and then we parse it to figure out how many columns
        // we should have and then create the necessary empty vectors for each
        // stack.
        if line.is_empty() {
            let columns = state_lines.pop().unwrap();
            let number_of_columns: u32 =
                columns.split_whitespace().last().unwrap().parse().unwrap();

            for _ in 0..number_of_columns {
                state.push(Vec::new());
            }

            break;
        }

        // we haven't hit the empty line that splits the state and moves yet
        // so add this line to the "state_lines" that we'll process below
        state_lines.push(line);
    }

    // loop through the lines that represent the state in reverse order
    // (meaning that we start at the _bottom_ of the columns so that we can
    // add each element to the column vector
    for line in state_lines.iter().rev() {
        // whitespace doesn't extend to the end of each line for all columns
        // so we need to calculate how many actual columns are in a given line
        //
        // columns are represented by a length of 4, we add one to the line
        // length to compensate for the column spacer trimmed at the end of
        // the line
        let number_of_columns_in_line = (line.len() + 1) / 4;

        // loop through the columns, and if a column begins with a "["
        // character it means that the column contains an element, add it to
        // the current column
        for (i, column) in
            state.iter_mut().enumerate().take(number_of_columns_in_line)
        {
            if line.chars().nth(i * 4).unwrap() == '[' {
                column.push(line.chars().nth(i * 4 + 1).unwrap().to_string());
            }
        }
    }

    state
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_initial_state() {
        let input = concat!(
            "[A]     [B]\n",
            "[C] [D] [E] [F]\n",
            "[G] [H] [I] [J] [K]\n",
            " 1   2   3   4   5\n",
            "\n",
            "we don't care about this...\n",
            "or this...\n",
        );
        let lines = input.lines().collect();

        assert_eq!(
            parse_initial_state(&lines),
            vec![
                vec!["G", "C", "A"],
                vec!["H", "D"],
                vec!["I", "E", "B"],
                vec!["J", "F"],
                vec!["K"],
            ]
        );
    }

    #[test]
    fn it_works() {
        let input = concat!(
            "[A]\n",
            "[B] [C] [D]\n",
            " 1   2   3\n",
            "\n",
            "move 2 from 1 to 2\n",
        );

        assert_eq!(y22d05(input, 1), " BD");
        assert_eq!(y22d05(input, 2), " AD");
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2022/day05.txt").unwrap();

        assert_eq!(y22d05(&contents, 1), "QMBMJDFTD");
        assert_eq!(y22d05(&contents, 2), "NBTVTJNFJ");
    }
}
