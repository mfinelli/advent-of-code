// use std::any::Any;
use std::collections::VecDeque;
use regex::Regex;

#[derive(Debug, PartialEq)]
enum Thing {
    V(Vec<Thing>),
    N(u32),
}


pub fn y22d13(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();

    let mut tmp = 1;

    for packet_pair in lines.chunks(3) {
        let first_packet_raw: Vec<_> = packet_pair[0].chars().collect();
        let second_packet_raw: Vec<_> = packet_pair[1].chars().collect();
        println!("{:?}", first_packet_raw);
        println!("{:?}", second_packet_raw);

        let r = Regex::new(r"^\[(.*)\]$").unwrap();

        // let mut first_packet: Vec<Thing> = Vec::new();
        // println!("{:?}", r.captures(first_packet_raw));
        // let captures = r.captures(first_packet_raw)


        println!("{:?}", packet_pair[0]);
        let fp = parse_packet(first_packet_raw);
        println!("{:?}", fp);
        println!("done first");

        println!("{:?}", packet_pair[1]);
        let sp = parse_packet(second_packet_raw);
        println!("{:?}", sp);
        println!("done second");



        if tmp == 1 {
            break;
        }
        tmp += 1;
    }

    0
}

// fn in_order(a: Vec<Thing>, b: Vec<Thing) -> bool {
//     false
// }

fn parse_packet(chars: Vec<char>) -> Thing {
    // let r = Regex::new(r"^\[(.*)\]$").unwrap();
    // let mut packet = Thing::V(Vec::new());
    let mut numbuilder = String::new();
    let mut vecs: Vec<Thing> = Vec::new();
    let mut arrbuilder = Thing::V(Vec::new());

    for c in chars {
        if c == '[' {
            println!("started array");
            vecs.push(arrbuilder);
            arrbuilder = Thing::V(Vec::new());
            println!("{:?}", vecs);
            println!("{:?}", arrbuilder);
        } else if c == ']' {
            if numbuilder != "" {
                let num: u32 = numbuilder.parse().unwrap();
                println!("found number {}", num);
                numbuilder = String::new();

                let v = if let Thing::V(ref mut v) = arrbuilder { v } else { panic!("Must be a vector") };
                v.push(Thing::N(num));
            } else {
                // found the end of an array
                let finished_arr = arrbuilder;
                arrbuilder = vecs.pop().unwrap();

                let v = if let Thing::V(ref mut v) = arrbuilder { v } else { panic!("Must be a vector") };
                v.push(finished_arr);

            }
            println!("closed array");
            println!("{:?}", vecs);
            println!("{:?}", arrbuilder);
        } else if c == ',' {
            if numbuilder != "" {
                let num: u32 = numbuilder.parse().unwrap();
                println!("found number {}", num);
                numbuilder = String::new();

                let v = if let Thing::V(ref mut v) = arrbuilder { v } else { panic!("Must be a vector") };
                v.push(Thing::N(num));
            } else {
                // println!("closed
                // found the end of an array inside an array
                // let finished_arr = arrbuilder;
                // arrbuilder = vecs.pop().unwrap();

                // let v = if let Thing::V(ref mut v) = arrbuilder { v } else { panic!("Must be a vector") };
                // v.push(finished_arr);
            }
            println!("{:?}", vecs);
            println!("{:?}", arrbuilder);
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

    #[test]
    fn test_parse_packet() {
        let mut input: Vec<char> = "[9]".chars().collect();
        let mut inner = Vec::new();
        inner.push(Thing::N(9));
        assert_eq!(parse_packet(input), Thing::V(inner));

        input = "[[9,8],7]".chars().collect();
        let mut inner2 = Vec::new();
        let mut inner = Vec::new();
        inner2.push(Thing::N(9));
        inner2.push(Thing::N(8));
        inner.push(Thing::V(inner2));
        inner.push(Thing::N(7));
        assert_eq!(parse_packet(input), Thing::V(inner));
    }
}
