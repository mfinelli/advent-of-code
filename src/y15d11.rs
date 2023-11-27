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

//! Advent of Code 2015 Day 11: <https://adventofcode.com/2015/day/11>
//!
//! TODO

/// The solution for the day eleven challenge.
///
/// # Example
/// ```rust
/// # use aoc::y15d11::y15d11;
/// // probably read this from the input file...
/// let input = "zzzzzzzz";
/// assert_eq!(y15d11(input, 1), "aaaaaabcc");
/// ```
pub fn y15d11(input: &str, howmany: u32) -> String {
    let mut password: Vec<u8> = input.trim().chars().map(|c| c as u8).collect();

    for _ in 0..howmany {
        password = next_password(password);
        while !valid_password(password.clone()) {
            password = next_password(password);
        }
    }

    // password
    // "".to_string()
    String::from_utf8(password).unwrap()
}

/// TODO
fn next_password(current: Vec<u8>) -> Vec<u8> {
    // let mut password: Vec<u8> = current.trim().chars().rev().map(|c| c as u8).collect();
    let mut password = current.into_iter().rev().collect();
    password = increment_char(password, 0);
    // let converted = password.iter().rev().map(|c| *c as char);
    // converted.map(|c| c.to_string())
    let rev: Vec<u8> = password.iter().rev().map(|c| *c).collect();
    // String::from_utf8(rev).unwrap()
    rev
}

/// TODO
fn increment_char(mut p: Vec<u8>, index: u8) -> Vec<u8> {
    if index >= p.len().try_into().unwrap() {
        p.push(97);
        p
    } else {
        p[index as usize] += 1;

        if p[index as usize] > 122 {
            p[index as usize] = 97;
            p = increment_char(p, index + 1);
        }

        p
    }
}

/// TODO
fn valid_password(password: Vec<u8>) -> bool {
    if password.contains(&105)
        || password.contains(&108)
        || password.contains(&111)
    {
        return false;
    }

    let mut found_run = false;
    for w in password.windows(3) {
        if w[1] - 1 == w[0] && w[2] - 2 == w[0] {
            found_run = true;
            break;
        }
    }

    if !found_run {
        return false;
    }

    let mut found_double: Option<u8> = None;
    for w in password.windows(2) {
        if w[0] == w[1] {
            found_double = Some(w[0]);
            break;
        }
    }

    match found_double {
        None => return false,
        Some(double) => {
            for w in password.windows(2) {
                if w[0] == w[1] && w[0] != double {
                    return true;
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_next_password() {
        let mut expected: Vec<u8> = "xy".into();
        assert_eq!(next_password("xx".into()), expected);

        expected = "xz".into();
        assert_eq!(next_password("xy".into()), expected);

        expected = "ya".into();
        assert_eq!(next_password("xz".into()), expected);

        expected = "yb".into();
        assert_eq!(next_password("ya".into()), expected);
    }

    #[test]
    fn test_valid_password() {
        assert_eq!(valid_password("hijklmmn".into()), false);
        assert_eq!(valid_password("abbceffg".into()), false);
        assert_eq!(valid_password("abbcegjk".into()), false);
        assert_eq!(valid_password("abcdffaa".into()), true);
        assert_eq!(valid_password("ghjaabcc".into()), true);
    }

    #[test]
    fn test_increment_char() {
        assert_eq!(increment_char(vec!(97), 0), vec!(98)); // a -> b
        assert_eq!(increment_char(vec!(122), 0), vec!(97, 97)); // z -> aa
        assert_eq!(increment_char(vec!(122, 97), 0), vec!(97, 98)); // za -> ab
    }

    #[test]
    fn it_works() {
        let mut input = "abcdefgh\n";
        assert_eq!(y15d11(input, 1), "abcdffaa");

        input = "ghijklmn";
        assert_eq!(y15d11(input, 1), "ghjaabcc");
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day11.txt").unwrap();

        assert_eq!(y15d11(&contents, 1), "hxbxxyzz");
        assert_eq!(y15d11(&contents, 2), "hxcaabcc");
    }
}
