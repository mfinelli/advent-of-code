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
//! TODO

use serde_json::{Result, Value};

/// The solution for the day twelve challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y15d12::y15d12;
/// // probably read this from the input file...
/// let input = "[]";
/// assert_eq!(y15d12(input), 0);
/// ```
pub fn y15d12(input: &str) -> i64 {
    let json: Value = serde_json::from_str(input).unwrap();
    calculate_value(json)
}

/// TODO
fn calculate_value(value: Value) -> i64 {
    let mut sum = 0;

    match value {
        Value::Array(a) => {
            for i in a {
                sum += calculate_value(i);
            }
        }
        Value::Object(o) => {
            for (_, i) in o.iter() {
                sum += calculate_value(i.clone());
            }
        }
        Value::Number(n) => {
            sum += n.as_i64().unwrap();
        }
        _ => {}
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_calculate_value() {
        let mut input = "{\"a\":[]}";
        let mut json: Value = serde_json::from_str(input).unwrap();
        assert_eq!(calculate_value(json), 0);

        input = "[0,1,2]";
        json = serde_json::from_str(input).unwrap();
        assert_eq!(calculate_value(json), 3);
    }

    #[test]
    fn it_works() {
        let mut input = "[1,2,3]";
        assert_eq!(y15d12(input), 6);

        input = "{\"a\":2,\"b\":4}\n";
        assert_eq!(y15d12(input), 6);

        input = "[[[3]]]\n";
        assert_eq!(y15d12(input), 3);

        input = "{\"a\":{\"b\":4},\"c\":-1}";
        assert_eq!(y15d12(input), 3);

        input = "{\"a\":[-1,1]}\n";
        assert_eq!(y15d12(input), 0);

        input = "[-1,{\"a\":1}]";
        assert_eq!(y15d12(input), 0);

        input = "[]\n";
        assert_eq!(y15d12(input), 0);

        input = "{}";
        assert_eq!(y15d12(input), 0);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day12.txt").unwrap();

        assert_eq!(y15d12(&contents), 119433);
    }
}
