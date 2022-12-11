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

//! Utility functions for Advent of Code.

use std::ops::{Div, Mul, Rem};

/// Compute the greatest common divisor.
///
/// The greatest common divisor is defined as largest positive integer (among
/// a set of integers that are not all zero) that divides each of the integers.
///
/// Computes the greatest common divisor using the [Euclidean
/// algorithm](https://en.wikipedia.org/wiki/Greatest_common_divisor#Euclidean_algorithm).
///
/// # Example
/// ```rust
/// # use aoc::util;
/// assert_eq!(util::gcd(48, 18), 6);
/// assert_eq!(util::gcd(18, 48), 6);
/// ```
pub fn gcd<T>(a: T, b: T) -> T
where
    T: PartialEq + Rem<Output = T> + From<u8> + Copy,
{
    if b == 0u8.into() {
        return a;
    } else {
        return gcd(b, a % b);
    }
}

/// Compure the least common multiple.
///
/// The least common multiple is defined as the smallest possible integer that
/// is divisible by both of two other integers.
///
/// Computes the least common multiple using the [greatest common
/// divisor](https://en.wikipedia.org/wiki/Least_common_multiple#Using_the_greatest_common_divisor).
///
/// # Example
/// ```rust
/// # use aoc::util;
/// assert_eq!(util::lcm(21, 6), 42);
/// ```
pub fn lcm<T>(a: T, b: T) -> T
where
    T: PartialEq
        + Div<Output = T>
        + Mul<Output = T>
        + Rem<Output = T>
        + From<u8>
        + Copy,
{
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(1, 0), 1);
        assert_eq!(gcd(0, 1), 1);
        assert_eq!(gcd(7, 1), 1);
        assert_eq!(gcd(120, 84), 12);
        assert_eq!(gcd(84, 120), 12);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(1, 0), 0);
        assert_eq!(lcm(1, 1), 1);
        assert_eq!(lcm(9, 12), 36);
        assert_eq!(lcm(7, 1), 7);
    }
}
