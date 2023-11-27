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
//! This was fairly simple to implement and wasn't too challenging but the
//! solution ended up being rather long and verbose. The solution doesn't use
//! strings or regular expressions but operations instead on vectors of code
//! points and so the magic numbers found within correspond to the several
//! characters that we need to know about: `a`: `97`, `z`: `122`, `i`: `105`,
//! `l`: `108`, and `o`: `111`.

/// The solution for the day eleven challenge.
///
/// We take the input as a string and a second parameter for how many valid
/// new passwords we want to generate (it just so happens that in part one it's
/// `1` and in part two it's `2`). We first convert the password into a vector
/// of integers corresponding to the character code points. Then for the number
/// of new passwords that we want we generate the next password and then until
/// we have a valid password we keep generating the next password. Then we
/// convert our vector back into a string and we're done.
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

    String::from_utf8(password).unwrap()
}

/// This function calculates the next password (it doesn't care if it generates
/// a valid password or not). To make things easier it reverses the order of
/// the password first and then calls the next character function (which is
/// responsible for actually incrementing the password correctly) and then
/// reverses it again to return it.
fn next_password(current: Vec<u8>) -> Vec<u8> {
    let mut password = current.into_iter().rev().collect();
    password = increment_char(password, 0);
    password.iter().rev().copied().collect()
}

/// This function is responsible for actually incrementing the password
/// characters correctly. If we have an index greater than the length of our
/// current password then we've overflowed and add a new character. Otherwise,
/// we increment the character of the provided index and if that increases
/// past "z" (code point 122) then we reset it to "a" (code point 97) and then
/// run the increment on the next index recursively until we have incremented
/// everything all the way.
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

/// This function determines if the provided password meets all of the
/// requirements laid out in the prompt: no "i", "l", or "o" characters (code
/// points 105, 108, and 111 respectively), at least one run of three
/// characters, and two distinct sets of double characters.
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

    // if we find a double character record it for later
    let mut found_double: Option<u8> = None;
    for w in password.windows(2) {
        if w[0] == w[1] {
            found_double = Some(w[0]);
            break;
        }
    }

    // if we didn't find a double then we're done, otherwise repeat the above
    // but also check to make sure that we find a double distinct from the one
    // that we already found
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
        assert!(!valid_password("hijklmmn".into()));
        assert!(!valid_password("abbceffg".into()));
        assert!(!valid_password("abbcegjk".into()));
        assert!(valid_password("abcdffaa".into()));
        assert!(valid_password("ghjaabcc".into()));
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
