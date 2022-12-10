pub fn y22d10(input: &str) -> i32 {
    let lines: Vec<_> = input.lines().collect();
    let mut x = 1;
    let mut cycles = Vec::new();
    cycles.push(x);

    for line in lines {
        if line == "noop" {
            cycles.push(x);
        } else {
            let parts: Vec<_> = line.split_whitespace().collect();
            let addx: i32 = parts[1].parse().unwrap();

            cycles.push(x);
            x += addx;
            cycles.push(x);
        }
    }

    if cycles.len() < 20 {
        return 0;
    } else if cycles.len() == 20 {
        return cycles.pop().unwrap() * 20;
    }

    println!("{:?}", cycles);
    let signal_strength_checks = (cycles.len() - 20) / 40;
    let mut signal_strength = cycles[19] * 20;
    println!("starting signal strenth: {}", signal_strength);
    for i in 0..signal_strength_checks {
        let cycle = (i+1)* 40 + 20;
        println!("adding strength from position {}: {}", cycle, cycles[cycle -1]);
        signal_strength += cycles[cycle - 1] * cycle as i32;
        println!("with multiplier: {}", cycles[cycle -1] * cycle as i32);
    }




    signal_strength
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = concat!(
            "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\n",
            "addx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\n",
            "addx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\n",
            "addx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\n",
            "addx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\n",
            "addx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\n",
            "addx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\n",
            "addx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\n",
            "addx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\n",
            "addx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\n",
            "noop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\n",
            "addx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\n",
            "noop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\n",
            "addx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\n",
            "addx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\n",
            "noop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\n",
            "addx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\n",
            "addx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\n",
            "addx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop\n",
            );

        assert_eq!(y22d10(input), 13140);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2022/day10.txt").unwrap();

        assert_eq!(y22d10(&contents), 14560);
    }
}
