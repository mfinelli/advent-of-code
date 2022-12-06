use std::collections::HashSet;

pub fn y22d06(input: &str, size: u32) -> u32 {
    // let size = 4;
    let chars: Vec<_> = input.trim().chars().collect();
    for (i, window) in chars.windows(size as usize).enumerate() {
        let mut set = HashSet::new();
        for c in window {
            set.insert(c);
        }
        if set.len() == size as usize {
            return (i + size as usize) as u32;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let mut input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(y22d06(input, 4), 5);
        assert_eq!(y22d06(input, 14), 23);

        input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(y22d06(input, 4), 6);
        assert_eq!(y22d06(input, 14), 23);

        input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(y22d06(input, 4), 10);
        assert_eq!(y22d06(input, 14), 29);

        input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(y22d06(input, 4), 11);
        assert_eq!(y22d06(input, 14), 26);

        input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(y22d06(input, 14), 19);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2022/day06.txt").unwrap();

        assert_eq!(y22d06(&contents, 4), 1802);
        assert_eq!(y22d06(&contents, 14), 3551);
    }
}
