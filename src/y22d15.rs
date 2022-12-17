use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn y22d15p1(input: &str, row: i32) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut rows: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    let mut beacons = HashSet::new();
    let r = Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();

    for line in lines {
        let captures = r.captures(line).unwrap();
        // println!("{:?}", captures);
        let sx: i32 = captures[1].parse().unwrap();
        let sy: i32 = captures[2].parse().unwrap();
        let bx: i32 = captures[3].parse().unwrap();
        let by: i32 = captures[4].parse().unwrap();

        if by == row {
            beacons.insert((bx, by));
        }

        // let mut range: HashSet<(i32, i32)> = HashSet::new();

        let md = (sx - bx).abs() + (sy - by).abs();

        for i in 0..md + 1 {
            let start_x = sx - md + i as i32;
            let end_x = sx + md - i as i32;
            let up_y = sy + i as i32;
            let down_y = sy - i as i32;

            // println!("run line from {} to {} on {}", start_x, end_x, up_y);
            // println!("run line from {} to {} on {}", start_x, end_x, down_y);
            // for x in start_x..end_x +1 {
            // range.insert((x, up_y));
            if up_y == row {
                let mut up_ranges = rows.entry(up_y).or_insert(Vec::new());
                up_ranges.push((start_x, end_x));
            }

            if i != 0 && down_y == row {
                let mut down_ranges = rows.entry(down_y).or_insert(Vec::new());
                down_ranges.push((start_x, end_x));
                // range.insert((x, down_y));
            }
            // }
        }
        // range.insert((sx,sy));

        // let mut i = 1;
        // loop {
        //     if range.contains(&(bx,by)) {
        //         break;
        //     }

        //     println!("i is {}", i);

        //     for (j,x) in (sx..sx+i).enumerate() {
        //         println!("need to write coord {},?", x);
        //         for y in sy..sy+i-j as i32 {
        //             println!("write coord {}, {}", x, y);
        //             range.insert((x, y));
        //         }

        //         println!("need to write coord {},? (p2)", x);
        //         for y in (sy-i+j as i32..sy).rev() {
        //             println!("write coord {}, {}", x, y+1);
        //             range.insert((x, y+1));
        //         }
        //     }

        //     for (j,x) in (sx-i..sx).rev().enumerate() {
        //         println!("need to write coord {},? (down)", x+1);
        //         for y in sy..sy+i-j as i32 {
        //             println!("write coord {}, {}", x+1, y);
        //             range.insert((x+1, y));
        //         }

        //         println!("need to write coord {},? (down p2)", x+1);
        //         for y in (sy-i+j as i32..sy).rev() {
        //             println!("write coord {}, {}", x+1, y+1);
        //             range.insert((x+1, y+1));
        //         }
        //     }

        //     i += 1;
        // }

        // sensors.insert((sx, sy), range);

        // println!("{:?}", sensors);
        // break;
    }

    // println!("{:?}", rows);
    let row = rows.get(&row).unwrap();
    let mut matches: HashSet<i32> = HashSet::new();
    // println!("{:?}", row);
    for r in row {
        let (start, end) = r;
        for x in *start..*end + 1 {
            matches.insert(x);
        }
    }

    // println!("{:?}", matches);

    // let mut matches = HashSet::new();
    // for (sensor, range) in sensors {
    //     for r in range {
    //         let (x,y) = r;
    //         if y == row {
    //             matches.insert(r);
    //         }
    //     }
    // }

    // subtract one to account for the actual beacon
    // println!("{:?}", beacons);
    matches.len() as u32 - beacons.len() as u32
    // 0
}

pub fn y22d15p2(input: &str, max: i32) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let r = Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();

    let mut sensors = HashMap::new();

    for line in lines {
        let captures = r.captures(line).unwrap();
        // println!("{:?}", captures);
        let sx: i32 = captures[1].parse().unwrap();
        let sy: i32 = captures[2].parse().unwrap();
        let bx: i32 = captures[3].parse().unwrap();
        let by: i32 = captures[4].parse().unwrap();

        let md = (sx-bx).abs() + (sy-by).abs();
        sensors.insert((sx,sy), md);
    }

    // println!("{:?}", sensors);

    for ((sx,sy), md) in &sensors {
        // if sx != &12 && sy != &14 {
        //     continue;
        // }

        for i in 0..md + 2 {
            let start_x = sx - md +i as i32;
            let end_x = sx +md -i as i32;
            let up_y = sy + i as i32;
            let down_y = sy - i as i32;

            // println!("sensor {},{} has border {},{}", sx, sy, start_x, up_y);
            // println!("sensor {},{} has border {},{}", sx, sy, start_x, down_y);
            // println!("sensor {},{} has border {},{}", sx, sy, end_x, up_y);
            // println!("sensor {},{} has border {},{}", sx, sy, end_x, down_y);

            let checks: [(i32, i32); 4] = [(start_x-1, up_y), (end_x+1, up_y), (start_x -1, down_y), (end_x +1,down_y)];

            for (x,y) in checks {
                if x < 0 || x > max || y < 0 || y > max {
                    continue;
                }

                // println!("sensor {},{} outside border: {},{}", sx, sy, x, y);

                // if x == 14 && y == 11 {
                //     println!("found 14,11: {:?},{:?} {:?}", sx,sy,md);
                // }

                let mut in_range = false;
                for ((ox,oy), omd) in &sensors {
                    if ox == sx && oy == sy {
                        continue;
                    }

                    if (ox-x).abs() + (y-oy).abs() < *omd {
                        // println!("{:?},{:?} is in range of {:?},{:?}", x,y,ox,oy);
                        in_range = true;
                        break;
                    }
                }

                if !in_range {
                    // println!("{},{} is not in range of any sensor", x,y);
                    return (x as i64 * 4000000 + y as i64).try_into().unwrap();
                    // break;
                }
            }
        }

    }



    // we shouldn't get here...

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = concat!(
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n",
            "Sensor at x=9, y=16: closest beacon is at x=10, y=16\n",
            "Sensor at x=13, y=2: closest beacon is at x=15, y=3\n",
            "Sensor at x=12, y=14: closest beacon is at x=10, y=16\n",
            "Sensor at x=10, y=20: closest beacon is at x=10, y=16\n",
            "Sensor at x=14, y=17: closest beacon is at x=10, y=16\n",
            "Sensor at x=8, y=7: closest beacon is at x=2, y=10\n",
            "Sensor at x=2, y=0: closest beacon is at x=2, y=10\n",
            "Sensor at x=0, y=11: closest beacon is at x=2, y=10\n",
            "Sensor at x=20, y=14: closest beacon is at x=25, y=17\n",
            "Sensor at x=17, y=20: closest beacon is at x=21, y=22\n",
            "Sensor at x=16, y=7: closest beacon is at x=15, y=3\n",
            "Sensor at x=14, y=3: closest beacon is at x=15, y=3\n",
            "Sensor at x=20, y=1: closest beacon is at x=15, y=3\n",
        );

        assert_eq!(y22d15p1(input, 10), 26);
        assert_eq!(y22d15p2(input, 20), 56000011);
    }

    #[test]
    #[ignore]
    fn the_solution() {
        let contents = fs::read_to_string("input/2022/day15.txt").unwrap();

        assert_eq!(y22d15p1(&contents, 2000000), 4717631);
        assert_eq!(y22d15p2(&contents, 4000000), 13197439355220);
    }
}
