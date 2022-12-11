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
    name: u32,
    inspections: u32,
    items: VecDeque<u32>,
    operation_type: OperationType,
    operation_value: u32,
    test: u32,
    if_true: u32,
    if_false: u32,
}

pub fn y22d11(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut monkeys = Vec::new();

    for monkey_defn in lines.chunks(7) {
        let name_parts: Vec<_> = monkey_defn[0].split_whitespace().collect();
        // println!("{:?}", monkey);
        let mut items = VecDeque::new();

        // split_whitespace automatically strips leading whitespace
        let items_parts: Vec<_> = monkey_defn[1].split_whitespace().collect();
        if items_parts.len() > 2 {
            for item in items_parts.iter().skip(2) {
                // let value: u32 = item.trim_end_matches(',').parse().unwrap();
                // println!("{}", value);
                items.push_back(item.trim_end_matches(',').parse().unwrap());
            }
        }
        // println!("{:?}", items_parts);

        let operation_parts: Vec<_> = monkey_defn[2].split_whitespace().collect();
        let operation_type: OperationType;
        let operation_value: u32;
        // let operation_type = if operation_parts[4] == "+" {
        //     OperationType::Addition
        // } else {
        //     OperationType::Multiplication
        // };
        //

        if operation_parts[4] == "+" {
            if operation_parts[5] == "old" {
                operation_type = OperationType::AdditionSelf;
                operation_value = 0;
            } else {
                operation_type = OperationType::Addition;
                operation_value = operation_parts[5].parse().unwrap();
            }
        } else {
            if operation_parts[5] == "old" {
                operation_type = OperationType::MultiplicationSelf;
                operation_value = 0;
            } else {
                operation_type = OperationType::Multiplication;
                operation_value = operation_parts[5].parse().unwrap();
            }
        }

        let test_parts: Vec<_> = monkey_defn[3].split_whitespace().collect();
        let true_parts: Vec<_> = monkey_defn[4].split_whitespace().collect();
        let false_parts: Vec<_> = monkey_defn[5].split_whitespace().collect();

        monkeys.push(Monkey {
            name: name_parts[1].strip_suffix(':').unwrap().parse().unwrap(),
            inspections: 0,
            items: items,
            operation_type: operation_type,
            operation_value: operation_value,
            test: test_parts[3].parse().unwrap(),
            if_true: true_parts[5].parse().unwrap(),
            if_false: false_parts[5].parse().unwrap(),
        });

    }

    println!("{:?}", monkeys);

    0
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

        assert_eq!(y22d11(input), 10605);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2022/day11.txt").unwrap();

        assert_eq!(y22d11(&contents), 0);
    }
}
