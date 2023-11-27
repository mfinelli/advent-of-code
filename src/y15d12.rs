/* Copyright 2023 Mario Finelli
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

//! Advent of Code 2015 Day 12: <https://adventofcode.com/2015/day/12>
//!
//! A straigtforward puzzle made easy by Rust's match syntax to allow us to
//! easily recurse based on the possible JSON types. We make use of
//! [serde](https://github.com/serde-rs/json) to handle the actual parsing of
//! the input string.

use serde_json::Value;

/// The solution for the day twelve challenge.
///
/// Given the input string and a boolean whether or not we should ignore
/// objects that have a value `red` we pass everything off to the calculate
/// value function which can call itself recursively to calculate the total
/// sum.
///
/// # Example
/// ```rust
/// # use aoc::y15d12::y15d12;
/// // probably read this from the input file...
/// let input = "[1,{\"a\":\"red\",\"b\":2}]";
/// assert_eq!(y15d12(input, false), 3);
/// assert_eq!(y15d12(input, true), 1);
/// ```
pub fn y15d12(input: &str, ignore_red: bool) -> i64 {
    let json: Value = serde_json::from_str(input).unwrap();
    calculate_value(json, ignore_red)
}

/// This function is what is responsible for actually calculating the value of
/// all of the numbers of a JSON. We first check to see if we're ignoring red
/// and if the object has red in it. If it does then we're done, otherwise we
/// continue processing. We're only interested in three JSON types: arrays in
/// which re recurse to each value of the array adding it to the sum, objects
/// in which we recurse to each value of the object adding it to the sum, and
/// numbers which we add directly to the sum. Everything else can be ignored.
fn calculate_value(value: Value, ignore_red: bool) -> i64 {
    let mut sum = 0;

    if ignore_red && has_red(value.clone()) {
        return 0;
    }

    match value {
        Value::Array(a) => {
            for i in a {
                sum += calculate_value(i, ignore_red);
            }
        }
        Value::Object(o) => {
            for (_, i) in o.iter() {
                sum += calculate_value(i.clone(), ignore_red);
            }
        }
        Value::Number(n) => {
            sum += n.as_i64().unwrap();
        }
        _ => {}
    }

    sum
}

/// This function determines if the provided JSON value is an object that has
/// `red` for any of its values.
fn has_red(value: Value) -> bool {
    match value {
        Value::Object(o) => {
            for (_, i) in o.iter() {
                if let Value::String(s) = i {
                    if s == "red" {
                        return true;
                    }
                }
            }

            false
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_calculate_value() {
        let mut input = "{\"a\":[]}";
        let mut json: Value = serde_json::from_str(input).unwrap();
        assert_eq!(calculate_value(json, false), 0);

        input = "[0,1,2]";
        json = serde_json::from_str(input).unwrap();
        assert_eq!(calculate_value(json, false), 3);
    }

    #[test]
    fn test_has_red() {
        let mut input = "{\"a\":\"red\"}";
        let mut json: Value = serde_json::from_str(input).unwrap();
        assert!(has_red(json));

        input = "{\"a\":\"green\"}";
        json = serde_json::from_str(input).unwrap();
        assert!(!has_red(json));

        input = "{\"red\":\"\"}";
        json = serde_json::from_str(input).unwrap();
        assert!(!has_red(json));
    }

    #[test]
    fn it_works() {
        let mut input = "[1,2,3]";
        assert_eq!(y15d12(input, false), 6);
        assert_eq!(y15d12(input, true), 6);

        input = "{\"a\":2,\"b\":4}\n";
        assert_eq!(y15d12(input, false), 6);
        assert_eq!(y15d12(input, true), 6);

        input = "[[[3]]]\n";
        assert_eq!(y15d12(input, false), 3);
        assert_eq!(y15d12(input, true), 3);

        input = "{\"a\":{\"b\":4},\"c\":-1}";
        assert_eq!(y15d12(input, false), 3);
        assert_eq!(y15d12(input, true), 3);

        input = "{\"a\":[-1,1]}\n";
        assert_eq!(y15d12(input, false), 0);
        assert_eq!(y15d12(input, true), 0);

        input = "[-1,{\"a\":1}]";
        assert_eq!(y15d12(input, false), 0);
        assert_eq!(y15d12(input, true), 0);

        input = "[]\n";
        assert_eq!(y15d12(input, false), 0);
        assert_eq!(y15d12(input, true), 0);

        input = "{}";
        assert_eq!(y15d12(input, false), 0);
        assert_eq!(y15d12(input, true), 0);

        input = "[1,{\"c\":\"red\",\"b\":2},3]\n";
        assert_eq!(y15d12(input, false), 6);
        assert_eq!(y15d12(input, true), 4);

        input = "{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}";
        assert_eq!(y15d12(input, false), 15);
        assert_eq!(y15d12(input, true), 0);

        input = "[1,\"red\",5]\n";
        assert_eq!(y15d12(input, false), 6);
        assert_eq!(y15d12(input, true), 6);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day12.txt").unwrap();

        assert_eq!(y15d12(&contents, false), 119433);
        assert_eq!(y15d12(&contents, true), 68466);
    }
}
