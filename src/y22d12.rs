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
//! When first reading the challenge prompt I saw "find the shortest path" and
//! immediately thought of [Dijkstra's
//! algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm) but as I
//! was re-familiarizing myself with it I got confused by the weights in the
//! algorithm because the challenge prompt essentially defines that the weight
//! of all edges are `1`. So, I instead started implementing a brute-force
//! _depth_-first search that worked for the small sample input that was
//! provided, but took way too long to run on the actual input. I decided to
//! take another look at Dijkstra's algorithm because I must have been missing
//! something. After rewriting my mess, I was able to solve part one in a
//! reasonable amount of time, but the additional challenge in part two took
//! my solution more than five minutes to run even if it ultimately returned
//! the correct answer. This caused me to do some reading on the Advent of Code
//! subreddit, and I discovered two things: firstly, that because all of the
//! weights were equal a simple [breadth-first
//! search](https://en.wikipedia.org/wiki/Breadth-first_search) would be
//! sufficient to solve the problem, and secondly, that a neat trick to solve
//! part two was to start at the end and search backwards to find a suitable
//! start point (as opposed to looping through all of the valid starting points
//! and computing the shortest path to the end for all of them and then taking
//! the shortest of those as I was currently doing). I switched my solution to
//! use these two strategies and now it runs significantly faster.

use std::collections::{HashMap, HashSet, VecDeque};

/// The solution for the day twelve challenge.
///
/// We start by parsing the input which is provided as a string. The second
/// argument determines if we are looking for a solution from the start to the
/// end (part `1`) or if we're looking for the shortest valid starting position
/// (height `a`/`0`) to the end (part `2`). To actually parse the input we
/// enumerate each character in each line to use as the coordinates. Based on
/// the character found we also assign the height: `S`, and `a` have height
/// `0`, `E`, and `z` have height `25`, and every other letter has its height
/// in between. Additionally, we do some special handing for the `S`, `E`, and
/// `a` characters. When we find `S` and we're on part `1` then we also need
/// to initialize the starting position; if we're on part `2` then we instead
/// need to add it as a valid ending position. Similarly, when we find `E` on
/// part `1` we need to assign it as (the only) end position while in part `2`
/// it gets assigned as the starting position. Finally, when we encounter `a`
/// during part `2` we need to add it as a valid ending position.
///
/// Once we have initialized our grid, we need to determine determine to which
/// nodes it's possible to traverse from every other node. Keeping in mind the
/// trick above for part `2`, we essentially look at every node in the grid.
/// Then we look in each of the four directions (unless we're on the top or
/// left edge in which case we skip those two directions since we can't have a
/// negative `usize` in rust) and if a node exists (meaning that we're also not
/// on the _other_ edge) then we check its height. In part one we check to see
/// if the height is less than or equal to the height of the current node plus
/// one. Using the trick above for part two we check if the height is greater
/// than or equal to the current node minus one. Any other nodes that are valid
/// paths from the original node get added to the original node's edge list.
///
/// One last step as we are analyzing all of the nodes to determine the valid
/// neighbors: we check to see if we're on the starting node or not. If we are
/// then we initialize the distances hash with `0` (`Some(0)`), if we are not
/// then we initialize the distance as `None`.
///
/// Finally, it's time to actually perform the search. We use a
/// [`std::collections::VecDeque`] to keep track of the nodes that we need to
/// visit as it allows us to visit them in order that they are seen
/// (breadth-first) using the `push_back()` and `pop_front()` methods. While
/// there are still nodes left to visit we get the current distance associated
/// with the node that we are inspecting. If the node is in the list of end
/// positions then we're done, just return the distance. Otherwise, if we
/// haven't yet visited the node then we add it to our set of visited nodes and
/// then inspect all of its edges. If we also haven't visited the edge then
/// we update the distances hash for the edge with the current distance plus
/// one and then add the edge to the list of nodes to visit. If we get through
/// all of the nodes reachable from the start and never find a valid ending
/// node then we instead need to return `None`.
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

    // `grid` stores all of the points with their numeric heights and `edges`
    // stores the other nodes that are reachable from a given node
    let mut grid = HashMap::new();
    let mut edges = HashMap::new();

    let mut end_positions = Vec::new();
    let mut to_visit = VecDeque::new();
    let mut visited = HashSet::new();
    let mut distances = HashMap::new();

    // we initialize the start position or the compiler complains that it
    // could be uninitialized, but if we have valid input then that's not
    // possible; when we discover the real starting node we'll update this
    // variable
    let mut start_position: (usize, usize) = (0, 0);

    // parse the input to populate the grid, the starting position, and any
    // valid ending positions
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

    // calculate which nodes are reachable from every other node: we can look
    // up, down, left or right and in part one to any of those with a height
    // less than or equal to the current height plus one, and in part two to
    // any of those with a height greater than or equal to the current height
    // minus one
    let (start_x, start_y) = start_position;
    for ((x, y), height) in &grid {
        let mut node_edges = Vec::new();

        // if x == 0 then we're at the left edge
        if *x > 0 {
            let left = grid.get(&(x - 1, *y)).unwrap();
            if (part == 1 && *left <= height + 1)
                || (part == 2 && *left >= height - 1)
            {
                node_edges.push((x - 1, *y));
            }
        }

        // if y == 0 then we're at the top edge
        if *y > 0 {
            let top = grid.get(&(*x, y - 1)).unwrap();
            if (part == 1 && *top <= height + 1)
                || (part == 2 && *top >= height - 1)
            {
                node_edges.push((*x, y - 1));
            }
        }

        // if right is `None` then we're at the right edge
        if let Some(right) = grid.get(&(x + 1, *y)) {
            if (part == 1 && *right <= height + 1)
                || (part == 2 && *right >= height - 1)
            {
                node_edges.push((x + 1, *y));
            }
        }

        // if bottom is `None` then we're at the bottom edge
        if let Some(bottom) = grid.get(&(*x, y + 1)) {
            if (part == 1 && *bottom <= height + 1)
                || (part == 2 && *bottom >= height - 1)
            {
                node_edges.push((*x, y + 1));
            }
        }

        // initialize the starting point distance with `0`, everything else
        // with `None`
        if (x, y) == (&start_x, &start_y) {
            distances.insert((*x, *y), Some(0));
        } else {
            distances.insert((*x, *y), None);
        }

        edges.insert((x, y), node_edges);
    }

    // actually do the BFS starting at the... starting... node
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
