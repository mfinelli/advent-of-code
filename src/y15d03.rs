use std::collections::HashMap;

pub fn y15d03(input: &str, santas: u32) -> u32 {
    let chars: Vec<_> = input.trim().chars().collect();
    let mut houses = HashMap::new();
    houses.insert((0, 0), santas); // starting position gets a present

    let mut positions = Vec::new();

    for _ in 0..santas {
        positions.push((0,0));
    }

    for chunk in chars.chunks(santas as usize) {
        for santa in 0..santas {
            let (mut x, mut y) = positions[santa as usize];
            let c = chunk[santa as usize];

            if c == '<' {
                x -= 1;
            } else if c == '^' {
                y += 1;
            } else if c == '>' {
                x += 1;
            } else {
                y -= 1;
            }

            positions[santa as usize] = (x, y);

            let count = houses.entry((x, y)).or_insert(0);
            *count += 1;
        }
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
        assert_eq!(y15d03(input, 1), 2);

        input = "^>v<";
        assert_eq!(y15d03(input, 1), 4);

        input = "^v^v^v^v^v\n";
        assert_eq!(y15d03(input, 1), 2);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day03.txt").unwrap();

        assert_eq!(y15d03(&contents, 1), 2565);
        assert_eq!(y15d03(&contents, 2), 2639);
    }
}
