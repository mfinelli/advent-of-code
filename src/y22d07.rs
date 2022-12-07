use std::path::PathBuf;

#[derive(Debug)]
struct Directory {
    path: String,
    files: Vec<File>,
    directories: Vec<Directory>,
}

impl PartialEq for Directory {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for Directory {}

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for File {}

pub fn y22d07(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();

    // we assume that the first line/command is to change into the root
    let mut current_path = PathBuf::new().join("/");
    let mut root = Directory {
        path: current_path.to_str().unwrap().to_string(),
        files: Vec::new(),
        directories: Vec::new(),
    };
    let mut current_directory = &mut root;
    let mut parent = None;

    // enforce our assumption above...
    if lines[0] != "$ cd /"  || lines[1] != "$ ls" {
        panic!("Didn't get expected starting input!");
    }

    // let mut in_directory_listing = false;


    let mut tmp = 0;


    for line in lines.iter().skip(2) {
        println!("");
        println!("{}", line);
        println!("{:?}", current_directory);
        println!("");

        if line.starts_with("$ cd ") {
            // in_directory_listing = false;
            let cmd: Vec<&str> = line.split_whitespace().collect();
            let dir = cmd[2];
            // current_path.push(dir);

            if dir == ".." {
                println!("moving up a directory");
                current_path.pop();
                println!("changed to directory {}", current_path.display());
            } else {
            //     current_path = &format!("{}/{}", current_path, dir);
                current_path.push(dir);
                println!("changing to directory {}", current_path.display());
            }

            parent = Some(&current_directory);
            let current_path_str = current_path.to_str().unwrap().to_string();
            let found = current_directory.directories.iter_mut().find(|d| d.path == current_path_str).unwrap();
            current_directory = found;
        } else if line == &"$ ls" {
            println!("listing directory {}", current_path.display());
            // in_directory_listing = true;
        } else {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let name = parts[1];

            if parts[0] == "dir" {
                println!("found directory {}", name);
                // if current_directory.directories.
                //
                let fullname = current_path.join(name).to_str().unwrap().to_string();
                match current_directory.directories.iter().find(|d| d.path == fullname) {
                    Some(dir) => continue,
                    None => {
                        current_directory.directories.push(Directory{
                            path: fullname,
                            files: Vec::new(),
                            directories: Vec::new(),
                        });
                    }
                }
            } else {
                let fsize: u32 = parts[0].parse().unwrap();
                println!("found file {} with size {}", name, fsize);
            }
        }

        tmp += 1;
        if tmp >=50{
            break;
        }
    }

    println!("{:?}", root);

    0
}
