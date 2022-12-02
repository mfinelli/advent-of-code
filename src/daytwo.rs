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

// fn sumline(line: &str, points: &HashMap<&str, u32>) -> u32 {
//     let plays: Vec<&str> = line.split_whitespace().collect();

//     let mut sum = points.get(plays[1]).unwrap();
//     *sum
// }
