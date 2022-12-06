pub fn y15d02(input: &str, part: u32) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut total = 0;

    for line in lines {
        let dimensions: Vec<&str> = line.split('x').collect();
        let l: u32 = dimensions[0].parse().unwrap();
        let w: u32 = dimensions[1].parse().unwrap();
        let h: u32 = dimensions[2].parse().unwrap();

        if part == 1 {
            let lw = l * w;
            let wh = w * h;
            let hl = h * l;

            let extra = if lw <= wh && lw <= hl {
                lw
            } else if wh <= lw && wh <= hl {
                wh
            } else {
                hl
            };

            total += 2 * lw + 2 * wh + 2 * hl + extra;
        } else {
            let volume = l * w * h;
            let mut sides = [l, w, h];
            sides.sort();

            for (i, side) in sides.iter().enumerate() {
                total += 2 * side;

                if i == 1 {
                    break;
                }
            }

            total += volume;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let mut input = "2x3x4\n";
        assert_eq!(y15d02(input, 1), 58);
        assert_eq!(y15d02(input, 2), 34);

        input = "1x1x10";
        assert_eq!(y15d02(input, 1), 43);
        assert_eq!(y15d02(input, 2), 14);

        input = "2x3x4\n1x1x10\n";
        assert_eq!(y15d02(input, 1), 101);
        assert_eq!(y15d02(input, 2), 48);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day02.txt").unwrap();

        assert_eq!(y15d02(&contents, 1), 1606483);
        assert_eq!(y15d02(&contents, 2), 3842356);
    }
}
