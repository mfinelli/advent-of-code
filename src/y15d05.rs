pub fn y15d05(input: &str, part: u32) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut nice_strings = 0;

    // let mut tmp = 0;

    for line in lines {
        if part == 1 && is_nice_p1(line) {
            nice_strings += 1;
        } else if part == 2 && is_nice_p2(line) {
            nice_strings += 1;
        }

        // if tmp == 1 {
        //     break;
        // }
        // tmp += 1;
    }

    nice_strings
}

fn is_nice_p1(s: &str) -> bool {
    let bad_strings = ["ab", "cd", "pq", "xy"];
    for bad_string in bad_strings {
        if s.contains(bad_string) {
            return false;
        }
    }

    let chars: Vec<_> = s.chars().collect();
    let mut vowels = 0;
    let mut double = false;
    for (i, c) in chars.iter().enumerate() {
        if c == &'a' || c == &'e' || c == &'i' || c == &'o' || c == &'u' {
            vowels += 1;
        }

        if i < chars.len() - 1 {
            if c == &chars[i+1] {
                double = true;
            }
        }

        if double && vowels >= 3 {
            return true;
        }
    }

    false
}

fn is_nice_p2(s: &str) -> bool {
    println!("{}", s);
    let chars: Vec<_> = s.chars().collect();

    let mut non_overlapping_pair = false;
    let mut repeat_with_middle = false;
    // println!("{}", chars.len());
    for i in 0..chars.len() {
        // if i < chars.len() - 2 {
        //     if chars[i] == chars[i+2] && chars[i+1] == chars[i+3] {
        //         non_overlapping_pair = true;
        //     }
            // for (j, c) in chars.iter().enumerate() {
            // for j in 0..chars.len() -1 {
            //     if j != i && j + 1 != i {
            //         if chars[i] == chars[j] && chars[i+1] == chars[j+1] {
            //             non_overlapping_pair = true;
            //         }
            //     }
            // }
            // println!("{:?}", chars.windows(2));
            if i != chars.len() - 1{
            // println!("{}/{}: {}{}", s, i, chars[i], chars[i+1]);
            for (j, window) in chars.windows(2).enumerate() {
                if i == j || i + 1 == j || (i >= 1 && i - 1 == j){
                    // println!("discarding window {}", window.iter().collect::<String>());
                    continue;
                } else {
                    // println!("checking window {}", window.iter().collect::<String>());
                }

                if chars[i] == window[0] && chars[i + 1] == window[1] {
                    non_overlapping_pair = true;
                    // println!("found non_overlapping_pair {} {}, {} {}", s, window.iter().collect::<String>(), i, j);
                }
                // println!("{:?}", window);
                // println!("{}: {}", j, window.iter().collect::<String>());
            }
        }

        if i < chars.len() - 2 && chars[i] == chars[i+2] {
            repeat_with_middle = true;
        }

        if non_overlapping_pair && repeat_with_middle {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_is_nice_p1() {
        assert_eq!(is_nice_p1("aei"), false);
        assert_eq!(is_nice_p1("xazegov"), false);
        assert_eq!(is_nice_p1("aeiouaeiouaeiou"), false);
        assert_eq!(is_nice_p1("xx"), false);
        assert_eq!(is_nice_p1("abcdde"), false);
        assert_eq!(is_nice_p1("aabbccdd"), false);
        assert_eq!(is_nice_p1("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice_p1("aaa"), true);
        assert_eq!(is_nice_p1("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice_p1("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice_p1("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn test_is_nice_p2() {
        assert_eq!(is_nice_p2("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_nice_p2("xxyxx"), true);
        assert_eq!(is_nice_p2("uurcxstgmygtbstg"), false);
        assert_eq!(is_nice_p2("ieodomkazucvgmuy"), false);
    }

    #[test]
    fn it_works() {
        let input = concat!(
            "ugknbfddgicrmopn\n",
            "aaa\n",
            "jchzalrnumimnmhp\n",
            "haegwjzuvuyypxyu\n",
            "dvszwmarrgswjxmb\n",
            "qjhvhtzxzqqjkmpb\n",
            "xxyxx\n",
            "uurcxstgmygtbstg\n",
            "ieodomkazucvgmuy\n",
        );

        assert_eq!(y15d05(input, 1), 2);
        assert_eq!(y15d05(input, 2), 2);
    }

    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day05.txt").unwrap();

        assert_eq!(y15d05(&contents, 1), 238);
        assert_eq!(y15d05(&contents, 2), 69);
    }
}
