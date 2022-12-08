use std::collections::BinaryHeap;

pub fn y22d08(input: &str, part: u32) -> u32 {
    let grid = parse_input(input);

    // calculate the outer edge which is always visible
    let mut total = 2 * grid.len() as u32 + 2 * (grid[0].len() as u32 - 2);

    let mut scenic_scores = BinaryHeap::new();

    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            println!("considering {},{}: {}", x, y, grid[y][x]);

            let mut visible_from_left = true;
            let mut visible_to_left = 0;
            for left in (0..x).rev() {
                println!(
                    "comparing to the left {},{}: {}",
                    left, y, grid[y][left]
                );
                visible_to_left += 1;
                if grid[y][left] >= grid[y][x] {
                    println!("not visible from the left");
                    visible_from_left = false;
                    break;
                }
            }

            if visible_from_left {
                total += 1;
                println!("visible from the left");
                // continue;
            }

            println!("{} visible to the left", visible_to_left);

            let mut visible_from_right = true;
            let mut visible_to_right = 0;
            for right in x + 1..grid[0].len() {
                println!(
                    "comparing to the right {},{}: {}",
                    right, y, grid[y][right]
                );
                visible_to_right += 1;
                if grid[y][right] >= grid[y][x] {
                    println!("not visible from the right");
                    visible_from_right = false;
                    break;
                }
            }

            if !visible_from_left && visible_from_right {
                total += 1;
                println!("visible from the right");
                // continue;
            }

            println!("{} visible to the right", visible_to_right);

            let mut visible_from_top = true;
            let mut visible_to_top = 0;
            for top in (0..y).rev() {
                println!(
                    "comparing to the top {},{}: {}",
                    x, top, grid[top][x]
                );
                visible_to_top += 1;
                if grid[top][x] >= grid[y][x] {
                    println!("not visible from the top");
                    visible_from_top = false;
                    break;
                }
            }

            if !visible_from_left && !visible_from_right && visible_from_top {
                total += 1;
                println!("visible from the top");
                // continue;
            }

            println!("{} visible to the top", visible_to_top);

            let mut visible_from_bottom = true;
            let mut visible_to_bottom = 0;
            for bottom in y + 1..grid.len() {
                println!(
                    "comparing to the bottom {},{}: {}",
                    x, bottom, grid[bottom][x]
                );
                visible_to_bottom += 1;
                if grid[bottom][x] >= grid[y][x] {
                    println!("not visible from the bottom");
                    visible_from_bottom = false;
                    break;
                }
            }

            if !visible_from_left
                && !visible_from_right
                && !visible_from_top
                && visible_from_bottom
            {
                total += 1;
                println!("visible from the bottom");
                // continue;
            }

            println!("{} visible to the bottom", visible_to_bottom);

            println!("not visible");

            let scenic_score = visible_to_left
                * visible_to_right
                * visible_to_top
                * visible_to_bottom;
            println!("scenic score: {}", scenic_score);

            println!("");

            scenic_scores.push(scenic_score);
        }
    }

    if part == 1 {
        total
    } else {
        scenic_scores.pop().unwrap()
    }
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
    use std::fs;

    #[test]
    fn it_works() {
        let input =
            concat!("30373\n", "25512\n", "65332\n", "33549\n", "35390",);

        assert_eq!(y22d08(input, 1), 21);
        assert_eq!(y22d08(input, 2), 8);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2022/day08.txt").unwrap();

        assert_eq!(y22d08(&contents, 1), 1807);
        assert_eq!(y22d08(&contents, 2), 480000);
    }
}
