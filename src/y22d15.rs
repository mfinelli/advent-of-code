use std::collections::HashMap;
use regex::Regex;

pub fn y22d15(input: &str, row: u32) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut sensors: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    let r = Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();

    for line in lines {
        let captures = r.captures(line).unwrap();
        // println!("{:?}", captures);
        let sx: i32 = captures[1].parse().unwrap();
        let sy: i32 = captures[2].parse().unwrap();
        let bx: i32 = captures[3].parse().unwrap();
        let by: i32 = captures[4].parse().unwrap();

        let mut range: Vec<(i32, i32)> = Vec::new();

        sensors.insert((sx, sy), range);




    }

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

        assert_eq!(y22d15(input, 10), 26);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2022/day15.txt").unwrap();

        // assert_eq!(y22d15(&contents, 2000000), 0);
    }
}
