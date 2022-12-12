/* Copyright 2022 Mario Finelli
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

//! Advent of Code 2022 Day 12: <https://adventofcode.com/2022/day/12>
//!
//! TODO

use std::collections::{HashMap, HashSet, VecDeque};

/// The solution for the day twelve challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y22d12::y22d12;
/// // probably read this from the input file...
/// let mut input = "Sbcdef\nlkjihg\nmnopqr\nxwvuts\nyEzzzz\nzzzzza";
/// assert_eq!(y22d12(input, 1), Some(25));
/// assert_eq!(y22d12(input, 2), Some(25));
/// ```
pub fn y22d12(input: &str, part: u32) -> Option<u32> {
    let lines: Vec<_> = input.lines().collect();
    let mut grid = HashMap::new();
    let mut edges = HashMap::new();

    let mut end_positions = Vec::new();
    let mut to_visit = VecDeque::new();
    let mut visited = HashSet::new();
    let mut distances = HashMap::new();

    let mut start_position: (usize, usize) = (0, 0);

    for (y, line) in lines.iter().enumerate() {
        let chars: Vec<_> = line.chars().collect();
        for (x, c) in chars.iter().enumerate() {
            if c == &'S' {
                if part == 1 {
                    start_position = (x, y);
                } else {
                    end_positions.push((x, y));
                }
                grid.insert((x, y), 0);
            } else if c == &'E' {
                if part == 1 {
                    end_positions.push((x, y));
                } else {
                    start_position = (x, y);
                }
                grid.insert((x, y), 25);
            } else if c == &'a' {
                if part == 2 {
                    end_positions.push((x, y));
                }
                grid.insert((x, y), 0);
            } else {
                grid.insert(
                    (x, y),
                    match c {
                        'b' => 1,
                        'c' => 2,
                        'd' => 3,
                        'e' => 4,
                        'f' => 5,
                        'g' => 6,
                        'h' => 7,
                        'i' => 8,
                        'j' => 9,
                        'k' => 10,
                        'l' => 11,
                        'm' => 12,
                        'n' => 13,
                        'o' => 14,
                        'p' => 15,
                        'q' => 16,
                        'r' => 17,
                        's' => 18,
                        't' => 19,
                        'u' => 20,
                        'v' => 21,
                        'w' => 22,
                        'x' => 23,
                        'y' => 24,
                        'z' => 25,
                        _ => panic!("Unrecognized character!"),
                    },
                );
            }
        }
    }

    let (start_x, start_y) = start_position;

    for ((x, y), height) in &grid {
        let mut node_edges = Vec::new();
        if *x > 0 {
            if let Some(left) = grid.get(&(x - 1, *y)) {
                if (part == 1 && *left <= height + 1)
                    || (part == 2 && *left >= height - 1)
                {
                    node_edges.push((x - 1, *y));
                }
            }
        }
        if *y > 0 {
            if let Some(top) = grid.get(&(*x, y - 1)) {
                if (part == 1 && *top <= height + 1)
                    || (part == 2 && *top >= height - 1)
                {
                    node_edges.push((*x, y - 1));
                }
            }
        }
        if let Some(right) = grid.get(&(x + 1, *y)) {
            if (part == 1 && *right <= height + 1)
                || (part == 2 && *right >= height - 1)
            {
                node_edges.push((x + 1, *y));
            }
        }
        if let Some(bottom) = grid.get(&(*x, y + 1)) {
            if (part == 1 && *bottom <= height + 1)
                || (part == 2 && *bottom >= height - 1)
            {
                node_edges.push((*x, y + 1));
            }
        }
        if (x, y) == (&start_x, &start_y) {
            distances.insert((*x, *y), Some(0));
        } else {
            distances.insert((*x, *y), None);
        }
        edges.insert((x, y), node_edges);
    }

    to_visit.push_back((start_x, start_y));
    while let Some(edge) = to_visit.pop_front() {
        let (x, y) = edge;
        let distance = *distances.get(&edge).unwrap();

        if end_positions.contains(&edge) {
            return distance;
        }

        if !visited.contains(&edge) {
            visited.insert(edge);

            for node_edge in edges.get(&(&x, &y)).unwrap() {
                if !visited.contains(node_edge) {
                    let new_distance = distance.unwrap() + 1;
                    distances.insert(*node_edge, Some(new_distance));
                    to_visit.push_back(*node_edge);
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi\n";
        assert_eq!(y22d12(input, 1), Some(31));
        assert_eq!(y22d12(input, 2), Some(29));
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2022/day12.txt").unwrap();

        assert_eq!(y22d12(&contents, 1).unwrap(), 370);
        assert_eq!(y22d12(&contents, 2).unwrap(), 363);
    }
}
