use std::collections::HashSet;

pub fn y22d09(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut visited = HashSet::new();

    // set starting position
    let mut hx = 0;
    let mut hy = 0;
    let mut tx = 0;
    let mut ty = 0;
    visited.insert((tx, ty));

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let steps: u32 = parts[1].parse().unwrap();

        for _ in 0..steps {
            match parts[0] {
                "U" => hy += 1,
                "D" => hy -= 1,
                "L" => hx -= 1,
                "R" => hx += 1,
                _ => panic!("Invalid direction!"),
            }

            (tx, ty) = reconcile_tail(hx, hy, tx, ty);
            visited.insert((tx, ty));
        }
    }

    visited.len() as u32
}

/// Given an updated head position and the current tail position calculate if
/// tail needs to move and return its updated position.
fn reconcile_tail(hx: i32, hy: i32, tx: i32, ty: i32) -> (i32, i32) {
    if hx == tx && hy == ty {
        // head is overlapping tail; tail doesn't move
        return (tx, ty);
    }

    // head and tail share a row or column
    if hy == ty && hx > tx + 1 {
        // head has moved to the right
        return (tx + 1, ty);
    } else if hy == ty && hx < tx - 1 {
        // head has moved to the left
        return (tx - 1, ty);
    } else if hx == tx && hy > ty + 1 {
        // head has moved up
        return (tx, ty + 1);
    } else if hx == tx && hy < ty - 1 {
        // head has moved down
        return (tx, ty - 1);
    }

    // head and tail are on different rows and/or columns
    if hx > tx + 1 && hy > ty {
        // head is above tail and moved to the right;
        // tail moves up and to the right
        return (tx + 1, ty + 1);
    } else if hx > tx + 1 && hy < ty {
        // head is below tail and moved to the right;
        // tail moves down and to the right
        return (tx + 1, ty - 1);
    } else if hx < tx - 1 && hy > ty {
        // head is above tail and moved to the left;
        // tail moves up and to the left
        return (tx - 1, ty + 1);
    } else if hx < tx - 1 && hy < ty {
        // head is below tail and moved to the left;
        // tail moves down and to the left
        return (tx - 1, ty - 1);
    } else if hy > ty + 1 && hx > tx {
        // head is to the right of tail and moved up;
        // tail moves up and to the right
        return (tx + 1, ty + 1);
    } else if hy > ty + 1 && hx < tx {
        // head is to the left of tail and moved up;
        // tail moves up and to the left
        return (tx - 1, ty + 1);
    } else if hy < ty - 1 && hx > tx {
        // head is to the right of tail and moved down
        // tail moves down and to the right
        return (tx + 1, ty - 1);
    } else if hy < ty - 1 && hx < tx {
        // head is to the left of tail and moved down
        // tail moves down and to the left
        return (tx - 1, ty - 1);
    }

    // head and tail are still touching; tail doesn't move
    (tx, ty)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n";
        assert_eq!(y22d09(input), 13);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2022/day09.txt").unwrap();

        assert_eq!(y22d09(&contents), 6243);
    }
}
