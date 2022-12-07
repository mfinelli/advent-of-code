use std::collections::HashMap;
use std::path::PathBuf;

pub fn y22d07(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let mut total = 0;

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
        if line.starts_with("$ cd ") {
            let cmd: Vec<&str> = line.split_whitespace().collect();
            let dir = cmd[2];

            if dir == ".." {
                current_path.pop();
            } else {
                current_path.push(dir);
            }
        } else if line == &"$ ls" {
            if visited.contains(&current_path.to_str().unwrap().to_string()) {
                panic!("We already visited this directory");
            } else {
                visited.push(current_path.to_str().unwrap().to_string());
            }
        } else {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let name = parts[1];

            if parts[0] == "dir" {
                let fulldir =
                    current_path.join(name).to_str().unwrap().to_string();
                let status = sizes.insert(fulldir, 0);
                if !status.is_none() {
                    panic!("Tried to insert a directory that already exists");
                }
            } else {
                let fsize: u32 = parts[0].parse().unwrap();
                println!("found file {} with size {}", name, fsize);

                sizes
                    .entry(current_path.to_str().unwrap().to_string())
                    .and_modify(|size| *size += fsize);
                let mut paths = current_path.clone();
                while paths.pop() {
                    println!("add {} to {}", fsize, paths.display());
                    sizes
                        .entry(paths.to_str().unwrap().to_string())
                        .and_modify(|size| *size += fsize);
                }
            }
        }
    }

    for (dir, size) in sizes {
        if size <= 100000 {
            total += size;
        }
    }

    total
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

        assert_eq!(y22d07(input), 95437);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2022/day07.txt").unwrap();

        assert_eq!(y22d07(&contents), 1582412);
    }
}
