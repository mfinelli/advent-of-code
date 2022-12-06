pub fn y15d01(input: &str) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut input = "(())";
        assert_eq!(y15d01(&input), 0);

        input = "()()\n";
        assert_eq!(y15d01(&input), 0);

        input = "(((";
        assert_eq!(y15d01(&input), 3);

        input = "(()(()(\n";
        assert_eq!(y15d01(&input), 3);

        input = "))(((((\n";
        assert_eq!(y15d01(&input), 3);

        input = "())";
        assert_eq!(y15d01(&input), -1);

        input = "))(\n";
        assert_eq!(y15d01(&input), -1);

        input = ")))";
        assert_eq!(y15d01(&input), -3);

        input = ")())())\n";
        assert_eq!(y15d01(&input), -3);
    }
}
