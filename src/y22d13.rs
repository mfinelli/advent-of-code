// use std::any::Any;
use std::collections::VecDeque;
use regex::Regex;

#[derive(Debug, PartialEq)]
enum Thing {
    V(VecDeque<Thing>),
    N(u32),
}


pub fn y22d13(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut sum = 0;

    // let mut tmp = 1;

    for (i,packet_pair) in lines.chunks(3).enumerate() {
        let first_packet_raw: Vec<_> = packet_pair[0].chars().collect();
        let second_packet_raw: Vec<_> = packet_pair[1].chars().collect();
        // println!("{:?}", first_packet_raw);
        // println!("{:?}", second_packet_raw);

        // let r = Regex::new(r"^\[(.*)\]$").unwrap();

        // let mut first_packet: Vec<Thing> = Vec::new();
        // println!("{:?}", r.captures(first_packet_raw));
        // let captures = r.captures(first_packet_raw)


        // println!("{:?}", packet_pair[0]);
        let fp = parse_packet(first_packet_raw);
        // println!("{:?}", fp);
        // println!("done first");

        // println!("{:?}", packet_pair[1]);
        let sp = parse_packet(second_packet_raw);
        // println!("{:?}", sp);
        // println!("done second");

        match in_order(fp, sp) {
            Some(b) => {
                if b {
                    sum += i as u32 + 1;
                }
            },
            None => panic!("Couldn't reach a decision!"),
        }
        // if in_order(fp, sp) {
        //     sum += i as u32 + 1;
        // }




        // if tmp == 1 {
        //     // break;
        // }
        // tmp += 1;
    }

    // 0
    sum
}

fn in_order(a: Thing, b: Thing) -> Option<bool> {
    match a {
        Thing::V(mut a) => {
            match b {
                Thing::V(mut b) => {
                    println!("a and b are both lists");
                    while true {
                        let a_item = a.pop_front();
                        let b_item = b.pop_front();

                        println!("comparing {:?} to {:?}", a_item, b_item);

                        match a_item {
                            Some(a_item) => {
                                match b_item {
                                    Some(b_item) => {
                                        match in_order(a_item, b_item) {
                                            Some(v) => return Some(v),
                                            None => continue,
                                        }
                                    },
                                    None => {
                                        return Some(false);
                                    }
                                }
                            },
                            None => {
                                if b_item.is_some() {
                                    return Some(true);
                                } else {
                                    return None;
                                }
                                // return Some(b_item.is_some());
                            }
                        }

                    }
                },
                Thing::N(b) => {
                    let mut b_as_v = VecDeque::new();
                    b_as_v.push_back(Thing::N(b));
                    return in_order(Thing::V(a), Thing::V(b_as_v));
                },
            }
        },
        Thing::N(a) => {
            match b {
                Thing::V(b) => {
                    let mut a_as_v = VecDeque::new();
                    a_as_v.push_back(Thing::N(a));
                    return in_order(Thing::V(a_as_v), Thing::V(b));
                },
                Thing::N(b) => {
                    if a < b {
                        return Some(true);
                    } else if a > b {
                        return Some(false);
                    } else {
                        return None;
                    }
                },
            }
        },
    }

    None
    // Some(false)
    // false
}

fn parse_packet(chars: Vec<char>) -> Thing {
    // let r = Regex::new(r"^\[(.*)\]$").unwrap();
    // let mut packet = Thing::V(Vec::new());
    let mut numbuilder = String::new();
    let mut vecs: Vec<Thing> = Vec::new();
    let mut arrbuilder = Thing::V(VecDeque::new());

    for c in chars.iter().skip(1) {
        if c == &'[' {
            // println!("started array");
            // println!("vecs: {:?}", vecs);
            // println!("arrbuilder: {:?}", arrbuilder);
            vecs.push(arrbuilder);
            arrbuilder = Thing::V(VecDeque::new());
            // println!("after op");
            // println!("vecs: {:?}", vecs);
            // println!("arrbuilder: {:?}", arrbuilder);
        } else if c == &']' {
            if numbuilder != "" {
                let num: u32 = numbuilder.parse().unwrap();
                // println!("found number {}", num);
                numbuilder = String::new();

                let v = if let Thing::V(ref mut v) = arrbuilder { v } else { panic!("Must be a vector") };
                v.push_back(Thing::N(num));
            } else {
                // found the end of an array
                // println!("closed an array");
                // println!("vecs: {:?}", vecs);
                // println!("arrbuilder: {:?}", arrbuilder);
                // arrbuilder = vecs.pop().unwrap();
                match vecs.pop() {
                    Some(vv) => {
                        let finished_arr = arrbuilder;
                        arrbuilder = vv;
                        let v = if let Thing::V(ref mut v) = arrbuilder { v } else { panic!("Must be a vector") };
                        v.push_back(finished_arr);
                    },
                    None => return arrbuilder,
                //         arrbuilder = vv;
                //         let v = if let Thing::V(ref mut v) = arrbuilder { v } else { panic!("Must be a vector") };
                //         v.push(finished_arr);
                //     },
                //     None => return arrbuilder,
                }

            }
            // println!("after op");
            // println!("vecs: {:?}", vecs);
            // println!("arrbuilder: {:?}", arrbuilder);
        } else if c == &',' {
            if numbuilder != "" {
                let num: u32 = numbuilder.parse().unwrap();
                // println!("found number {}", num);
                numbuilder = String::new();

                let v = if let Thing::V(ref mut v) = arrbuilder { v } else { panic!("Must be a vector") };
                v.push_back(Thing::N(num));
            } else {
                // println!("found comma but no number");
                // println!("vecs: {:?}", vecs);
                // println!("arrbuilder: {:?}", arrbuilder);
                // println!("closed
                // found the end of an array inside an array
                match vecs.pop() {
                    Some(vv) => {
                        let finished_arr = arrbuilder;
                        arrbuilder = vv;

                        let v = if let Thing::V(ref mut v) = arrbuilder { v } else { panic!("Must be a vector") };
                        v.push_back(finished_arr);
                    }
                    None => continue, // arrbuilder is already the root
                }
            }
            // println!("after op");
            // println!("vecs: {:?}", vecs);
            // println!("arrbuilder: {:?}", arrbuilder);
       } else {
            numbuilder += &c.to_string();
        }
    }


    // let c = chars.remove(0);
    // if c == '[' {
    //     // TODO
    // } else if  c == ']' {
    //     // TODO
    // } else if c == ','

    // let mut capture = r.captures(line);
    // while let Some(cap) = capture {
    //     println!("{:?}", cap.get(1).unwrap().as_str());
    //     capture = r.captures(cap.get(1).unwrap().as_str());
    // }

    // let chars: Vec<_> = line.chars().collect();
    // for c in chars {
    // }

    // let Thing::V(ref mut p) = packet;
    // let p = if let Thing::V(ref mut p) = packet { p } else { todo!() };
    // p.push(Thing::N(1));

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
        assert_eq!(in_order(parse_packet(input_a), parse_packet(input_b)), Some(true));

        let input_a = "[[1],[2,3,4]]".chars().collect();
        let input_b = "[[1],4]".chars().collect();
        assert_eq!(in_order(parse_packet(input_a), parse_packet(input_b)), Some(true));

        let input_a = "[9]".chars().collect();
        let input_b = "[[8,7,6]]".chars().collect();
        assert_eq!(in_order(parse_packet(input_a), parse_packet(input_b)), Some(false));

        let input_a = "[[4,4],4,4]".chars().collect();
        let input_b = "[[4,4],4,4,4]".chars().collect();
        assert_eq!(in_order(parse_packet(input_a), parse_packet(input_b)), Some(true));

        let input_a = "[7,7,7,7]".chars().collect();
        let input_b = "[7,7,7]".chars().collect();
        assert_eq!(in_order(parse_packet(input_a), parse_packet(input_b)), Some(false));

        let input_a = "[]".chars().collect();
        let input_b = "[3]".chars().collect();
        assert_eq!(in_order(parse_packet(input_a), parse_packet(input_b)), Some(true));

        let input_a = "[[[]]]".chars().collect();
        let input_b = "[[]]".chars().collect();
        assert_eq!(in_order(parse_packet(input_a), parse_packet(input_b)), Some(false));

        let input_a = "[1,[2,[3,[4,[5,6,7]]]],8,9]".chars().collect();
        let input_b = "[1,[2,[3,[4,[5,6,0]]]],8,9]".chars().collect();
        assert_eq!(in_order(parse_packet(input_a), parse_packet(input_b)), Some(false));
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

        assert_eq!(y22d13(input), 13);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2022/day13.txt").unwrap();

        assert_eq!(y22d13(&contents), 6395);
    }
}
