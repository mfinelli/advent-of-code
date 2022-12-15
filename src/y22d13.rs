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

//! Advent of Code 2022 Day 13: <https://adventofcode.com/2022/day/13>
//!
//! I today's prompt to be rather challenging. Not because the rules were
//! particularly complicated, but it took me a while to figure out how to
//! represent arbitrarily nested, mixed (list of more of self or numeric) data
//! types into a single vector that would capture each packet. In the end the
//! solution was to use an `enum` type (that I called `Data`) that can by the
//! `T` in a `Vec<T>` that is either another vector of Data items or a numeric
//! primitive. Note that I actually used a [`std::collections::VecDeque`] to
//! represent the list of other Data types as when doing the comparisons we
//! want to look at the front of the lists first. Using a Deque and then
//! mutating it to perform the comparisons is fundamentally the wrong approach
//! (sorting a list of elements shouldn't also _modify_ those elements) but it
//! does allow me to simply pop and element from both lists at the same time
//! and compare if I got back `Some` or `None` without needing to resort to
//! index and length checking before attempting to read each element. However,
//! this also caused problems in part two, as I couldn't just run a normal
//! sort on an array of the packets, so instead I kept track of the original
//! string packet and then parsed it into the Data representation and ran its
//! order check on each pass of the loop inside of a custom [insertion
//! sort](https://en.wikipedia.org/wiki/Insertion_sort) loop. I chose insertion
//! sort as it's easy to implement and even though it has a bad worst case
//! complexity the actual data set is small enough that I don't expect properly
//! implementing quicksort or similar to make an actual difference.

use std::cmp::Ordering;
use std::collections::VecDeque;

/// We use the Data type to hold either a vector of other Data elements or the
/// number primitive. Putting it in an enum lets us use a single vector to
/// handle both types.
#[derive(Debug, PartialEq)]
enum Data {
    V(VecDeque<Data>),
    N(u32),
}

/// The solution for the day thirteen challenge.
///
/// We take the usual input as a string and whether we're solving for part `1`
/// (sum of the indices that are in order) or part `2` (product of the indices
/// of the two extra data packets that we add).
///
/// We start by splitting the input every three lines so that we check each
/// pair if it's ordered or not, we also add the string representation of both
/// to our vector that we'll sort later if we're computing part `2`. If we're
/// computing part `1` then we need to parse the input into a `Data`
/// representation and then check if the pairs are ordered. If they are then
/// we add one to our index and then add that to the sum of ordered indices.
///
/// After going through all of the pairs and we're doing part `1` then we're
/// done - just return the result. Otherwise we're doing part two and we need
/// to add the extra data packets and then sort all of them. As explained above
/// I implemented a simple insertion sort (while additionally parsing the input
/// for each comparison pass because of the state mutation) to achieve this.
/// After sorting we do one final loop through the input to calculate the
/// product of the indices of the extra packets we added and then we're done.
///
/// # Example
/// ```rust
/// # use aoc::y22d13::y22d13;
/// // probably read this from the input file...
/// let input = "[1,2]\n[3,[4]]\n\n[[5]]\n[]\n";
/// assert_eq!(y22d13(input, 1), 1);
/// assert_eq!(y22d13(input, 2), 18);
/// ```
pub fn y22d13(input: &str, part: u32) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut data = Vec::new();
    let mut result = 0;

    for (i, packet_pair) in lines.chunks(3).enumerate() {
        data.push(packet_pair[0]);
        data.push(packet_pair[1]);

        if part == 1 {
            let first_packet_raw: Vec<_> = packet_pair[0].chars().collect();
            let second_packet_raw: Vec<_> = packet_pair[1].chars().collect();

            let first_packet = parse_packet(first_packet_raw);
            let second_packet = parse_packet(second_packet_raw);

            match in_order(first_packet, second_packet) {
                Some(ordered) => {
                    if ordered {
                        result += i as u32 + 1;
                    }
                }
                None => panic!("Couldn't reach a decision!"),
            }
        }
    }

    if part == 1 {
        return result;
    }

    // we're strictly in part two here
    data.push("[[2]]");
    data.push("[[6]]");

    // insertion sort can be O(n^2) but we have such a small dataset that it's
    // fine; it's not worth the effort to implement a more efficient sorting
    // algorithm
    for i in 1..data.len() {
        let mut j = i;
        while j > 0
            && !in_order(
                parse_packet(data[j - 1].chars().collect()),
                parse_packet(data[j].chars().collect()),
            )
            .unwrap()
        {
            data.swap(j - 1, j);
            j -= 1;
        }
    }

    result = 1;
    for (i, d) in data.iter().enumerate() {
        if d == &"[[2]]" || d == &"[[6]]" {
            result *= (i as u32) + 1;
        }
    }

    result
}

/// Given two Data elements determines if they are in order based on the rules
/// from the challenge prompt. Getting back `Some` means that we have a
/// definitive answer, getting back `None` means that we need to keep
/// processing within the current loop.
fn in_order(a: Data, b: Data) -> Option<bool> {
    match a {
        Data::V(mut a) => match b {
            Data::V(mut b) => loop {
                // both a and b are lists, loop through them and compare

                let a_item = a.pop_front();
                let b_item = b.pop_front();

                match a_item {
                    Some(a_item) => match b_item {
                        Some(b_item) => match in_order(a_item, b_item) {
                            Some(v) => return Some(v),
                            None => continue,
                        },
                        None => {
                            // b ran out of items before a; they're not ordered
                            return Some(false);
                        }
                    },
                    None => {
                        // a ran out of items before b; if b still has items
                        // they they're ordered, if b also ran out of items
                        // then keep processing

                        if b_item.is_some() {
                            return Some(true);
                        } else {
                            return None;
                        }
                    }
                }
            },
            Data::N(b) => {
                // a is a list, b is not - convert it to a list and try again
                let mut b_as_v = VecDeque::new();
                b_as_v.push_back(Data::N(b));
                in_order(Data::V(a), Data::V(b_as_v))
            }
        },
        Data::N(a) => match b {
            Data::V(b) => {
                // b is a list, a is not - convert it to a list and try again
                let mut a_as_v = VecDeque::new();
                a_as_v.push_back(Data::N(a));
                in_order(Data::V(a_as_v), Data::V(b))
            }
            Data::N(b) => match a.cmp(&b) {
                Ordering::Less => Some(true),
                Ordering::Greater => Some(false),
                Ordering::Equal => None,
            },
        },
    }
}

/// Parse a data packet (given as characters) and return a its Data
/// representation.
fn parse_packet(chars: Vec<char>) -> Data {
    let mut number_builder = String::new();
    let mut array_stack: Vec<Data> = Vec::new();
    let mut current_array = Data::V(VecDeque::new());

    // skip the first character as input must always be a list and we
    // initialized the root vector above
    for c in chars.iter().skip(1) {
        if c == &'[' {
            // we found a new nested vector, put the current vector onto the
            // stack and then set the current vector to a new, empty vector
            array_stack.push(current_array);
            current_array = Data::V(VecDeque::new());
        } else if c == &']' {
            if !number_builder.is_empty() {
                // we found the end of an array, and we previously found some
                // numbers, parse them into an integer and then add it to the
                // current vector
                let number: u32 = number_builder.parse().unwrap();
                number_builder = String::new();

                let arr = if let Data::V(ref mut arr) = current_array {
                    arr
                } else {
                    panic!("vector will always be a Vector")
                };
                arr.push_back(Data::N(number));
            } else {
                // we found the end of an array, and we did not previously
                // find any numbers, close the array, pop the parent vector
                // off the stack and add the array to it; if there aren't any
                // vectors on the stack then we're already on the root vector
                // and we found the end of its input so we can return it
                match array_stack.pop() {
                    Some(parent_array) => {
                        let finished_array = current_array;
                        current_array = parent_array;
                        let arr = if let Data::V(ref mut arr) = current_array {
                            arr
                        } else {
                            panic!("array will always be a Vector")
                        };
                        arr.push_back(finished_array);
                    }
                    None => return current_array,
                }
            }
        } else if c == &',' {
            if !number_builder.is_empty() {
                // we found an array separator (",") and we've been previously
                // parsing numbers, convert them into an integer and then
                // re-initialize the number builder
                let number: u32 = number_builder.parse().unwrap();
                number_builder = String::new();

                let arr = if let Data::V(ref mut arr) = current_array {
                    arr
                } else {
                    panic!("array will always be a Vector")
                };
                arr.push_back(Data::N(number));
            } else {
                // we found an array separator (",") but we haven't been
                // parsing numbers which means that we found the end of an
                // array that is mixed in with other elements, pop the parent
                // vector off the stack and add the array to it; if there are
                // no parent vectors it means that we're in the root vector
                // already (which we got back after we found its closing "]")
                match array_stack.pop() {
                    Some(parent_array) => {
                        let finished_array = current_array;
                        current_array = parent_array;

                        let arr = if let Data::V(ref mut arr) = current_array {
                            arr
                        } else {
                            panic!("array will always be a Vector")
                        };
                        arr.push_back(finished_array);
                    }
                    None => continue, // current_array is already the root
                }
            }
        } else {
            // we found a number add it to our number builder so that we can
            // parse it when we get to a character that signals the end of a
            // number ("]" or ",")
            number_builder += &c.to_string();
        }
    }

    current_array
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_in_order() {
        let mut input_a = "[1,1,3,1,1]".chars().collect();
        let mut input_b = "[1,1,5,1,1]".chars().collect();
        assert_eq!(
            in_order(parse_packet(input_a), parse_packet(input_b)),
            Some(true)
        );

        input_a = "[[1],[2,3,4]]".chars().collect();
        input_b = "[[1],4]".chars().collect();
        assert_eq!(
            in_order(parse_packet(input_a), parse_packet(input_b)),
            Some(true)
        );

        input_a = "[9]".chars().collect();
        input_b = "[[8,7,6]]".chars().collect();
        assert_eq!(
            in_order(parse_packet(input_a), parse_packet(input_b)),
            Some(false)
        );

        input_a = "[[4,4],4,4]".chars().collect();
        input_b = "[[4,4],4,4,4]".chars().collect();
        assert_eq!(
            in_order(parse_packet(input_a), parse_packet(input_b)),
            Some(true)
        );

        input_a = "[7,7,7,7]".chars().collect();
        input_b = "[7,7,7]".chars().collect();
        assert_eq!(
            in_order(parse_packet(input_a), parse_packet(input_b)),
            Some(false)
        );

        input_a = "[]".chars().collect();
        input_b = "[3]".chars().collect();
        assert_eq!(
            in_order(parse_packet(input_a), parse_packet(input_b)),
            Some(true)
        );

        input_a = "[[[]]]".chars().collect();
        input_b = "[[]]".chars().collect();
        assert_eq!(
            in_order(parse_packet(input_a), parse_packet(input_b)),
            Some(false)
        );

        input_a = "[1,[2,[3,[4,[5,6,7]]]],8,9]".chars().collect();
        input_b = "[1,[2,[3,[4,[5,6,0]]]],8,9]".chars().collect();
        assert_eq!(
            in_order(parse_packet(input_a), parse_packet(input_b)),
            Some(false)
        );
    }

    #[test]
    fn test_parse_packet() {
        let mut input: Vec<char> = "[9]".chars().collect();
        let mut inner = VecDeque::new();
        inner.push_back(Data::N(9));
        assert_eq!(parse_packet(input), Data::V(inner));

        input = "[[9,8],7]".chars().collect();
        let mut inner2 = VecDeque::new();
        let mut inner = VecDeque::new();
        inner2.push_back(Data::N(9));
        inner2.push_back(Data::N(8));
        inner.push_back(Data::V(inner2));
        inner.push_back(Data::N(7));
        assert_eq!(parse_packet(input), Data::V(inner));

        input = "[[],[]]".chars().collect();
        let inner1 = VecDeque::new();
        let inner2 = VecDeque::new();
        let mut inner = VecDeque::new();
        inner.push_back(Data::V(inner1));
        inner.push_back(Data::V(inner2));
        assert_eq!(parse_packet(input), Data::V(inner));
    }

    #[test]
    fn it_works() {
        let input = concat!(
            "[1,1,3,1,1]\n",
            "[1,1,5,1,1]\n",
            "\n",
            "[[1],[2,3,4]]\n",
            "[[1],4]\n",
            "\n",
            "[9]\n",
            "[[8,7,6]]\n",
            "\n",
            "[[4,4],4,4]\n",
            "[[4,4],4,4,4]\n",
            "\n",
            "[7,7,7,7]\n",
            "[7,7,7]\n",
            "\n",
            "[]\n",
            "[3]\n",
            "\n",
            "[[[]]]\n",
            "[[]]\n",
            "\n",
            "[1,[2,[3,[4,[5,6,7]]]],8,9]\n",
            "[1,[2,[3,[4,[5,6,0]]]],8,9]",
        );

        assert_eq!(y22d13(input, 1), 13);
        assert_eq!(y22d13(input, 2), 140);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2022/day13.txt").unwrap();

        assert_eq!(y22d13(&contents, 1), 6395);
        assert_eq!(y22d13(&contents, 2), 24921);
    }
}
