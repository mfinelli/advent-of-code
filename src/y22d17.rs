const SHAPES: [&[(u32, u32)]; 5] = [
    &[(0, 0), (1, 0), (2, 0), (3, 0)], // horizontal line
    &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], // cross
    &[(2, 2), (2, 1), (0, 0), (1, 0), (2, 0)], // "J"
    &[(0, 0), (0, 1), (0, 2), (0, 3)], // vertical line
    &[(0, 0), (1, 0), (0, 1), (1, 1)], // square
];

pub fn y22d17(input: &str) -> u32 {
    let chars: Vec<_> = input.trim().chars().collect();
    let mut height = 0;

    let mut shapes = SHAPES.iter().enumerate().cycle();
    let mut jets = chars.iter().enumerate().cycle();

    // tracks world state; initialize a floor
    let mut cave = [&[(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0)]];

    for i in 0..2022 {
        let Some((block_i, shape)) = shapes.next() else { panic!("cycle!") };
        let Some((jet_i, jet)) = jets.next() else { panic!("cycle!") };
        let width = shape_width(shape);

        let shape: &[(u32, u32)] = &shape.clone().iter().map(|(x, y)| (x+2, y+4)).collect::<Vec<(u32, u32)>>();
        // println!("{:?}", shape);

        if i == 2 {
            break;
        }
    }

    height
}

// fn can_move(cave: [&[(u32, u32)]

/// Compute the width of a given shape.
fn shape_width(shape: &[(u32, u32)]) -> u32 {
    // get the maximum of all the x coordinates and then add one
    shape.iter().map(|(x,_)| x).max().unwrap() + 1
}

/// Compute the height of a given shape.
fn shape_height(shape: &[(u32, u32)]) -> u32 {
    // get the maximum of all the y coordinates and then add one
    shape.iter().map(|(_, y)| y).max().unwrap() + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_shape_star() {
        let mut shape = [(0, 0), (0, 1), (0, 2), (0, 3)];
        assert_eq!(shape_width(&shape), 1);
        assert_eq!(shape_height(&shape), 4);

        shape = [(0, 0), (1, 0), (2, 0), (3, 0)];
        assert_eq!(shape_width(&shape), 4);
        assert_eq!(shape_height(&shape), 1);
    }

    #[test]
    fn it_works() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        assert_eq!(y22d17(input), 3068);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2022/day17.txt").unwrap();

        assert_eq!(y22d17(&contents), 0);
    }
}
