use md5::{Md5, Digest};

pub fn y15d04(input: &str) -> Option<u64> {
    let bytes = input.trim().as_bytes();
    // println!("{}", hash.get(0..5).unwrap());
    for i in 1..u64::MAX {
        // let hash = format!("{:x}", Md5::digest(trimmed + &i.to_string()));
        let hash = format!("{:x}", Md5::new().chain_update(bytes).chain_update(i.to_string().as_bytes()).finalize());
        if hash.get(0..5).unwrap() == "00000" {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let mut input = "abcdef\n";
        assert_eq!(y15d04(input), Some(609043));

        input = "pqrstuv";
        assert_eq!(y15d04(input), Some(1048970));
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day04.txt").unwrap();

        assert_eq!(y15d04(&contents).unwrap(), 254575);
    }
}
