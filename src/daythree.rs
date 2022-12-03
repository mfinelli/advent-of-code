use std::collections::HashSet;

pub fn daythree(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let priorities = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

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
                    sum += priorities.find(ch).unwrap() as u32 + 1;
                    break;
                    // sum += priorities.chars().position(|c| c== ch).unwrap();
                }
            }
            i += 1;
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
        assert_eq!(daythree(input), 5);

        input = "ABBCaaDEEF\nHabcHdef\n";
        assert_eq!(daythree(input), 35);

        input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        assert_eq!(daythree(input), 16);

        input = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        assert_eq!(daythree(input), 38);

        input = "PmmdzqPrVvPwwTWBwg";
        assert_eq!(daythree(input), 42);

        input = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn";
        assert_eq!(daythree(input), 22);

        input = "ttgJtRGJQctTZtZT";
        assert_eq!(daythree(input), 20);

        input = "CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(daythree(input), 19);
    }
}
