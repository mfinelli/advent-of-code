pub fn y22d08(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut total = 0;

    let width = lines[0].len() - 1;
    let height = lines.len() - 1;

    for (y, line) in lines.iter().enumerate() {
        if y == 0{
            total += line.len() as u32;
            continue;
        } else if y == height {
            total += line.len() as u32;
            continue;
        }

        let chars: Vec<_> = line.chars().collect();

        for (x, ch) in chars.iter().enumerate() {
            if x == 0 {
                total += 1;
                continue;
            } else if x == width {
                total += 1;
                continue;
            }

            let tree: u32 = ch.to_string().parse().unwrap();
            let mut visible = true;

            for left in 0..x {
                let left_tree: u32 = chars[left].to_string().parse().unwrap();
                if left_tree >= tree {
                    println!("tree in position {}, {} ({}) is not visible because tree to the left {}, {} ({}) is taller", x+1, y+1, tree, left+1, y+1, left_tree);
                    visible = false;
                    break;
                }
            }

            if visible {
                println!("tree in position {}, {} is visible from the left", x+1, y+1);
                total += 1;
                continue;
            }

            for right in x+1..width {
                let right_tree: u32 = chars[right].to_string().parse().unwrap();
                if right_tree >= tree {
                    // println!("tree in position {}, {} is not visible because tree to the right ({}) is taller: {} >= {}", x+1, y+1, right+1, right_tree, tree);
                    println!("tree in position {}, {} ({}) is not visible because tree to the right {}, {} ({}) is taller", x+2, y+1, tree, right+1, y+1, right_tree);
                    visible = false;
                    break;
                }
            }

            if visible {
                println!("tree in position {}, {} is visible from the right", x+1, y+1);
                total += 1;
                continue;
            }

            for top in 0..y {
                let top_tree: u32 = lines[top].chars().collect::<Vec<_>>()[y].to_string().parse().unwrap();
                if top_tree >= tree {
                    // println!("tree in position {}, {} is not visible because tree to the top ({}) is taller: {} >= {}", x+1, y+1, top+1, top_tree, tree);
                    println!("tree in position {}, {} ({}) is not visible because tree to the top {}, {} ({}) is taller", x+1, y+1, tree, x+1, top+1, top_tree);
                    visible = false;
                    break;
                }
            }

            if visible {
                println!("tree in position {}, {} is visible from the top", x+1, y+1);
                total += 1;
                continue;
            }

            for bottom in y..height {
                let bottom_tree: u32 = lines[bottom].chars().collect::<Vec<_>>()[y].to_string().parse().unwrap();
                if bottom_tree >= tree {
                    // println!("tree in position {}, {} is not visible because tree to the bottom ({}) is taller: {} >= {}", x+1, y+1, bottom+1, bottom_tree, tree);
                    println!("tree in position {}, {} ({}) is not visible because tree to the bottom {}, {} ({}) is taller", x+1, y+1, tree, x+1, bottom+1, bottom_tree);
                    visible = false;
                    break;
                }
            }

            if visible {
                println!("tree in position {}, {} is visible from the bottom", x+1, y+1);
                total += 1;
                // continue;
            }
        }
    }

    total
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
