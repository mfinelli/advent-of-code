/* Copyright 2023 Mario Finelli
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Advent of Code 2023 Day 16: <https://adventofcode.com/2023/day/16>
//!
//! TODO

use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

/// The solution for the day sixteen challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y23d16::y23d16;
/// // probably read this from the input file...
/// let input = "";
/// assert_eq!(y23d16(input), 0);
/// ```
pub fn y23d16(input: &str, part: u32) -> u32 {
    let mut grid = HashMap::new();
    let rows = input.lines().collect::<Vec<_>>().len();
    let mut heap = BinaryHeap::new();
    let mut cols = 0;

    for (y, line) in input.lines().enumerate() {
        // let y: i32 = y.try_into().unwrap();

        for (x, c) in line.chars().enumerate() {
            if y == 0 {
                cols = x;
            }

            // let x: i32 = x.try_into().unwrap();
            grid.insert((x, y), c);
        }
    }

    cols += 1;

    if part == 1 {
        return send_light((0,0), 'R', &grid, rows, cols);
    }

    for x in 0..cols {
        heap.push(send_light((x, 0), 'D', &grid, rows, cols));
        heap.push(send_light((x, rows-1), 'U', &grid, rows, cols));
    }

    for y in 0..rows {
        heap.push(send_light((0, y), 'R', &grid, rows, cols));
        heap.push(send_light((cols-1, y), 'L', &grid, rows, cols));
    }



    heap.pop().unwrap()



    // 0
}

/// TODO
fn send_light(
    start: (usize, usize),
    direction: char,
    grid: &HashMap<(usize, usize), char>,
    rows: usize,
    cols: usize,
) -> u32 {
    let mut energized = HashSet::new();
    let mut queue = VecDeque::new();
    let mut visited = Vec::new();

    // send_light((-1, 0), 'R', &grid, &mut Vec::new(), &mut energized);
    queue.push_back((start, direction));

    // println!("{:?}", energized);
    //
    loop {
        match queue.pop_front() {
            None => break,
            Some(((x,y), direction)) => {
                // let ((x,y), direction) = item;
                // println!("visiting {}, {} ({})", x,y,direction);
                let tile = grid.get(&(x,y)).unwrap();
                energized.insert((x,y));

                if x < cols-1 && !visited.contains(&((x+1,y), 'R')) &&
                    ((direction == 'R' && (*tile == '.' || *tile == '-')) ||
                     (direction == 'U' && (*tile == '/' || *tile == '-')) ||
                     (direction == 'D' && (*tile == '\\' || *tile == '-'))) {
                        // println!("1: sending light {},{} ({})", x+1, y, 'R');
                        queue.push_back(((x+1, y), 'R'));
                        visited.push(((x+1, y), 'R'));
                }

                if y > 0 && !visited.contains(&((x, y-1), 'U')) &&
                    ((direction == 'U' && (*tile == '.' || *tile == '|')) ||
                     (direction == 'R' && (*tile == '/' || *tile == '|')) ||
                     (direction == 'L' && (*tile == '\\' || *tile == '|'))) {
                        // println!("2: sending light {},{} ({})", x, y-1, 'U');
                        queue.push_back(((x,y-1), 'U'));
                        visited.push(((x,y-1), 'U'));
                }

                if x > 0 && !visited.contains(&((x-1, y), 'L')) &&
                    ((direction == 'L' && (*tile == '.' || *tile == '-')) ||
                     (direction == 'U' && (*tile == '\\' || *tile == '-')) ||
                     (direction == 'D' && (*tile == '/' || *tile == '-'))) {
                        // println!("3: sending light {},{} ({})", x-1, y, 'L');
                        queue.push_back(((x-1,y), 'L'));
                        visited.push(((x-1,y), 'L'));
                }

                if y < rows-1 && !visited.contains(&((x, y+1), 'D')) &&
                    ((direction == 'D' && (*tile == '.' || *tile == '|')) ||
                     (direction == 'R' && (*tile == '\\' || *tile == '|')) ||
                     (direction == 'L' && (*tile == '/' || *tile == '|'))) {
                        // println!("4: sending light {},{} ({})", x, y+1, 'D');
                        queue.push_back(((x, y+1), 'D'));
                        visited.push(((x,y+1), 'D'));
                }
            },
        }
    }

    // energized.remove(&(-1, 0));
    energized.len().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_send_light() {}

    #[test]
    fn iit_works() {
        let input = concat!(
            ".|...\\....\n",
            "|.-.\\.....\n",
            ".....|-...\n",
            "........|.\n",
            "..........\n",
            ".........\\\n",
            "..../.\\\\..\n",
            ".-.-/..|..\n",
            ".|....-|.\\\n",
            "..//.|....\n",
        );

        assert_eq!(y23d16(input, 1), 46);
        assert_eq!(y23d16(input, 2), 51);
    }

    #[test]
    #[ignore]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day16.txt").unwrap();

        assert_eq!(y23d16(&contents, 1), 7236);
        assert_eq!(y23d16(&contents, 2), 7521);
    }
}
