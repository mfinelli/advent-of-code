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

//! Advent of Code 2022 Day 11: <https://adventofcode.com/2022/day/11>
//!
//! I thought that part one of this challenge was relatively easy and was at
//! first surprised that part two was just to increase the number of rounds
//! and remove a division step. Then I ran into my first overflow, increased
//! my integers to `u64` and then `u128` and they were still too small. I
//! thought that the prompt's instructions "you'll need to find another way to
//! keep your worry levels manageable" was just flavor text. I started to
//! implement a crate that added support for `u256`+ sized integers until I
//! decided that I was missing something. I wasn't really sure how to approach
//! the problem until I read something really helpful on
//! [reddit](https://www.reddit.com/r/adventofcode/comments/zim5o6/comment/izrr6mq/):
//!
//! >Hint: you don't care what the result of the division is, you just care
//! >whether the current worry level is divisible by the monkey's test value.
//! >Is there a way you can keep the worry level small(er) while still being
//! >able to tell if it's divisible?
//!
//! I played around some trying to divide the original factor until I found
//! the modulo method. During my cleanup phase I also read that it's better to
//! compute the least common multiple and use that instead of the product, but
//! reading around some more it seems that may be bad advice. I already
//! implemented it again using the LCM and the tests still pass so it can't
//! _hurt_, but I don't understand the math well enough to know for sure one
//! way or the other.
//!
//! What follows is my attempt to explain why/how the modulo method is needed
//! and works. Inspecting the monkeys we can see that some (or rather one) of
//! them multiplies the item value by itself (i.e., `value^2`) each time that
//! it inspects an object (it must be _really_ careless). In part one we have
//! a low enough number of rounds that reducing the worry factor by three each
//! time is enough to keep this value from getting too out of hand. It also
//! probably wouldn't be a problem if we never had the exponential growth and
//! only multiplied (or added) by set, fixed values. In any case, as it is the
//! item values can grow crazy big crazy fast. The strategy then is to compute
//! a value that we can remove from the item's value that will reduce its size
//! but keep in tact whether it's divisible by all of the monkeys' test values.
//! Consider some smaller, arbitrary numbers for the tests: `3`, `5`, and `7`.
//! Let's assume that we have an item with value `300` and that our overflow
//! is small say `500`. If we have a monkey that multiplies the item value by
//! itself we'll end up with a worry score of 90000 which would obviously
//! overflow. We also want to know whether the result is divisible by our
//! initial numbers and if not what the remainder is. `90000 % 3` and `90000 %
//! 5` are both `0`, but `90000 % 7` is `1`. We can "take out" `857` copies of
//! our LCM `105` (`857 * 105 == 89985`) and be left with `15`. `15 % 3` and
//! `15 % 5` are both `0` and `15 % 7` is `1`, just like our original, huge
//! number. We can safely replace that value with `15` which fits inside our
//! overflow. There are some much better, math filled explanations on the
//! Advent of Code subreddit that explain why/how this all works should you be
//! interested.

use crate::util;
use std::collections::VecDeque;

/// OperationType is a representation of how the worry level changes, we can
/// either add/multiply by another value or add/multiply the same value. This
/// lets store the strategy for each monkey in a way that will let us `match`
/// later.
#[derive(Debug)]
enum OperationType {
    Addition,
    AdditionSelf,
    Multiplication,
    MultiplicationSelf,
}

/// Monkey is the current state of each monkey, how many item inspections that
/// they have done, which items they're currently holding, which worry level
/// strategy they implement (and the value modifier if it's not *Self), as
/// well as the information necessary for determining how and to which monkey
/// it will pass items.
#[derive(Debug)]
struct Monkey {
    inspections: u64,
    items: VecDeque<u64>,
    operation_type: OperationType,
    operation_value: u32,
    test: u32,
    if_true: usize,
    if_false: usize,
}

/// The solution for the day eleven challenge.
///
/// There are three input arguments: the input (monkey definitions) as a
/// string, the number of rounds to simulate as an integer, and a boolean
/// whether or not we should implement the "static relief" method (i.e., divide
/// the worry by `3` on each inspection). If it's `false` then we do part two
/// of the challenge where we reduce the worry using the modulo method.
///
/// We start by parsing the input. Each monkey's definition is the same: it
/// takes six lines (seven including a trailing newline, so we split the input
/// into chunks of seven lines to parse it). The first line is the monkey's
/// "name" or index, starting with `0`. The next line is a comma-separated
/// list of the items/worry that a monkey is currently holding. The next line
/// is the definition of how the worry changes each time the monkey inspects
/// an item. There are four possibilities: the monkey can add or multiply by
/// a static number or they monkey can add the item's value to itself or
/// multiply the item's value by itself. Finally, are three lines that tell
/// the monkey where to send the item next. The first of these three lines is
/// the number to divide the item by and if the result is `0` (i.e., no
/// remainder) then the monkey gives the item to the monkey on the second line
/// but otherwise (i.e., there _is_ a remainder) then the monkey gives the
/// item to the monkey on the third line.
///
/// Parsing this input essentially amounts to splitting a bunch of whitespace
/// and parsing the integers found in certain indexes of the result. One
/// important implementation note is that the monkeys inspect the items in a
/// FIFO (first-in-first-out) order meaning that a standard vector is
/// unsuitable for tracking the items that the monkey is holding. We instead
/// use a [`std::collections::VecDeque`] and its `push_back()` and
/// `pop_front()` methods to achieve this behavior.
///
/// Then we start simulating the rounds. We check each item that the monkey
/// is holding (`pop()`ping them from the items list) and apply the worry
/// modification to it. We increase the number of inspections that the monkey
/// has done and then apply a worry relief strategy. We then check if the
/// item is divisible by the monkey's `test` number and then pass the item to
/// the corresponding monkey.
///
/// Finally, we sort the monkeys by how many inspections they did and return
/// the number of inspections made by the top two monkeys multiplied together.
///
/// # Example
/// ```rust
/// # use aoc::y22d11::y22d11;
/// // probably read this from the input file...
/// let input = concat!(
///     "Monkey 0:\n",
///     "  Starting items: 1, 2\n",
///     "  Operation: new = old * 8\n",
///     "  Test: divisible by 3\n",
///     "    If true: throw to monkey 2\n",
///     "    If false: throw to monkey 1\n",
///     "\n",
///     "Monkey 1:\n",
///     "  Starting items: 3, 4, 5, 6\n",
///     "  Operation: new = old + 9\n",
///     "  Test: divisible by 5\n",
///     "    If true: throw to monkey 0\n",
///     "    If false: throw to monkey 2\n",
///     "\n",
///     "Monkey 2:\n",
///     "  Starting items: 7\n",
///     "  Operation: new = old * old\n",
///     "  Test: divisible by 7\n",
///     "    If true: throw to monkey 1\n",
///     "    If false: throw to monkey 0",
/// );
/// assert_eq!(y22d11(&input, 5, true), 858);
/// assert_eq!(y22d11(&input, 5, false), 896);
/// ```
pub fn y22d11(input: &str, rounds: u32, static_relief: bool) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let mut monkeys = Vec::new();

    for monkey_defn in lines.chunks(7) {
        let mut items = VecDeque::new();

        // split_whitespace() automatically strips leading whitespace
        let items_parts: Vec<_> = monkey_defn[1].split_whitespace().collect();
        if items_parts.len() > 2 {
            for item in items_parts.iter().skip(2) {
                items.push_back(item.trim_end_matches(',').parse().unwrap());
            }
        }

        let operation_parts: Vec<_> =
            monkey_defn[2].split_whitespace().collect();
        let operation_type: OperationType;
        let operation_value: u32;

        if operation_parts[5] == "old" {
            operation_value = 0;

            if operation_parts[4] == "+" {
                operation_type = OperationType::AdditionSelf;
            } else {
                operation_type = OperationType::MultiplicationSelf;
            }
        } else {
            operation_value = operation_parts[5].parse().unwrap();

            if operation_parts[4] == "+" {
                operation_type = OperationType::Addition;
            } else {
                operation_type = OperationType::Multiplication;
            }
        }

        let test_parts: Vec<_> = monkey_defn[3].split_whitespace().collect();
        let true_parts: Vec<_> = monkey_defn[4].split_whitespace().collect();
        let false_parts: Vec<_> = monkey_defn[5].split_whitespace().collect();

        monkeys.push(Monkey {
            inspections: 0,
            items,
            operation_type,
            operation_value,
            test: test_parts[3].parse().unwrap(),
            if_true: true_parts[5].parse().unwrap(),
            if_false: false_parts[5].parse().unwrap(),
        });
    }

    let mut lcm = 1;
    for monkey in &monkeys {
        lcm = util::lcm(lcm, u64::from(monkey.test));
    }

    for _ in 0..rounds {
        for monkey_index in 0..monkeys.len() {
            // if the monkey doesn't have any items then their turn is over
            if monkeys[monkey_index].items.is_empty() {
                continue;
            }

            while let Some(mut item) = monkeys[monkey_index].items.pop_front() {
                // first the monkey inspects the item
                match monkeys[monkey_index].operation_type {
                    OperationType::AdditionSelf => item += item,
                    OperationType::Addition => {
                        item +=
                            u64::from(monkeys[monkey_index].operation_value);
                    }
                    OperationType::MultiplicationSelf => item *= item,
                    OperationType::Multiplication => {
                        item *=
                            u64::from(monkeys[monkey_index].operation_value);
                    }
                }

                // increment the monkey's inspection counter
                monkeys[monkey_index].inspections += 1;

                // implement the relief algorithm
                if static_relief {
                    item /= 3;
                } else if item > lcm {
                    item %= lcm;
                }

                // finally, throw (assign) the item to a new monkey
                if item % u64::from(monkeys[monkey_index].test) == 0 {
                    let new_monkey_index = monkeys[monkey_index].if_true;
                    monkeys[new_monkey_index].items.push_back(item);
                } else {
                    let new_monkey_index = monkeys[monkey_index].if_false;
                    monkeys[new_monkey_index].items.push_back(item);
                }
            }
        }
    }

    monkeys.sort_by(|a, b| a.inspections.cmp(&b.inspections));
    monkeys.pop().unwrap().inspections * monkeys.pop().unwrap().inspections
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = concat!(
            "Monkey 0:\n",
            "  Starting items: 79, 98\n",
            "  Operation: new = old * 19\n",
            "  Test: divisible by 23\n",
            "    If true: throw to monkey 2\n",
            "    If false: throw to monkey 3\n",
            "\n",
            "Monkey 1:\n",
            "  Starting items: 54, 65, 75, 74\n",
            "  Operation: new = old + 6\n",
            "  Test: divisible by 19\n",
            "    If true: throw to monkey 2\n",
            "    If false: throw to monkey 0\n",
            "\n",
            "Monkey 2:\n",
            "  Starting items: 79, 60, 97\n",
            "  Operation: new = old * old\n",
            "  Test: divisible by 13\n",
            "    If true: throw to monkey 1\n",
            "    If false: throw to monkey 3\n",
            "\n",
            "Monkey 3:\n",
            "  Starting items: 74\n",
            "  Operation: new = old + 3\n",
            "  Test: divisible by 17\n",
            "    If true: throw to monkey 0\n",
            "    If false: throw to monkey 1\n",
        );

        assert_eq!(y22d11(input, 20, true), 10605);
        assert_eq!(y22d11(input, 1000, false), 27019168);
        assert_eq!(y22d11(input, 10000, false), 2713310158);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2022/day11.txt").unwrap();

        assert_eq!(y22d11(&contents, 20, true), 50830);
        assert_eq!(y22d11(&contents, 10000, false), 14399640002);
    }
}
