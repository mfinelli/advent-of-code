pub fn y15d01p1(input: &str) -> i32 {
    let chars: Vec<_> = input.trim().chars().collect();
    let mut floor = 0;

    for c in chars {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
    }

    floor
}

pub fn y15d01p2(input: &str) -> Option<u32> {
    let chars: Vec<_> = input.trim().chars().collect();
    let mut floor = 0;

    for (i, &c) in chars.iter().enumerate() {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }

        if floor < 0 {
            return Some((i + 1).try_into().unwrap());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let mut input = "(())";
        assert_eq!(y15d01p1(&input), 0);

        input = "()()\n";
        assert_eq!(y15d01p1(&input), 0);

        input = "(((";
        assert_eq!(y15d01p1(&input), 3);

        input = "(()(()(\n";
        assert_eq!(y15d01p1(&input), 3);

        input = "))(((((\n";
        assert_eq!(y15d01p1(&input), 3);

        input = "())";
        assert_eq!(y15d01p1(&input), -1);

        input = "))(\n";
        assert_eq!(y15d01p1(&input), -1);

        input = ")))";
        assert_eq!(y15d01p1(&input), -3);

        input = ")())())\n";
        assert_eq!(y15d01p1(&input), -3);

        input = ")";
        assert_eq!(y15d01p2(&input), Some(1));

        input = "()())\n";
        assert_eq!(y15d01p2(&input), Some(5));
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day01.txt").unwrap();

        assert_eq!(y15d01p1(&contents), 232);
        assert_eq!(y15d01p2(&contents).unwrap(), 1783);
    }
}
