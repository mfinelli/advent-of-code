use regex::Regex;
use std::collections::{BinaryHeap, HashMap};

pub fn y22d16(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let flow_regex = Regex::new(r"^rate=(\d+);$").unwrap();

    let mut rates: HashMap<&str, u32> = HashMap::new();
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut distances: HashMap<&str, HashMap<&str, Option<i32>>> = HashMap::new();

    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();
        let flow_captures = flow_regex.captures(parts[4]).unwrap();

        let mut room_connections = Vec::new();

        for i in 9..parts.len() {
            room_connections.push(parts[i].trim_end_matches(','));
        }

        rates.insert(parts[1], flow_captures[1].parse().unwrap());
        connections.insert(parts[1], room_connections);
    }

    for (room,rate) in &rates {
        let mut other_distances: HashMap<&str, Option<i32>> = HashMap::new();

        for (other_room,other_rate) in &rates {
            if room == other_room {
                other_distances.insert(other_room, Some(0));
            } else if connections[room].contains(other_room) {
                other_distances.insert(other_room, Some(1));
            } else {
                other_distances.insert(other_room, None);
            }
        }

        distances.insert(room, other_distances);
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
                                            distances.get_mut(i).map(|val| val.insert(j, Some(ik + kj)));
                                        }
                                    },
                                    None => {
                                        // kj is infinity so ik + kj is infinity which is always
                                        // greater than ij (even if that's infinity)
                                    },
                                }
                            },
                            None => {
                                // ik is infinity so ik + kj is infinity which is always
                                // greater than ij (even if that's infinity)
                            },
                        }
                    }
                    None => {
                        match ik {
                            Some(ik) => {
                                match kj {
                                    Some(kj) => {
                                        // ik and kj are _not_ infinity but ik _is_
                                        // so ik + kj will always be less than
                                        distances.get_mut(i).map(|val| val.insert(j, Some(ik + kj)));
                                    },
                                    None => {
                                        // kj is infinity so ik + kj is infinity which is always
                                        // greater than ij (even if that's infinity)
                                    },
                                }
                            },
                            None => {
                                // ik is infinity so ik + kj is infinity which is always
                                // greater than ij (even if that's infinity)
                            },
                        }
                    }
                }
            }
        }
    }

    let mut positive_flows: HashMap<&str, u32> = HashMap::new();
    for (room, rate) in &rates {
        if *rate > 0 {
            positive_flows.insert(room, *rate);
        }
    }

    println!("{:?}", rates);
    println!("{:?}", connections);
    println!("{:?}", distances);
    println!("{:?}", positive_flows);

    let start = HashMap::new();
    let paths = dfs(&distances, &rates, &positive_flows, 30, "AA", &mut start);
    println!("{:?}", paths);


    0
}

// fn dfs(distances: &HashMap<&str, HashMap<&str, Option<i32>>>, rates: &HashMap<&str, u32>, positive_flows: &HashMap<&str, u32>, time_remaining: i32, current_room: &str, opened: &HashMap<&str, i32>) -> Vec<&HashMap<&str, i32>> {
fn dfs<'a>(distances: &'a HashMap<&'a str, HashMap<&'a str, Option<i32>>>, rates: &'a HashMap<&'a str, u32>, positive_flows: &'a HashMap<&'a str, u32>, time_remaining: i32, current_room: &'a str, opened: &'a mut HashMap<&'a str, i32>) -> Vec<&'a mut HashMap<&'a str, i32>> {
    let mut results = Vec::new();
    results.push(opened);

    if time_remaining < 2 {
        return results;
    }

    // let opened_keys: Vec<_> = opened.keys().collect();

    for (room, rate) in positive_flows {
        let mut my_opened = opened.clone();
        if !my_opened.contains_key(room) {
        // if !opened_keys.contains(&room) {
            let new_time_remaining = time_remaining - (distances[current_room][room].unwrap() + 1);
            my_opened.insert(room, new_time_remaining);
            // positive_flows.remove(room);
            let mut new_results = dfs(distances, rates, positive_flows, new_time_remaining, room, &mut my_opened);
            results.append(&mut new_results);
        }
    }

    // if time_remaining != 1 {
    //     let mut new_results = dfs(distances, rates, positive_flows, 1, "BB", opened);
    //     results.append(&mut new_results);
    // }

    return results;
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

        assert_eq!(y22d16(input), 1651);
    }

    #[test]
    #[ignore]
    fn the_solution() {
        let contents = fs::read_to_string("input/2022/day16.txt").unwrap();

        assert_eq!(y22d16(&contents), 0);
    }
}
