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

//! Advent of Code 2025 Day 5: <https://adventofcode.com/2025/day/5>
//!
//! TODO

/// The solution for part one of the day five challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y25d05::y25d05;
/// // probably read this from the input file...
/// let input = concat!(
/// );
/// assert_eq!(y25d05p1(input), 1);
/// ```
pub fn y25d05p1(input: &str) -> u64 {
    let mut sum = 0;
    let parts: Vec<_> = input.split("\n\n").collect();
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for range in parts[0].lines().into_iter() {
        let range_parts: Vec<_> = range.split('-').collect();
        let lower: u64 = range_parts[0].parse().unwrap();
        let upper: u64 = range_parts[1].parse().unwrap();
        ranges.push((lower, upper));
    }

    for item in parts[1].lines().into_iter() {
        let value: u64 = item.parse().unwrap();

        for (lower, upper) in &ranges {
            if value >= *lower && value <= *upper {
                sum += 1;
                break;
            }
        }
    }

    sum
}

/// The solution for part two of the day five challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y25d05::y25d05;
/// // probably read this from the input file...
/// let input = concat!(
/// );
/// assert_eq!(y25d05p2(input), 1);
/// ```
pub fn y25d05p2(input: &str) -> u64 {
    let parts: Vec<_> = input.split("\n\n").collect();
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut sum = 0;

    for range in parts[0].lines().into_iter() {
        let range_parts: Vec<_> = range.split('-').collect();
        let lower: u64 = range_parts[0].parse().unwrap();
        let upper: u64 = range_parts[1].parse().unwrap();
        ranges.push((lower, upper));
    }

    ranges.sort_by(|a, b| {
        let (a_lower, _) = a;
        let (b_lower, _) = b;
        a_lower.cmp(b_lower)
    });

    loop {
        let initial_sum = sum;
        let (final_ranges, new_sum) = combine_ranges(&mut ranges);
        sum = new_sum;

        // println!("initial sum was {} new sum is {}\n\n", initial_sum, new_sum);
        if sum == initial_sum {
            return sum;
        } else {
            ranges = final_ranges;
        }
    }

}

fn combine_ranges(initial_ranges: &mut Vec<(u64, u64)>) -> (Vec<(u64, u64)>, u64) {
    let mut final_ranges: Vec<(u64, u64)> = Vec::new();

    for range in initial_ranges {
        // println!("checking range {:?}", range);
        // println!("current final ranges {:?}", final_ranges);
        let mut addit: bool = true;

        for (lower, upper) in final_ranges.iter_mut() {
            let (l, u) = range;

            if l >= lower && u <= upper {
                // println!("range {:?} is completed contained by {}-{}", range, lower, upper);
                // range is completely contained by another... move on
                addit = false;
                break;
            }

            if l < lower && u >= lower && u <= upper {
                // println!("range {}-{} is partially contained (lower) by {}-{}", l, u, lower, upper);
                // range is partially contained on the lower bound...
                // extend the lower bound and move on
                *lower = *l;
                addit = false;
                break;
            }

            if l >= lower && l <= upper && u > upper {
                // println!("range {}-{} is partially contained (upper) by {}-{}", l, u, lower, upper);
                // range is partially contained on the upper bound...
                // extend the upper bound and move on
                *upper = *u;
                addit = false;
                break;
            }

            // the range didn't fit into any existing ranges... so add it to
            // the final list
            // addit = true (the default)
        }

        if addit {
            // println!("range {:?} is not contained by any existing range, adding it", range);
            let (l, u) = range;
            final_ranges.push((*l, *u));
        }
    }

    let mut sum = 0;

    for (lower, upper) in &final_ranges {
        sum +=  upper - lower + 1;
    }

    // println!("current sum: {}", sum);

    (final_ranges, sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_combine_ranges() {
        // TODO
    }

    #[test]
    fn it_works() {
        let input = concat!(
            "3-5\n",
            "10-14\n",
            "16-20\n",
            "12-18\n",
            "\n",
            "1\n",
            "5\n",
            "8\n",
            "11\n",
            "17\n",
            "32\n",
        );

        assert_eq!(y25d05p1(input), 3);
        assert_eq!(y25d05p2(input), 14);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2025/day05.txt").unwrap();

        assert_eq!(y25d05p1(&contents), 598);
        assert_eq!(y25d05p2(&contents), 360341832208407);
    }
}
