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

//! Advent of Code 2022 Day 7: <https://adventofcode.com/2022/day/7>
//!
//! I initially started this out by trying to make a more robust representation
//! of the "filesystem" using custom structs for directories and files but
//! after arguing for a little while with the borrow checker while trying to
//! keep track of the parent directory I realized that I was over-complicating
//! things. Instead I settled on a solution that just creates a
//! [`std::collections::HashMap`] of full directory paths and then as it sees
//! each file it adds the size to each of the directory components stored in
//! the hash all the way up to the root (`/`) entry.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::path::PathBuf;

/// The solution for the day seven challenge.
///
/// Given the usual parameters of the input as a string and which part `1` or
/// `2` we start by parsing the input to build a representation of the total
/// file size contained in each directory as described above.
///
/// We start by analyzing each line (skipping the first line which we assume
/// will be changing into the root directory which we've used to setup the
/// initial state i.e., set the current path to the root and initialize its
/// size to zero). If we have a change directory command then simply we update
/// the current path marker. If we have a list directory command then we
/// insert the path into a list of directories that we have already visited.
/// We do this to ensure that we don't double count any directories if we list
/// them more than once. The provided input doesn't actually do this but I
/// wasn't sure if that was the case or not when I started and wanted a guard.
/// In theory it's possible to remove it now, but it doesn't hurt to keep
/// around (just increases the memory requirements).
///
/// Otherwise, we can assume that we're reading the results from listing the
/// directory. Other directories start with the string `dir` which we can use
/// to then initialize those directories with size `0` in our sizes map. If
/// we don't have a directory then we must have a file so we add its size to
/// the current directory and then explode the directory and add its size to
/// every component back up to the root which gives us the desired cumulative
/// size for each directory part.
///
/// After computing the (cumulative) size of each directory we can respond to
/// the challenge prompt. In part one we just loop through all of the
/// directories and if their total size is greater than `1000000` (provided by
/// the prompt) then we add the size to the total. In part two we instead
/// compute the free space (total space minus the total size of the root
/// directory) and then loop through our directories. If deleting the directory
/// would give us enough free space (both total space and required space are
/// provided by the prompt) then we add the directory size to a min-heap.
/// Finally, we can simply pop the smallest directory that satisfies the
/// minimum space requirements.
///
/// # Example
/// ```rust
/// # use aoc::y22d07::y22d07;
/// // probably read this from the input file...
/// let input = concat![
///    "$ cd /\n",
///    "$ ls\n",
///    "39950000 a\n",
///    "dir b\n",
///    "dir c\n",
///    "$ cd b\n",
///    "$ ls\n",
///    "50000 d\n",
///    "$ cd ..\n",
///    "$ cd c\n",
///    "$ ls\n",
///    "10000000 e",
/// ];
/// assert_eq!(y22d07(&input, 1), 50000);
/// assert_eq!(y22d07(&input, 2), 10000000);
/// ```
pub fn y22d07(input: &str, part: u32) -> u32 {
    let lines: Vec<_> = input.lines().collect();

    // we assume that the first line/command is to change into the root
    let mut current_path = PathBuf::new().join("/");
    let mut sizes = HashMap::new();
    let mut visited = Vec::new();
    sizes.insert(current_path.to_str().unwrap().to_string(), 0);

    // enforce our assumption above...
    if lines[0] != "$ cd /" || lines[1] != "$ ls" {
        panic!("Didn't get expected starting input!");
    }

    for line in lines.iter().skip(1) {
        let current_path_string = current_path.to_str().unwrap().to_string();

        if line.starts_with("$ cd ") {
            let cmd: Vec<&str> = line.split_whitespace().collect();
            let dir = cmd[2];

            if dir == ".." {
                current_path.pop();
            } else {
                current_path.push(dir);
            }
        } else if line == &"$ ls" {
            if visited.contains(&current_path_string) {
                panic!("We already visited this directory");
            } else {
                visited.push(current_path_string);
            }
        } else {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let name = parts[1];

            if parts[0] == "dir" {
                let fulldir =
                    current_path.join(name).to_str().unwrap().to_string();
                let status = sizes.insert(fulldir, 0);
                if status.is_some() {
                    panic!("Tried to insert a directory that already exists");
                }
            } else {
                let fsize: u32 = parts[0].parse().unwrap();

                sizes
                    .entry(current_path_string)
                    .and_modify(|size| *size += fsize);
                let mut paths = current_path.clone();
                while paths.pop() {
                    let path_str = paths.to_str().unwrap().to_string();
                    sizes.entry(path_str).and_modify(|size| *size += fsize);
                }
            }
        }
    }

    if part == 1 {
        let mut total = 0;

        for (_dir, size) in sizes {
            if size <= 100000 {
                total += size;
            }
        }

        total
    } else {
        let total_disk_space = 70000000;
        let required_disk_space = 30000000;
        let current_free_space = total_disk_space - sizes.get("/").unwrap();
        let mut heap = BinaryHeap::new();

        for (_dir, size) in sizes {
            if size + current_free_space >= required_disk_space {
                heap.push(Reverse(size))
            }
        }

        let Reverse(smallest) = heap.pop().unwrap();
        smallest
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = concat!(
            "$ cd /\n",
            "$ ls\n",
            "dir a\n",
            "14848514 b.txt\n",
            "8504156 c.dat\n",
            "dir d\n",
            "$ cd a\n",
            "$ ls\n",
            "dir e\n",
            "29116 f\n",
            "2557 g\n",
            "62596 h.lst\n",
            "$ cd e\n",
            "$ ls\n",
            "584 i\n",
            "$ cd ..\n",
            "$ cd ..\n",
            "$ cd d\n",
            "$ ls\n",
            "4060174 j\n",
            "8033020 d.log\n",
            "5626152 d.ext\n",
            "7214296 k\n",
        );

        assert_eq!(y22d07(input, 1), 95437);
        assert_eq!(y22d07(input, 2), 24933642);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2022/day07.txt").unwrap();

        assert_eq!(y22d07(&contents, 1), 1582412);
        assert_eq!(y22d07(&contents, 2), 3696336);
    }
}
