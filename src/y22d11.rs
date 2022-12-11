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

use std::collections::VecDeque;

#[derive(Debug)]
enum OperationType {
    Addition,
    AdditionSelf,
    Multiplication,
    MultiplicationSelf,
}

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
/// # Example
/// ```rust
/// ```
pub fn y22d11(input: &str, rounds: u32, relief: bool) -> u64 {
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
        lcm *= u64::from(monkey.test);
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
                    OperationType::MultiplicationSelf => item = item * item,
                    OperationType::Multiplication => {
                        item *=
                            u64::from(monkeys[monkey_index].operation_value);
                    }
                }

                // increment the monkey's inspection counter
                monkeys[monkey_index].inspections += 1;

                if relief {
                    item /= 3;
                } else {
                    if item > lcm {
                        item %= lcm;
                    }
                }

                // finally throw (assign) the item to a new monkey
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
