use std::collections::HashSet;

pub fn y22d09(input: &str, number_of_knots: u32) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut visited = HashSet::new();
    let mut knots = Vec::new();

    // set starting position
    for _ in 0..number_of_knots {
        knots.push((0, 0));
    }
    let tail_index = knots.len() - 1;
    visited.insert(knots[tail_index]);

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let steps: u32 = parts[1].parse().unwrap();

        for _ in 0..steps {
            for i in 0..knots.len() {
                let (mut hx, mut hy) = knots[i];

                if i == 0 {
                    match parts[0] {
                        "U" => hy += 1,
                        "D" => hy -= 1,
                        "L" => hx -= 1,
                        "R" => hx += 1,
                        _ => panic!("Invalid direction!"),
                    }

                    knots[i] = (hx, hy);
                }

                if i != tail_index {
                    let (mut tx, mut ty) = knots[i + 1];
                    (tx, ty) = reconcile_tail(hx, hy, tx, ty);
                    knots[i + 1] = (tx, ty);
                }

                if i == tail_index {
                    visited.insert(knots[i]);
                }
            }
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
        let mut input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n";
        assert_eq!(y22d09(input, 2), 13);
        assert_eq!(y22d09(input, 10), 1);

        input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";
        assert_eq!(y22d09(input, 10), 36);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2022/day09.txt").unwrap();

        assert_eq!(y22d09(&contents, 2), 6243);
        assert_eq!(y22d09(&contents, 10), 2630);
    }
}
