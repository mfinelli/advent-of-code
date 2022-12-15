use std::cmp::Ordering;
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
enum Thing {
    V(VecDeque<Thing>),
    N(u32),
}

pub fn y22d13(input: &str, part: u32) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut things = Vec::new();
    let mut result = 0;

    for (i, packet_pair) in lines.chunks(3).enumerate() {
        let first_packet_raw: Vec<_> = packet_pair[0].chars().collect();
        let second_packet_raw: Vec<_> = packet_pair[1].chars().collect();

        things.push(packet_pair[0]);
        things.push(packet_pair[1]);

        let fp = parse_packet(first_packet_raw);
        let sp = parse_packet(second_packet_raw);

        if part == 1 {
            match in_order(fp, sp) {
                Some(b) => {
                    if b {
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
    things.push("[[2]]");
    things.push("[[6]]");

    // insertion sort can be O(n^2) but we have such a small dataset that it's
    // fine; it's not worth the effort to implement a more efficient sorting
    // algorithm
    for i in 1..things.len() {
        let mut j = i;
        while j > 0
            && !in_order(
                parse_packet(things[j - 1].chars().collect()),
                parse_packet(things[j].chars().collect()),
            )
            .unwrap()
        {
            things.swap(j - 1, j);
            j -= 1;
        }
    }

    result = 1;
    for (i, thing) in things.iter().enumerate() {
        if thing == &"[[2]]" || thing == &"[[6]]" {
            result *= (i as u32) + 1;
        }
    }

    result
}

fn in_order(a: Thing, b: Thing) -> Option<bool> {
    match a {
        Thing::V(mut a) => match b {
            Thing::V(mut b) => loop {
                let a_item = a.pop_front();
                let b_item = b.pop_front();

                match a_item {
                    Some(a_item) => match b_item {
                        Some(b_item) => match in_order(a_item, b_item) {
                            Some(v) => return Some(v),
                            None => continue,
                        },
                        None => {
                            return Some(false);
                        }
                    },
                    None => {
                        if b_item.is_some() {
                            return Some(true);
                        } else {
                            return None;
                        }
                    }
                }
            },
            Thing::N(b) => {
                let mut b_as_v = VecDeque::new();
                b_as_v.push_back(Thing::N(b));
                in_order(Thing::V(a), Thing::V(b_as_v))
            }
        },
        Thing::N(a) => match b {
            Thing::V(b) => {
                let mut a_as_v = VecDeque::new();
                a_as_v.push_back(Thing::N(a));
                in_order(Thing::V(a_as_v), Thing::V(b))
            }
            Thing::N(b) => match a.cmp(&b) {
                Ordering::Less => Some(true),
                Ordering::Greater => Some(false),
                Ordering::Equal => None,
            },
        },
    }
}

fn parse_packet(chars: Vec<char>) -> Thing {
    let mut numbuilder = String::new();
    let mut vecs: Vec<Thing> = Vec::new();
    let mut arrbuilder = Thing::V(VecDeque::new());

    for c in chars.iter().skip(1) {
        if c == &'[' {
            vecs.push(arrbuilder);
            arrbuilder = Thing::V(VecDeque::new());
        } else if c == &']' {
            if !numbuilder.is_empty() {
                let num: u32 = numbuilder.parse().unwrap();
                numbuilder = String::new();

                let v = if let Thing::V(ref mut v) = arrbuilder {
                    v
                } else {
                    panic!("Must be a vector")
                };
                v.push_back(Thing::N(num));
            } else {
                match vecs.pop() {
                    Some(vv) => {
                        let finished_arr = arrbuilder;
                        arrbuilder = vv;
                        let v = if let Thing::V(ref mut v) = arrbuilder {
                            v
                        } else {
                            panic!("Must be a vector")
                        };
                        v.push_back(finished_arr);
                    }
                    None => return arrbuilder,
                }
            }
        } else if c == &',' {
            if !numbuilder.is_empty() {
                let num: u32 = numbuilder.parse().unwrap();
                numbuilder = String::new();

                let v = if let Thing::V(ref mut v) = arrbuilder {
                    v
                } else {
                    panic!("Must be a vector")
                };
                v.push_back(Thing::N(num));
            } else {
                match vecs.pop() {
                    Some(vv) => {
                        let finished_arr = arrbuilder;
                        arrbuilder = vv;

                        let v = if let Thing::V(ref mut v) = arrbuilder {
                            v
                        } else {
                            panic!("Must be a vector")
                        };
                        v.push_back(finished_arr);
                    }
                    None => continue, // arrbuilder is already the root
                }
            }
        } else {
            numbuilder += &c.to_string();
        }
    }

    arrbuilder
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

        let input_a = "[[1],[2,3,4]]".chars().collect();
        let input_b = "[[1],4]".chars().collect();
        assert_eq!(
            in_order(parse_packet(input_a), parse_packet(input_b)),
            Some(true)
        );

        let input_a = "[9]".chars().collect();
        let input_b = "[[8,7,6]]".chars().collect();
        assert_eq!(
            in_order(parse_packet(input_a), parse_packet(input_b)),
            Some(false)
        );

        let input_a = "[[4,4],4,4]".chars().collect();
        let input_b = "[[4,4],4,4,4]".chars().collect();
        assert_eq!(
            in_order(parse_packet(input_a), parse_packet(input_b)),
            Some(true)
        );

        let input_a = "[7,7,7,7]".chars().collect();
        let input_b = "[7,7,7]".chars().collect();
        assert_eq!(
            in_order(parse_packet(input_a), parse_packet(input_b)),
            Some(false)
        );

        let input_a = "[]".chars().collect();
        let input_b = "[3]".chars().collect();
        assert_eq!(
            in_order(parse_packet(input_a), parse_packet(input_b)),
            Some(true)
        );

        let input_a = "[[[]]]".chars().collect();
        let input_b = "[[]]".chars().collect();
        assert_eq!(
            in_order(parse_packet(input_a), parse_packet(input_b)),
            Some(false)
        );

        let input_a = "[1,[2,[3,[4,[5,6,7]]]],8,9]".chars().collect();
        let input_b = "[1,[2,[3,[4,[5,6,0]]]],8,9]".chars().collect();
        assert_eq!(
            in_order(parse_packet(input_a), parse_packet(input_b)),
            Some(false)
        );
    }

    #[test]
    fn test_parse_packet() {
        let mut input: Vec<char> = "[9]".chars().collect();
        let mut inner = VecDeque::new();
        inner.push_back(Thing::N(9));
        assert_eq!(parse_packet(input), Thing::V(inner));

        input = "[[9,8],7]".chars().collect();
        let mut inner2 = VecDeque::new();
        let mut inner = VecDeque::new();
        inner2.push_back(Thing::N(9));
        inner2.push_back(Thing::N(8));
        inner.push_back(Thing::V(inner2));
        inner.push_back(Thing::N(7));
        assert_eq!(parse_packet(input), Thing::V(inner));

        input = "[[],[]]".chars().collect();
        let inner1 = VecDeque::new();
        let inner2 = VecDeque::new();
        let mut inner = VecDeque::new();
        inner.push_back(Thing::V(inner1));
        inner.push_back(Thing::V(inner2));
        assert_eq!(parse_packet(input), Thing::V(inner));
    }

    #[test]
    fn it_works() {
        let mut input = concat!(
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
