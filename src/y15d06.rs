use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Instruction {
    Toggle,
    TurnOn,
    TurnOff,
}

pub fn y15d06p1(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut on = 0;

    let mut lights = [[false; 1000]; 1000];

    for line in lines {
        let (instruction, x1, y1, x2, y2) = parse_instruction(line);

        for y in y1..y2 + 1 {
            for x in x1..x2 + 1 {
                if instruction == Instruction::Toggle {
                    if lights[y][x] {
                        lights[y][x] = false;
                    } else {
                        lights[y][x] = true;
                    }
                } else if instruction == Instruction::TurnOff {
                    lights[y][x] = false;
                } else {
                    lights[y][x] = true;
                }
            }
        }
    }

    for i in 0..1000 {
        for j in 0..1000 {
            if lights[i][j] {
                on += 1;
            }
        }
    }

    on
}

pub fn y15d06p2(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let mut lights = HashMap::new();
    let mut total = 0;

    for line in lines {
        let (instruction, x1, y1, x2, y2) = parse_instruction(line);

        for y in y1..y2 + 1 {
            for x in x1..x2 + 1 {
                if instruction == Instruction::Toggle {
                    lights.entry((x, y)).and_modify(|v| *v += 2).or_insert(2);
                } else if instruction == Instruction::TurnOff {
                    let v = lights.entry((x, y)).or_insert(0);

                    if *v == 0 {
                        lights.remove(&(x, y));
                    } else {
                        *v -= 1;
                    }
                } else {
                    lights.entry((x, y)).and_modify(|v| *v += 1).or_insert(1);
                }
            }
        }
    }

    for value in lights.values() {
        total += value;
    }

    total
}

fn parse_instruction(line: &str) -> (Instruction, usize, usize, usize, usize) {
    let parts: Vec<_> = line.split_whitespace().collect();

    if line.starts_with("toggle") {
        let from: Vec<_> = parts[1].split(',').collect();
        let to: Vec<_> = parts[3].split(',').collect();
        let x1: usize = from[0].parse().unwrap();
        let x2: usize = to[0].parse().unwrap();
        let y1: usize = from[1].parse().unwrap();
        let y2: usize = to[1].parse().unwrap();

        (Instruction::Toggle, x1, y1, x2, y2)
    } else {
        let from: Vec<_> = parts[2].split(',').collect();
        let to: Vec<_> = parts[4].split(',').collect();
        let x1: usize = from[0].parse().unwrap();
        let x2: usize = to[0].parse().unwrap();
        let y1: usize = from[1].parse().unwrap();
        let y2: usize = to[1].parse().unwrap();

        if line.starts_with("turn off") {
            (Instruction::TurnOff, x1, y1, x2, y2)
        } else {
            (Instruction::TurnOn, x1, y1, x2, y2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_instruction() {
        let mut l = "toggle 1,2 through 10,20";
        assert_eq!(parse_instruction(l), (Instruction::Toggle, 1, 2, 10, 20));

        l = "turn on 3,4 through 30,40";
        assert_eq!(parse_instruction(l), (Instruction::TurnOn, 3, 4, 30, 40));

        l = "turn off 5,6 through 50,60";
        assert_eq!(parse_instruction(l), (Instruction::TurnOff, 5, 6, 50, 60));
    }

    #[test]
    fn it_works() {
        let mut input = "turn on 0,0 through 999,999";
        assert_eq!(y15d06p1(input), 1000000);

        input = "turn on 0,0 through 0,99\ntoggle 0,0 through 0,999\n";
        assert_eq!(y15d06p1(input), 900);

        input = concat!(
            "turn on 0,0 through 999,999\n",
            "toggle 0,0 through 999,0\n",
            "turn off 499,499 through 500,500\n"
        );
        assert_eq!(y15d06p1(input), 998996);

        input = "turn on 0,0 through 0,0\ntoggle 0,0 through 999,999";
        assert_eq!(y15d06p2(input), 2000001);
    }

    #[test]
    #[ignore]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day06.txt").unwrap();

        assert_eq!(y15d06p1(&contents), 543903);
        assert_eq!(y15d06p2(&contents), 14687245);
    }
}
