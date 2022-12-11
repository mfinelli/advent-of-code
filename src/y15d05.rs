pub fn y15d05(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut nice_strings = 0;

    for line in lines {
        if is_nice(line) {
            nice_strings += 1;
        }
    }

    nice_strings
}

fn is_nice(s: &str) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = concat!(
            "ugknbfddgicrmopn\n",
            "aaa\n",
            "jchzalrnumimnmhp\n",
            "haegwjzuvuyypxyu\n",
            "dvszwmarrgswjxmb\n",
        );

        assert_eq!(y15d05(input), 2);
    }

    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day05.txt").unwrap();

        assert_eq!(y15d05(&contents), 0);
    }
}
