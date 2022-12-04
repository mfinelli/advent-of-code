pub fn y22d04(input: &str, part: u32) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut sum = 0;

    for line in lines {
        let pairs: Vec<&str> = line.split(",").collect();
        let first_range: Vec<u32> =
            pairs[0].split("-").map(|r| r.parse().unwrap()).collect();
        let second_range: Vec<u32> =
            pairs[1].split("-").map(|r| r.parse().unwrap()).collect();

        if part == 1 {
            if (first_range[0] >= second_range[0]
                && first_range[1] <= second_range[1])
                || (second_range[0] >= first_range[0]
                    && second_range[1] <= first_range[1])
            {
                sum += 1;
            }
        } else {
            for i in first_range[0]..(first_range[1]+1) {
                if (second_range[0]..(second_range[1]+1)).contains(&i) {
                    sum += 1;
                    break;
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("src/input/day4.txt").unwrap();

        assert_eq!(y22d04(&contents, 1), 448);
        assert_eq!(y22d04(&contents, 2), 794);
    }
}
