use std::collections::HashSet;

pub fn y22d06(input: &str) -> u32 {
    let size = 4;
    let chars: Vec<_> = input.trim().chars().collect();
    for (i, window) in chars.windows(size).enumerate() {
        let mut set = HashSet::new();
        for c in window {
            set.insert(c);
        }
        if set.len() == size {
            return (i + size) as u32;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(y22d06(input), 5);

        input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(y22d06(input), 6);

        input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(y22d06(input), 10);

        input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(y22d06(input), 11);
    }
}
