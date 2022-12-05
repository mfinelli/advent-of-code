pub fn y22d05(input: &str) -> String {
    let lines: Vec<_> = input.lines().collect();

    let mut state = parse_initial_state(&lines);
    let mut in_moves = false;
    let mut output = String::new();

    for line in lines {
        if in_moves {
            let text: Vec<&str> = line.split_whitespace().collect();
            let how_many_to_move = text[1].parse().unwrap();
            let from_index: u32 = text[3].parse().unwrap();
            let to_index: u32 = text[5].parse().unwrap();

            for _ in 0..how_many_to_move {
                let from: &mut Vec<String> = &mut state[(from_index-1) as usize];
                let to_move = from.pop().unwrap();
                let to: &mut Vec<String> = &mut state[(to_index-1) as usize];
                to.push(to_move);
            }
        } else if line.is_empty() {
            in_moves = true;
        }
    }

    for mut stack in state {
        output += &stack.pop().unwrap();
    }

    output
}

fn parse_initial_state(lines: &Vec<&str>) -> Vec<Vec<String>> {
    // let mut state_lines: Vec<&str> = Vec::Vec<&str>::new();
    let mut state_lines: Vec<&str> = Vec::new();
    let mut state: Vec<Vec<String>> = Vec::new();

    for line in lines {
        if line.is_empty() {
            // state_lines.remove(state_lines.len() - 1);
            let columns = state_lines.pop().unwrap();
            let number_of_columns: u32 = columns.split_whitespace().last().unwrap().parse().unwrap();

            for _ in 0..number_of_columns {
                state.push(Vec::new());
            }

            break;
        }

        state_lines.push(line);

    }

    for line in state_lines.iter().rev() {
        let number_of_columns = (line.len() + 1) / 4;
        let mut column = 0;
        for i in 0..number_of_columns {
            if line.chars().nth(i * 4).unwrap() == '[' {
                state[column].push(line.chars().nth(i * 4 + 1).unwrap().to_string());
            }
            column += 1;
        }
    }

    // println!("{:?}", state_lines);
    // println!("{:?}", state);

    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_initial_state() {
        let input = concat!(
            "[A]     [B]\n",
            "[C] [D] [E] [F]\n",
            "[G] [H] [I] [J] [K]\n",
            " 1   2   3   4   5\n",
            "\n",
            "we don't care about this...\n",
            "or this...\n",
        );

        assert_eq!(parse_initial_state(input.lines().collect()), vec![
                   vec!["G", "C", "A"],
                   vec!["H", "D"],
                   vec!["I", "E", "B"],
                   vec!["J", "F"],
                   vec!["K"],
        ]);
    }
}
