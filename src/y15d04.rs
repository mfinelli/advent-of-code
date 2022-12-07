use md5::{Digest, Md5};

pub fn y15d04(input: &str, leading_zeros: u32) -> Option<u64> {
    let check = "0".repeat(leading_zeros as usize);
    let bytes = input.trim().as_bytes();
    for i in 1..u64::MAX {
        let hash = format!(
            "{:x}",
            Md5::new()
                .chain_update(bytes)
                .chain_update(i.to_string().as_bytes())
                .finalize()
        );
        if hash.get(0..leading_zeros as usize).unwrap() == check {
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
    #[ignore]
    fn it_works() {
        let mut input = "abcdef\n";
        assert_eq!(y15d04(input, 5), Some(609043));

        input = "pqrstuv";
        assert_eq!(y15d04(input, 5), Some(1048970));
    }

    #[test]
    #[ignore]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day04.txt").unwrap();

        assert_eq!(y15d04(&contents, 5).unwrap(), 254575);
        assert_eq!(y15d04(&contents, 6).unwrap(), 1038736);
    }
}
