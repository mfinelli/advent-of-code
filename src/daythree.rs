use std::collections::HashSet;

const PRIORITY: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn y22d03(input: &str, part: u32) -> u32 {
    if part == 1 {
        y22d03p1(&input)
    } else {
        y22d03p2(&input)
    }
}

fn y22d03p1(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();

    let mut sum = 0;

    for line in lines {
        let mut set = HashSet::new();
        let half = line.len() / 2;
        let mut i = 1;
        for ch in line.chars() {
            if i <= half {
                // add items to the set
                set.insert(ch);
            } else {
                // if set contains ch then add priority to sum and break
                if set.contains(&ch) {
                    sum += PRIORITY.find(ch).unwrap() as u32 + 1;
                    break;
                    // sum += priorities.chars().position(|c| c== ch).unwrap();
                }
            }
            i += 1;
        }
    }

    sum
}

fn y22d03p2(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let groups = lines.len() / 3;

    let mut sum = 0;

    for gi in 0..groups {
        for c in lines[gi * 3 + 2].chars() {
            if lines[gi * 3].contains(c) && lines[gi * 3 + 1].contains(c) {
                sum += PRIORITY.find(c).unwrap() as u32 + 1;
                break;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut input = "cDeFeg";
        assert_eq!(y22d03p1(input), 5);

        input = "ABBCaaDEEF\nHabcHdef\n";
        assert_eq!(y22d03p1(input), 35);

        input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        assert_eq!(y22d03p1(input), 16);

        input = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        assert_eq!(y22d03p1(input), 38);

        input = "PmmdzqPrVvPwwTWBwg";
        assert_eq!(y22d03p1(input), 42);

        input = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn";
        assert_eq!(y22d03p1(input), 22);

        input = "ttgJtRGJQctTZtZT";
        assert_eq!(y22d03p1(input), 20);

        input = "CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(y22d03p1(input), 19);
    }
}
