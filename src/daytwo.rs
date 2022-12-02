use std::collections::HashMap;

pub fn daytwo(input: &str) -> u32 {
    let points = HashMap::from([
        ("X", 1), // rock
        ("Y", 2), // paper
        ("Z", 3), // scissors
    ]);

    let matchups = HashMap::from([
        ("A X", 4), // rock v. rock (1) / draw (3)
        ("A Y", 8), // rock v. paper (2) / win (6)
        ("A Z", 3), // rock v. scissors (3) / lose (0)
        ("B X", 1), // paper v. rock (1) / lose (0)
        ("B Y", 5), // paper v. paper (2) / draw (3)
        ("B Z", 9), // paper v. scissors (3) / win (6)
        ("C X", 7), // scissors v. rock (1) / win (6)
        ("C Y", 2), // scissors v. paper (2) / lose (0)
        ("C Z", 6), // scissors v. scissors (3) / draw (3)
    ]);

    let lines: Vec<_> = input.lines().collect();
    let mut total = 0;

    for line in lines {
        total += matchups.get(line).unwrap();
    }

    total
}

pub fn daytwo_parttwo(input: &str) -> u32 {
    let matchups = HashMap::from([
        ("A X", 3), // rock v. lose (scissors): 0 + 3 = 3
        ("A Y", 4), // rock v. draw (rock): 3 + 1 = 4
        ("A Z", 8), // rock v. win (paper): 6 + 2 = 8
        ("B X", 1), // paper v. lose (rock): 0 + 1 = 1
        ("B Y", 5), // paper v. draw (paper): 3 + 2 = 5
        ("B Z", 9), // paper v. win (scissors): 6 + 3 = 9
        ("C X", 2), // scissors v. lose (paper): 0 + 2 = 2
        ("C Y", 6), // scissors v. draw (scissors): 3 + 3 = 6
        ("C Z", 7), // scissors v. win (rock): 6 + 1 = 7
    ]);

    let lines: Vec<_> = input.lines().collect();
    let mut total = 0;

    for line in lines {
        total += matchups.get(line).unwrap();
    }

    total
}
