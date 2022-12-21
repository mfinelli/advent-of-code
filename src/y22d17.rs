use std::collections::HashSet;
use std::collections::BinaryHeap;

const SHAPES: [&[(u32, u32)]; 5] = [
    &[(1, 0), (2, 0), (3, 0), (4, 0)], // horizontal line
    &[(2, 0), (1, 1), (2, 1), (3, 1), (2, 2)], // cross
    &[(3, 2), (3, 1), (1, 0), (2, 0), (3, 0)], // "J"
    &[(1, 0), (1, 1), (1, 2), (1, 3)], // vertical line
    &[(1, 0), (2, 0), (1, 1), (2, 1)], // square
];

pub fn y22d17(input: &str) -> u32 {
    let chars: Vec<_> = input.trim().chars().collect();
    let mut heights = BinaryHeap::new();

    let mut shapes = SHAPES.iter().enumerate().cycle();
    let mut jets = chars.iter().enumerate().cycle();

    // tracks world state; initialize a floor
    let mut cave = HashSet::from([(1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0)]);
    let mut y_start = 4;

    for i in 0..2022 {
        let Some((block_i, shape)) = shapes.next() else { panic!("cycle!") };

        // let mut shape: &[(u32, u32)] = &shape.clone().iter().map(|(x, y)| (x+2, y+4)).collect::<Vec<(u32, u32)>>();
        let mut shape = shape.clone().iter().map(|(x, y)| (x+2, y+y_start)).collect::<Vec<(u32, u32)>>();
        // println!("shape start: {:?}", shape);
        // print_state(&cave, &shape);

        let mut j = 0;
        loop {
            let Some((jet_i, jet)) = jets.next() else { panic!("cycle!") };
            // let width = shape_width(shape);
            //

            if *jet == '<' {
                // println!("attempt to move left");
            } else {
                // println!("attempt to move right");
            }


            if *jet == '<' && can_move(&cave, &shape, -1, 0){
                shape = do_move(&cave, &shape, -1, 0);
                // println!("moved left: {:?}", shape);
                // print_state(&cave, &shape);
            } else if *jet == '>' && can_move(&cave, &shape, 1, 0) {
                shape = do_move(&cave, &shape, 1, 0);
                // println!("moved right: {:?}", shape);
                // print_state(&cave, &shape);
            }

            if can_move(&cave, &shape, 0, -1) {
                shape = do_move(&cave, &shape, 0, -1);
                // println!("moved down: {:?}", shape);
                // print_state(&cave, &shape);
            } else {
                // add the shape to the cave and move to the next piece
                // println!("hit bottom: {:?}", shape);
                for (x,y) in shape {
                    heights.push(y);
                    cave.insert((x, y));
                    y_start = *heights.peek().unwrap() + 4;
                }

                // println!("cave: {:?}", cave);
                // print_state(&cave, &[(0, y_start)]);
                // println!("new y start: {}\n", y_start);

                break;
            }


            // if j == 2 {
            //     break;
            // }
            //  j+=1;
        }

        // if i == 10 {
        //     break;
        // }
    }

    // println!("{:?}", cave);

    heights.pop().unwrap()
}

fn can_move(cave: &HashSet<(u32, u32)>, shape: &[(u32, u32)], x_offset: i32, y_offset: i32) -> bool {
    let shape = shape.clone().iter().map(|(x, y)| {
        let ix: i32 = (*x).try_into().unwrap();
        let iy: i32 = (*y).try_into().unwrap();
        ((ix+x_offset).try_into().unwrap(), (iy +y_offset).try_into().unwrap())
    }).collect::<Vec<(u32, u32)>>();

    // println!("attempted move to: {:?}", shape);

    if shape.iter().any(|(x, _)| *x == 0 || *x == 8) {
        // we hit a wall and can't move
        return false;
    }

    if shape.iter().any(|point| cave.contains(point)) {
        // we hit another shape (or the floor) and can't move
        return false;
    }

    true
}

fn do_move(cave: &HashSet<(u32, u32)>, shape: &[(u32, u32)], x_offset: i32, y_offset: i32) -> Vec<(u32, u32)> {
    shape.clone().iter().map(|(x, y)| {
        let ix: i32 = (*x).try_into().unwrap();
        let iy: i32 = (*y).try_into().unwrap();
        ((ix+x_offset).try_into().unwrap(), (iy +y_offset).try_into().unwrap())
    }).collect::<Vec<(u32, u32)>>()
}

fn print_state(cave: &HashSet<(u32, u32)>, shape: &[(u32, u32)]) {
    let mut s = String::new();

    let mut i = *shape.clone().iter().map(|(_,y)| y).max().unwrap();
    while i > 0 {
        for j in 1..8 {
            if cave.contains(&(j, i)) {
                s += "#";
            } else if shape.contains(&(j, i)) {
                s += "@";
            } else {
                s += ".";
            }
        }
        s += "\n";
        i -= 1;
    }

    s += "-------\n";
    println!("{}", s);
}


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

        assert_eq!(y22d17(&contents), 3071);
    }
}
