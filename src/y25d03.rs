/* Copyright 2025 Mario Finelli
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

//! Advent of Code 2025 Day 3: <https://adventofcode.com/2025/day/3>
//!
//! TODO

/// The solution for the day three challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y25d03::y25d03;
/// // probably read this from the input file...
/// let input = concat!(
/// );
/// assert_eq!(y25d03(input), 1);
/// ```
pub fn y25d03(input: &str, size: usize) -> u64 {
    let mut sum = 0;

    let lines: Vec<_> = input.lines().collect();
    for line in lines {
        let chars: Vec<_> = line.chars().collect();

        // println!("checking: {}", line);

        let mut index = 0;
        for digit in 0..size {
            let mut largest: u64 = chars[index].to_string().parse().unwrap();

            // println!("for digit {} current largest is {} index {}", size-digit, largest, index);

            for i in index+1..chars.len()-(size-digit)+1{
                let v: u64 = chars[i].to_string().parse().unwrap();
                // println!("checking {}", v);
                if v > largest {
                    // println!("for digit {} current largest is {} index {}", size-digit, largest, index);
                    largest = v;
                    index = i;
                }
            }

            // println!("definitive largest for digit {} is {} in index {}", size-digit, largest, index);

            // let pow = 10 ^ u32::try_from(size-1-digit).unwrap();
            let base: u64 = 10;
            let pow: u64 = base.pow(u32::try_from(size-1-digit).unwrap());
            // println!("adding {} to sum ({} * {})", largest * pow, pow, largest);
            sum += largest * pow;

            if digit != size-1 {
                index += 1;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = concat!(
            "987654321111111\n",
            "811111111111119\n",
            "234234234234278\n",
            "818181911112111\n",
        );
        assert_eq!(y25d03(input, 2), 357);
        assert_eq!(y25d03(input, 12), 3121910778619);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2025/day03.txt").unwrap();

        assert_eq!(y25d03(&contents, 2), 17432);
        assert_eq!(y25d03(&contents, 12), 173065202451341);
    }
}
