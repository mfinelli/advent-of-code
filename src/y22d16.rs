use regex::Regex;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Clone,Debug)]
struct Room {
    name: String,
    flow: u32,
    connections: Vec<String>,
}

pub fn y22d16(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let flow_regex = Regex::new(r"^rate=(\d+);$").unwrap();
    let mut rooms = Vec::new();

    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();

        let flow_captures = flow_regex.captures(parts[4]).unwrap();
        let mut connections = Vec::new();

        for i in 9..parts.len() {
            connections.push(parts[i].trim_end_matches(',').to_string());
        }


        let mut room = Room {
            name: parts[1].to_string(),
            flow: flow_captures[1].parse().unwrap(),
            connections: connections,
        };

        // println!("{:?}", room);
        rooms.push(room);

    }

    println!("{:?}", rooms);
    let starting_room = rooms.iter().find(|r| r.name == "AA").unwrap();
    println!("{:?}", starting_room);

    let mut distances = HashMap::new();
    for room in &rooms {
        let mut other_distances: HashMap<&String, Option<i32>> = HashMap::new();
        for other_room in &rooms {
            if &room.name == &other_room.name {
                other_distances.insert(&other_room.name, Some(0));
            } else if room.connections.contains(&other_room.name) {
                other_distances.insert(&other_room.name, Some(1));
            }else {
                other_distances.insert(&other_room.name, None);
            }
        }
        distances.insert(&room.name, other_distances);
    }

    // println!("{:?}", distances);
    // println!("{:?}", distances[&"AA".to_string()]);

    for k in &rooms {
        for i in &rooms {
            for j in &rooms {
                let ij = distances[&i.name][&j.name];
                let ik = distances[&i.name][&k.name];
                let kj = distances[&k.name][&j.name];

                match ij {
                    Some(ij) => {
                        match ik {
                            Some(ik) => {
                                match kj {
                                    Some(kj) => {
                                        if ij > ik + kj {
                                            distances.get_mut(&i.name).map(|val| val.insert(&j.name, Some(ik + kj)));
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
                                        distances.get_mut(&i.name).map(|val| val.insert(&j.name, Some(ik + kj)));
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

    println!("{:?}", distances);
    println!("{:?}", distances[&"AA".to_string()]);

    // for room in &starting_room.connections {

    // }

    let mut rooms_to_visit = Vec::new();
    for room in &rooms {
        if room.flow > 0 {
            rooms_to_visit.push(&room.name);
        }
    }

    // let mut closed = Vec::new();
    // for room in &rooms {
    //     closed.push(&room.name);
    // }

    // let paths = dfs(&distances, &rooms, &closed, 30, starting_room, Vec::new());
    let paths = dfs(&distances, &rooms, 30, starting_room, rooms_to_visit, Vec::new());
    println!("{:?}", paths);


    0
}

#[derive(Clone,Debug)]
struct RoomPath<'a>{
    name: &'a String,
    flow: u32,
    time_remaining: i32,
}

// fn dfs<'a>(distances: &'a HashMap<&'a String, HashMap<&'a String, Option<i32>>>, rooms: &'a Vec<Room>, closed: &Vec<&'a String>, time_remaining: i32, current: &'a Room, mut opened: Vec<RoomPath<'a>>) -> Vec<RoomPath<'a>> {
fn dfs<'a>(distances: &'a HashMap<&'a String, HashMap<&'a String, Option<i32>>>, rooms: &'a Vec<Room>, time_remaining: i32, current: &'a Room, rooms_to_open: Vec<&'a String>, path: Vec<RoomPath>) -> Vec<Vec<RoomPath<'a>>> {
    // let mut results = Vec::new();

    for room_to_open in &rooms_to_open {
        let room = rooms.iter().find(|r| &r.name == *room_to_open).unwrap();

        if *room_to_open == &current.name {
            continue;
        }

        let distance_to_room_from_current = distances[&current.name][&room.name].unwrap();
        let new_time = time_remaining - distance_to_room_from_current - 1;
        if new_time < 2 {
            // we won't have time to move to this room _and_ open it
            continue;
        }

        let mut new_rooms_to_open = Vec::new();
        for r in &rooms_to_open {
            if r != room_to_open {
                new_rooms_to_open.push(*r);
            }
        }
        // new_rooms_to_open.push(&current.name);

        let mut new_results = dfs(distances, rooms, new_time, room, new_rooms_to_open);
        // println!("{:?}", results);
        results.append(&mut results);
    }

    return results;






    // for closed_room in closed {
    //     let room = rooms.iter().find(|r| &r.name == *closed_room).unwrap();

    //     if room.flow == 0 || room.name == current.name {
    //         continue;
    //     }

    //     // let time = time_remaining - rooms.iter().find(|r| r.name == distances[&current.name][&room.name]).unwrap().flow;
    //     let distance_to_room_from_current = distances[&current.name][&room.name].unwrap();
    //     let new_time = time_remaining - distance_to_room_from_current - 1;
    //     if new_time < 2 {
    //         // we won't have time to move to this room _and_ open it
    //         continue;
    //     }

    //     let mut new_opened = opened.clone();
    //     new_opened.push(RoomPath {
    //         name: &room.name,
    //         flow: room.flow,
    //         time_remaining: new_time,
    //     });

    //     let mut new_closed = Vec::new();
    //     for r in closed {
    //         if r != closed_room {
    //             new_closed.push(*r);
    //         }
    //     }
    //     // for r in rooms {
    //     //     if r.name == room.name {
    //     //         continue;
    //     //     }

    //     //     new_closed.push(&r.name);
    //     // }

    //     println!("\nnew closed: {:?}", new_closed);
    //     println!("\nnew opened: {:?}", new_opened);

    //     let mut downstream_results = dfs(distances, &rooms, &new_closed, new_time, room, new_opened);
    //     println!("{:?}\n", downstream_results);
    //     opened.append(&mut downstream_results);
    // }

    // return opened;
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
