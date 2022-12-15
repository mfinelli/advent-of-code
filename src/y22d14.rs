use std::collections::{BinaryHeap, HashSet};

pub fn y22d14(input: &str, part: u32) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut occupied = HashSet::new();
    let mut lowest = BinaryHeap::new();

    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();

        let coordinates: Vec<_> = parts[0].split(',').collect();
        let mut start_x: usize = coordinates[0].parse().unwrap();
        let mut start_y: usize = coordinates[1].parse().unwrap();
        lowest.push(start_y);

        for part in parts.iter().skip(1) {
            if part == &"->" {
                continue;
            }

            let coordinates: Vec<_> = part.split(',').collect();
            let mut end_x: usize = coordinates[0].parse().unwrap();
            let mut end_y: usize = coordinates[1].parse().unwrap();
            lowest.push(end_y);

            // println!("{:?}", coordinates);
            // println!("{:?} {:?} {:?} {:?}", start_x, start_y, end_x, end_y);

            let mut sorted_x: [usize; 2] = [start_x, end_x];
            let mut sorted_y: [usize; 2] = [start_y, end_y];
            sorted_x.sort();
            sorted_y.sort();

            for x in sorted_x[0]..sorted_x[1]+1 {
                for y in sorted_y[0]..sorted_y[1]+1 {
                    // println!("{:?}, {:?}", x, y);
                    occupied.insert((x,y));
                }
            }

            start_x = end_x;
            start_y = end_y;
        }
    }

    // println!("{:?}", occupied);

    let floor = if part ==1 {
        lowest.pop().unwrap()
    } else {
        lowest.pop().unwrap() + 2
    };

    // println!("floor is: {:?}", floor);
    let mut stop = false;
    let mut total = 0;
    while !stop {
        let mut sand: (usize, usize) = (500, 0);

        loop {
            let (x, y) = sand;

            if y > floor {
                occupied.insert((x, floor));
                occupied.insert((x-1, floor));
                occupied.insert((x+1, floor));
                total -= 1;
                break;
            }

            if part == 1 {
                let below: Vec<_> = occupied.iter().filter(|o| {
                    let (ox, oy) = o;
                    // println!("ox: {:?}, oy: {:?}, x: {:?}, y: {:?}", ox, oy, x, y);
                    ox == &x && oy > &y
                }).collect();

                if below.is_empty() {
                    // sand falls forever; we're done
                    // println!("sand is {:?} and it will fall forever", sand);
                    stop = true;
                    break;
                }
            }


            // println!("{:?}", occupied);
            // println!("{:?}", below);

            // if below.is_empty() {
            //     if part == 1 {
            //         // sand falls forever; we're done
            //         // println!("sand is {:?} and it will fall forever", sand);
            //         stop = true;
            //         break;
            //     } else {
            //         // add the floor and keep going
            //         occupied.insert((x, floor));
            //         occupied.insert((x-1, floor));
            //         occupied.insert((x+1, floor));
            //     }
            // }

            if !occupied.contains(&(x, y+1)) {
                sand = (x,y+1);
            } else if !occupied.contains(&(x-1, y+1)) {
                sand = (x-1,y+1);
            } else if !occupied.contains(&(x+1,y+1)) {
                sand = (x+1, y+1);
            } else {
                // println!("sand stopped at: {:?}", sand);
                break;
            }
        }

        if part == 2 && sand == (500, 0) {
            // the sand didn't move; we're done
            break;
        }

        occupied.insert(sand);
        total += 1
    }


    if part == 1{
        total -1
    } else {
        total +1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = concat!(
            "498,4 -> 498,6 -> 496,6\n",
            "503,4 -> 502,4 -> 502,9 -> 494,9\n"
            );

        assert_eq!(y22d14(input, 1), 24);
        assert_eq!(y22d14(input, 2), 93);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2022/day14.txt").unwrap();

        assert_eq!(y22d14(&contents, 1), 698);
        assert_eq!(y22d14(&contents, 1), 28594);
    }
}
