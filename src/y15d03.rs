use std::collections::HashMap;

pub fn y15d03(input: &str) -> u32 {
    let chars: Vec<_> = input.trim().chars().collect();
    let mut houses = HashMap::new();

    let mut x = 0;
    let mut y = 0;
    houses.insert((0, 0), 1); // starting position gets a present

    for c in chars {
        if c == '<' {
            x -= 1;
        } else if c == '^' {
            y += 1;
        } else if c == '>' {
            x += 1;
        } else {
            y -= 1;
        }

        let count = houses.entry((x, y)).or_insert(0);
        *count += 1;
    }

    houses.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let mut input = ">\n";
        assert_eq!(y15d03(input), 2);

        input = "^>v<";
        assert_eq!(y15d03(input), 4);

        input = "^v^v^v^v^v\n";
        assert_eq!(y15d03(input), 2);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day03.txt").unwrap();

        assert_eq!(y15d03(&contents), 2565);
    }
}
