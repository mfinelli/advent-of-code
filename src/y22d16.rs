/* Copyright 2022-2024 Mario Finelli
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

//! Advent of Code 2022 Day 16: <https://adventofcode.com/2022/day/16>
//!
//! TODO

use regex::Regex;
use std::collections::{BinaryHeap, HashMap, HashSet};

/// The solution for the day sixteen challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y22d16::y22d16;
/// ```
pub fn y22d16(input: &str, part: u32) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let flow_regex = Regex::new(r"^rate=(\d+);$").unwrap();

    let mut rates: HashMap<String, u32> = HashMap::new();
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    let mut distances: HashMap<String, HashMap<String, Option<i32>>> =
        HashMap::new();

    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();
        let flow_captures = flow_regex.captures(parts[4]).unwrap();

        let mut room_connections = Vec::new();

        for i in 9..parts.len() {
            room_connections.push(parts[i].trim_end_matches(',').to_string());
        }

        rates.insert(parts[1].to_string(), flow_captures[1].parse().unwrap());
        connections.insert(parts[1].to_string(), room_connections);
    }

    for (room, _rate) in &rates {
        let mut other_distances: HashMap<String, Option<i32>> = HashMap::new();

        for (other_room, _other_rate) in &rates {
            if room == other_room {
                other_distances.insert(other_room.to_string(), Some(0));
            } else if connections[room].contains(other_room) {
                other_distances.insert(other_room.to_string(), Some(1));
            } else {
                other_distances.insert(other_room.to_string(), None);
            }
        }

        distances.insert(room.to_string(), other_distances);
    }

    // now compute the distance from every node to every other node
    for (k, _) in &rates {
        for (i, _) in &rates {
            for (j, _) in &rates {
                let ij = distances[i][j];
                let ik = distances[i][k];
                let kj = distances[k][j];

                match ij {
                    Some(ij) => {
                        match ik {
                            Some(ik) => {
                                match kj {
                                    Some(kj) => {
                                        if ij > ik + kj {
                                            distances.get_mut(i).map(|val| {
                                                val.insert(
                                                    j.to_string(),
                                                    Some(ik + kj),
                                                )
                                            });
                                        }
                                    }
                                    None => {
                                        // kj is infinity so ik + kj is
                                        // infinity which is always greater
                                        // than ij (even if that's infinity)
                                    }
                                }
                            }
                            None => {
                                // ik is infinity so ik + kj is infinity which
                                // is always greater than ij (even if that's
                                // infinity)
                            }
                        }
                    }
                    None => {
                        match ik {
                            Some(ik) => {
                                match kj {
                                    Some(kj) => {
                                        // ik and kj are _not_ infinity but ij
                                        // _is_ so ik + kj will always be less
                                        distances.get_mut(i).map(|val| {
                                            val.insert(
                                                j.to_string(),
                                                Some(ik + kj),
                                            )
                                        });
                                    }
                                    None => {
                                        // kj is infinity so ik + kj is
                                        // infinity which is always greater
                                        // than ij (even if that's infinity)
                                    }
                                }
                            }
                            None => {
                                // ik is infinity so ik + kj is infinity which
                                // is always greater than ij (even if that's
                                // infinity)
                            }
                        }
                    }
                }
            }
        }
    }

    let mut positive_flows: HashSet<String> = HashSet::new();
    for (room, rate) in &rates {
        if *rate > 0 {
            positive_flows.insert(room.to_string());
        }
    }

    if part == 1 {
        dfs(
            &distances,
            &rates,
            positive_flows,
            "AA".to_string(),
            30,
            false,
        )
    } else {
        dfs(
            &distances,
            &rates,
            positive_flows,
            "AA".to_string(),
            26,
            true,
        )
    }
}

fn dfs(
    distances: &HashMap<String, HashMap<String, Option<i32>>>,
    rates: &HashMap<String, u32>,
    positive_flows: HashSet<String>,
    current_room: String,
    time_remaining: i32,
    help: bool,
) -> u32 {
    let mut paths: BinaryHeap<u32> = BinaryHeap::new();
    paths.push(0); // TODO: remove

    for room in &positive_flows {
        let new_time =
            time_remaining - (distances[&current_room][room].unwrap() + 1);
        let mut positive_flows = positive_flows.clone();
        positive_flows.remove(room);

        if time_remaining > new_time && new_time > 0 {
            let new_time_pos: u32 = new_time.try_into().unwrap();
            let rate = rates[room] * new_time_pos
                + dfs(
                    distances,
                    rates,
                    positive_flows,
                    room.to_string(),
                    new_time,
                    help,
                );
            paths.push(rate);
        }
    }

    // with the given rooms remaining, calculate most pressure that could be
    // released by the other person (elephant)
    if help {
        paths.push(dfs(
            distances,
            rates,
            positive_flows,
            "AA".to_string(),
            26,
            false,
        ));
    }

    paths.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = concat!(
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\n",
            "Valve BB has flow rate=13; tunnels lead to valves CC, AA\n",
            "Valve CC has flow rate=2; tunnels lead to valves DD, BB\n",
            "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE\n",
            "Valve EE has flow rate=3; tunnels lead to valves FF, DD\n",
            "Valve FF has flow rate=0; tunnels lead to valves EE, GG\n",
            "Valve GG has flow rate=0; tunnels lead to valves FF, HH\n",
            "Valve HH has flow rate=22; tunnel leads to valve GG\n",
            "Valve II has flow rate=0; tunnels lead to valves AA, JJ\n",
            "Valve JJ has flow rate=21; tunnel leads to valve II\n",
        );

        assert_eq!(y22d16(input, 1), 1651);
        assert_eq!(y22d16(input, 2), 1707);
    }

    #[test]
    #[ignore]
    fn the_solution() {
        let contents = fs::read_to_string("input/2022/day16.txt").unwrap();

        assert_eq!(y22d16(&contents, 1), 2124);
        // TODO: optimize this... it takes almost 10m even with a release build
        // assert_eq!(y22d16(&contents, 2), 2775);
    }
}
