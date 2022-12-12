use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub fn y22d12(input: &str, part: u32) -> Option<u32> {
    let lines: Vec<_> = input.lines().collect();
    let mut grid = HashMap::new();
    let mut edges = HashMap::new();
    // let mut shortest_paths = BinaryHeap::new();
    let mut shortest_path: Option<u32> = None;
    let mut start_position: (usize, usize) = (0,0);
    let mut end_position: (usize, usize) = (0,0);

    for (y, line) in lines.iter().enumerate() {
        let chars: Vec<_> = line.chars().collect();
        for (x, c) in chars.iter().enumerate() {
            if c == &'S' {
                start_position = (x, y);
                grid.insert((x, y), 0);
            } else if c == &'E' {
                end_position = (x, y);
                grid.insert((x, y), 25);
                // grid.insert((x, y), 2);
            } else {
                grid.insert((x, y), match c {
                    'a' => 0,
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
                });
            }
        }
    }

    let mut search_spots = Vec::new();
    if part == 1 {
        search_spots.push(start_position);
    } else {
        for ((x,y), height) in &grid {
            if *height == 0 {
                search_spots.push((*x,*y));
            }
        }
    }

    // println!("search spots: {:?}", search_spots);
    println!("search spots: {:?}", search_spots);
    println!("grid elements: {:?}", grid.len());

    let mut i = 1;
    let len = search_spots.len();
    for search_start in search_spots {
        println!("searching {:?} ({}/{:?})", search_start, i, len);

        let mut distances = HashMap::new();
        let mut visited = Vec::new();
        let mut to_visit = Vec::new();

        let (start_x, start_y) = search_start;

        // search_path(start_position, end_position, &grid, Vec::new(), &mut shortest_path);
        for ((x,y), height) in &grid {
            // println!("{:?},{:?}: {}", x, y, height);
            // let ((x,y), height) = node;
            let mut node_edges = Vec::new();
            if *x > 0 {
                if let Some(left) = grid.get(&(x-1, *y)) {
                    if *left <= height + 1 {
                        node_edges.push((x-1, *y));
                    }
                }
            }
            if *y > 0 {
                if let Some(top) = grid.get(&(*x, y-1)) {
                    if *top <= height + 1 {
                        node_edges.push((*x, y-1));
                    }
                }
            }
            if let Some(right) = grid.get(&(x+1, *y)) {
                if *right <= height + 1 {
                    node_edges.push((x+1, *y));
                }
            }
            if let Some(bottom) = grid.get(&(*x, y+1)) {
                if *bottom <= height + 1 {
                    node_edges.push((*x, y+1));
                }
            }
            if (x,y) == (&start_x, &start_y) {
                distances.insert((*x,*y), Some(0));
            } else {
                distances.insert((*x,*y), None);
            }
            edges.insert((x,y), node_edges);
        }

        to_visit.push((start_x, start_y));
        // visited.push(start_position);
        // let start_edges = edges.get(&(&start_x, &start_y)).unwrap();
        // for edge in start_edges {
            // to_visit.push(edge);
        // }

        while let Some(edge) = to_visit.pop() {
            // println!("checking node: {:?}", edge);
            visited.push(edge);
            let (x, y) = edge;

            let current_distance = *distances.get(&edge).unwrap();
            // println!("current distance: {:?}", current_distance);
            // println!("comparing edges: {:?}", edges.get(&(&x, &y)).unwrap());

            for node_edge in edges.get(&(&x, &y)).unwrap() {
                // let (node_x, node_y) = node_edge;
                // println!("checking edge {:?}", node_edge);
                let new_distance = current_distance.unwrap()+ 1;
                match distances.get(&node_edge).unwrap() {
                    None => {
                        // println!("no existing distance, inserting {}", new_distance);
                        distances.insert(*node_edge, Some(new_distance));
                        // if !visited.contains(node_edge) {
                            to_visit.push(*node_edge);
                        // }
                    }
                    Some(distance) => {
                        // println!("there is an existing distance {}", distance);
                        if &new_distance < distance {
                            // println!("new distance is less than existing distance {}", new_distance);
                            distances.insert(*node_edge, Some(new_distance));
                            to_visit.push(*node_edge);
                        }
                    }
                }
            }


        }

        // if let Some(shortest_path) = distances.get(&end_position).unwrap(){
        //     shortest_paths.push(Reverse(shortest_path));
        // }
        if let Some(steps) = distances.get(&end_position).unwrap() {
            match shortest_path {
                None => shortest_path = Some(*steps),
                Some(old_steps) => {
                    if steps < &old_steps {
                        shortest_path = Some(*steps);
                    }
                }
            }
        }

        i +=1;

    }


    // println!("start: {:?}", start_position);
    // println!("end: {:?}", end_position);
    // println!("grid: {:?}", grid);
    // println!("edges: {:?}", edges);
    // println!("distances: {:?}", distances);
    // println!("visited: {:?}", visited);
    // println!("to visit: {:?}", to_visit);

    // let shortest_path = distances.get(&end_position).unwrap();
    // println!("shortest path: {:?}", shortest_path);



    // // let Reverse(shortest_path) = paths.pop().unwrap();
    // shortest_path
    // *shortest_path
    // match shortest_paths.pop() {
    //     None => None,
    //     Some(v) => {
    //         let Reverse(shortest_path) = v;
    //         Some(v)
    //     }
    // }
    shortest_path
}

fn search_path(next_node: (usize, usize), end: (usize, usize), grid: &HashMap<(usize, usize), u32>, mut visited: Vec<(usize, usize)>, shortest_path: &mut u32) {
    // println!("checking node: {:?}", next_node);

    if next_node == end {
        // println!("found end, steps: {:?}", visited);
        *shortest_path = visited.len() as u32;
        // found.push(Reverse(steps));
        return;
    }

    if visited.contains(&next_node) {
        // println!("already visited this node: {:?}", visited);
        return;
    }

    if visited.len() as u32 == *shortest_path {
        // println!("this path is already taking too long!");
        return;
    }

    let (x, y) = next_node;
    let current_height = grid.get(&next_node).unwrap();
    // println!("({}, {}) current height: {}", x, y, current_height);
    visited.push((x, y));

    if y != 0 {
        if visited.contains(&(x, y-1)) {
            // println!("we already visited top node!");
        } else {
            if let Some(up_node) = grid.get(&(x, y -1)) {
                if *up_node <= current_height + 1 {
                    // println!("({}, {}): top node ({}, {}) height is ok: {}", x, y, x, y-1, *up_node);
                    let mut new_visited = Vec::new();
                    for v in visited.clone() {
                        new_visited.push(v);
                    }
                    // println!("new visited: {:?}", new_visited);
                    search_path((x, y -1), end, &grid, new_visited, shortest_path);
                } else {
                    // println!("({}, {}): top node ({}, {}) is too tall: {}", x, y, x, y-1, *up_node);
                }
            } else {
                // println!("reached the top of the grid");
            }
        }
    } else {
        // println!("reached the top of the grid");
    }

    if x != 0 {
        if visited.contains(&(x-1, y)) {
            // println!("we already visited left node!");
        } else {
            if let Some(left_node) = grid.get(&(x -1, y)) {
                if *left_node <= current_height + 1 {
                    // println!("({}, {}): left node ({}, {}) height is ok: {}", x, y, x-1, y, *left_node);
                    let mut new_visited = Vec::new();
                    for v in visited.clone() {
                        new_visited.push(v);
                    }
                    // println!("new visited: {:?}", new_visited);
                    search_path((x-1, y), end, &grid, new_visited, shortest_path);
                } else {
                    // println!("({}, {}): left node ({}, {}) is too tall: {}", x, y, x-1, y, *left_node);
                }
            } else {
                // println!("reached the left of the grid");
            }
        }
    } else {
        // println!("reached the left of the grid");
    }

    if visited.contains(&(x, y+1)) {
        // println!("we already visited bottom node");
    } else {
        if let Some(bottom_node) = grid.get(&(x, y+1)) {
            if *bottom_node <= current_height + 1 {
                // println!("({}, {}): bottom node ({}, {}), node height is ok: {}", x, y, x, y+1, *bottom_node);
                let mut new_visited = Vec::new();
                for v in visited.clone() {
                    new_visited.push(v);
                }
                // println!("new visited: {:?}", new_visited);
                search_path((x, y+1), end, &grid, new_visited, shortest_path);
            } else {
                // println!("({}, {}): bottom node ({}, {}), is too tall: {}", x, y, x, y+1, *bottom_node);
            }
        } else {
            // println!("reached the bottom of the grid!");
        }
    }

    if visited.contains(&(x+1, y)) {
        // println!("we already visited right node");
    } else {
        if let Some(right_node) = grid.get(&(x+1, y)) {
            if *right_node <= current_height + 1 {
                // println!("({}, {}): right node ({}, {}) height is ok: {}", x, y, x+1, y, *right_node);
                let mut new_visited = Vec::new();
                for v in visited.clone() {
                    new_visited.push(v);
                }
                // println!("new visited: {:?}", new_visited);
                search_path((x+1, y), end, &grid, new_visited, shortest_path);
            } else {
                // println!("({}, {}): right node ({}, {}) is too tall: {}", x, y, x+1, y, *right_node);
            }
        } else {
            // println!("reached the right of the grid");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        // let input = "Saa\nddb\nffE";
        // abcdef ghijkl mnopqr stuvwx yEzzzz zzzzzz
        let mut input = "Sbcdef\nlkjihg\nmnopqr\nxwvuts\nyEzzzz\nzzzzzz";
        assert_eq!(y22d12(input, 1), Some(25));
        let input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi\n";
        assert_eq!(y22d12(input, 1), Some(31));
        assert_eq!(y22d12(input, 2), Some(29));
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2022/day12.txt").unwrap();

        assert_eq!(y22d12(&contents, 1).unwrap(), 370);
        assert_eq!(y22d12(&contents, 1).unwrap(), 363);
    }
}
