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

//! Advent of Code 2015 Day 4: <https://adventofcode.com/2015/day/4>
//!
//! This is a simple, if computationally expensive challenge. Start at `1`
//! and append to the input string and compute the `MD5` hash. If the (hex)
//! results has the desired number of leading `0`s then you're done,
//! otherwise increment the counter and try again.
//!
//! I didn't try to implement the `MD5` algorithm myself and instead decided
//! to use the [md-5](https://docs.rs/md-5/latest/md5/) crate.

use md5::{Digest, Md5};

/// The solution for the day four challenge.
///
/// The function takes the input string and the number of leading zeros to
/// find as arguments. The logic is otherwise very simple, combine the input
/// string and counter and compute the `MD5` has until you've found the
/// desired number of leading zeros (or `None` if we get to the maximum
/// integer size without finding a match).
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
