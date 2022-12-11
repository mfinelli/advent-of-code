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
use std::{sync::Arc, thread};

/// The solution for the day four challenge.
///
/// The function takes the input string and the number of leading zeros to
/// find as arguments. The logic is otherwise very simple, combine the input
/// string and counter and compute the `MD5` has until you've found the
/// desired number of leading zeros (or `None` if we get to the maximum
/// integer size without finding a match).
///
/// # Example
/// ```rust
/// # use aoc::y15d04::y15d04;
/// let input = "a"; // probably read this from the input file...
/// assert_eq!(y15d04(input.to_string(), 1), Some(27));
/// ```
pub fn y15d04(input: String, leading_zeros: u32) -> Option<u64> {
    let check = Arc::new("0".repeat(leading_zeros as usize));
    // let bytes = Arc::new(input.trim().as_bytes());
    let threads = thread::available_parallelism().unwrap().get();
    let chunks = 10000 / threads as u64;
    let actual_chunks = chunks * threads as u64;
    let mut results = Vec::new();

    let mut i = 1;
    while i < u64::MAX {
        let mut handles = Vec::new();
        for j in 0..threads as u64 {
            // let bytes = Arc::clone(&bytes);
            let input = input.clone();
            let check = Arc::clone(&check);
            let start = i + j * chunks;
            handles.push(thread::spawn(move || {
                return do_work(
                    input,
                    start,
                    start + chunks + 1,
                    leading_zeros as usize,
                    check,
                );
            }));
        }

        for handle in handles {
            let result = handle.join().unwrap();
            if result.is_some() {
                results.push(result.unwrap());
            }
        }

        if results.len() >= 1 {
            results.sort();
            return Some(results[0]);
        }

        i += actual_chunks;
    }

    None
}

fn do_work(
    input: String,
    start: u64,
    end: u64,
    leading_zeros: usize,
    check: Arc<String>,
) -> Option<u64> {
    for i in start..end {
        let hash = format!(
            "{:x}",
            Md5::new()
                .chain_update(input.trim().as_bytes())
                .chain_update(i.to_string().as_bytes())
                .finalize()
        );
        if hash.get(0..leading_zeros).unwrap() == *check {
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
        assert_eq!(y15d04(input.to_string(), 5), Some(609043));

        input = "pqrstuv";
        assert_eq!(y15d04(input.to_string(), 5), Some(1048970));
    }

    #[test]
    #[ignore]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day04.txt").unwrap();

        assert_eq!(y15d04(contents.clone(), 5).unwrap(), 254575);
        assert_eq!(y15d04(contents.clone(), 6).unwrap(), 1038736);
    }
}
