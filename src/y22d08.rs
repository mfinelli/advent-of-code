pub fn y22d08(input: &str) -> u32 {
    let grid = parse_input(input);

    // calculate the outer edge which is always visible
    let mut total = 2 * grid.len() as u32 + 2 * (grid[0].len() as u32 - 2);

    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            println!("considering {},{}: {}", x, y, grid[y][x]);

            let mut visible = true;
            for left in 0..x {
                println!("comparing to the left {},{}: {}", left, y, grid[y][left]);
                if grid[y][left] >= grid[y][x] {
                    println!("not visible from the left");
                    visible = false;
                    break;
                }
            }

            if visible {
                total += 1;
                println!("visible from the left");
                println!("");
                continue;
            }

            visible = true;
            for right in x+1..grid[0].len() {
                println!("comparing to the right {},{}: {}", right, y, grid[y][right]);
                if grid[y][right] >= grid[y][x] {
                    println!("not visible from the right");
                    visible = false;
                    break;
                }
            }

            if visible {
                total += 1;
                println!("visible from the right");
                println!("");
                continue;
            }

            visible = true;
            for top in 0..y {
                println!("comparing to the top {},{}: {}", x, top, grid[top][x]);
                if grid[top][x] >= grid[y][x] {
                    println!("not visible from the top");
                    visible = false;
                    break;
                }
            }

            if visible {
                total += 1;
                println!("visible from the top");
                println!("");
                continue;
            }

            visible = true;
            for bottom in y+1..grid.len() {
                println!("comparing to the bottom {},{}: {}", x, bottom, grid[bottom][x]);
                if grid[bottom][x] >= grid[y][x] {
                    println!("not visible from the bottom");
                    visible = false;
                    break;
                }
            }

            if visible {
                total += 1;
                println!("visible from the bottom");
                println!("");
                continue;
            }

            println!("not visible");
            println!("");
        }
    }

    total
}

/// Parsing the input requires an extra n^2 time complexity since we need to
/// loop through every element, but I think it should save time overall as
/// otherwise we need to parse the character every time we do a check.
fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let mut grid = Vec::new();
    let lines: Vec<_> = input.lines().collect();
    for line in lines {
        let mut trees = Vec::new();
        let chars: Vec<_> = line.chars().collect();

        for c in chars {
            let height: u32 = c.to_string().parse().unwrap();
            trees.push(height);
        }

        grid.push(trees);
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = concat!(
            "30373\n",
            "25512\n",
            "65332\n",
            "33549\n",
            "35390",
        );

        assert_eq!(y22d08(input), 21);
    }
}
