pub fn y22d04(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut sum = 0;

    for line in lines {
        let pairs: Vec<&str> = line.split(",").collect();
        let first_range: Vec<u32> = pairs[0].split("-").map(|r| r.parse().unwrap()).collect();
        let second_range: Vec<u32> = pairs[1].split("-").map(|r| r.parse().unwrap()).collect();

        if (first_range[0] >= second_range[0] && first_range[1] <= second_range[1])
            || (second_range[0] >= first_range[0] && second_range[1] <= first_range[1])
        {
            sum += 1;
        }
    }

    sum
}
