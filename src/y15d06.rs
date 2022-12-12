pub fn y15d06(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut on = 0;

    let mut lights = [[false; 1000]; 1000];

    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();

        if line.starts_with("toggle") {
            let from: Vec<_> = parts[1].split(',').collect();
            let to: Vec<_> = parts[3].split(',').collect();
            let x1: usize = from[0].parse().unwrap();
            let x2: usize = to[0].parse().unwrap();
            let y1: usize = from[1].parse().unwrap();
            let y2: usize = to[1].parse().unwrap();

            for y in y1..y2 + 1 {
                for x in x1..x2 + 1 {
                    if lights[y][x] {
                        lights[y][x] = false;
                    } else {
                        lights[y][x] = true;
                    }
                }
            }
        } else if line.starts_with("turn off") {
            let from: Vec<_> = parts[2].split(',').collect();
            let to: Vec<_> = parts[4].split(',').collect();
            let x1: usize = from[0].parse().unwrap();
            let x2: usize = to[0].parse().unwrap();
            let y1: usize = from[1].parse().unwrap();
            let y2: usize = to[1].parse().unwrap();

            for y in y1..y2 + 1 {
                for x in x1..x2 + 1 {
                    lights[y][x] = false;
                }
            }
        } else if line.starts_with("turn on") {
            let from: Vec<_> = parts[2].split(',').collect();
            let to: Vec<_> = parts[4].split(',').collect();
            let x1: usize = from[0].parse().unwrap();
            let x2: usize = to[0].parse().unwrap();
            let y1: usize = from[1].parse().unwrap();
            let y2: usize = to[1].parse().unwrap();

            for y in y1..y2 + 1 {
                for x in x1..x2 + 1 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let mut input = "turn on 0,0 through 999,999";
        assert_eq!(y15d06(input), 1000000);

        input = "turn on 0,0 through 0,99\ntoggle 0,0 through 0,999\n";
        assert_eq!(y15d06(input), 900);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day06.txt").unwrap();

        assert_eq!(y15d06(&contents), 543903);
    }
}
