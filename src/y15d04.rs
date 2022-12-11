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
//! A second attempt to improve the runtime of this solution maintains the
//! same general logic as above but splits the work into parallel threads.
//! On my machine this saw time improvements from about 10-11 seconds down to
//! 2.5 to 3.5 seconds to compute both prefixes independently. There are
//! probably other improvements that could be made to speed this up further
//! (such as running both prefixes together so that we don't have to recompute
//! so many hashes that we know aren't long enough, but I prefer to have each
//! part distinct). The way that I implemented it is to spawn threads and have
//! them work on 10,000 hashes at a time. This could (maybe) be improved by
//! instead having a solution found mutex and having each thread work
//! independently until it finds a solution or until the solution mutex is
//! set or something similar. Obviously, the way I have currently done it,
//! there is a decent amount of wasted compute if the resulting number is very
//! small (e.g., answers less than 10,000 x number of threads) or even if we
//! could find an answer early in any batch of work we could return instantly
//! but instead need to wait for all of the threads to finish.
//!
//! I didn't try to implement the `MD5` algorithm myself and instead decided
//! to use the [md-5](https://docs.rs/md-5/latest/md5/) crate.

use md5::{Digest, Md5};
use std::{sync::Arc, thread};

/// The solution for the day four challenge.
///
/// This function takes the input string and the number of leading zeros to
/// find as arguments. The logic is otherwise fairly simple, as described
/// above we batch the work into threads. Each thread works on 10,000 numbers
/// combining the input string and each number one after the other to compute
/// the `MD5` hash until it either finds a hash that has the correct number of
/// leading zeros or it exhausts its numbers. After all threads have finished
/// we check to see if any results were found and if so we add them to a
/// vector, sort it, and return the smallest value (this is important because
/// in theory two threads could find a number that results in a has with the
/// correct number of leading zeros and we want the smallest of those numbers).
/// If no answer was found we spawn another batch of the threads and continue
/// in this way until we either find a match or reach the maximum integer size
/// (in which case we will return `None`).
///
/// # Example
/// ```rust
/// # use aoc::y15d04::y15d04;
/// let input = "a"; // probably read this from the input file...
/// assert_eq!(y15d04(input.to_string(), 1), Some(27));
/// ```
pub fn y15d04(input: String, leading_zeros: u32) -> Option<u64> {
    let check = Arc::new("0".repeat(leading_zeros as usize));
    let input = input.trim().to_string();
    let threads = thread::available_parallelism().unwrap().get();
    let chunks = 10000 / threads as u64;
    let actual_chunks = chunks * threads as u64;
    let mut results = Vec::new();

    let mut i = 1;
    while i < u64::MAX {
        let mut handles = Vec::new();
        for j in 0..threads as u64 {
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
                .chain_update(input.as_bytes())
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
