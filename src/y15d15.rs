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

//! Advent of Code 2015 Day 15: <https://adventofcode.com/2015/day/15>
//!
//! Today's problem was a little challenging. Looking for inspiration for how
//! to solve it I looked at some of the solutions on the advent of code
//! subreddit but found that almost all of the solutions brute-forced/hardcoded
//! the number of ingredients to four. This was unacceptable to me because I
//! wanted a generic solution that would work for any number of ingredients
//! (also because I wanted to be able to use the same functions to test my
//! solution against the examples which only had two ingredients). I originally
//! brute-forced it in my own way by generating a huge vector with all of the
//! possible ingredients and the possible teaspoons from 0-100 and then
//! generated all of the combinations (take length of ingredients) filtering
//! out any combination that didn't add up to the correct number of teaspoons
//! and/or didn't use all four (or length of ingredients) of the ingredients.
//! This worked and provided the correct answer, but it was painfully slow. On
//! my machine each part took about five minutes. To improve this I knew that
//! I needed to reduce the overall number of possible ingredient combinations.
//! I did this by instead computing the cartesian product of a vector for each
//! ingredient in all of its possible quantities. This greatly reduced the
//! overall number of combinations that needed to be checked/excluded. While
//! it still doesn't run extremely fast each part now runs in about 30 seconds
//! which for a generic solution is acceptable to me.

use itertools::Itertools;
use std::collections::BinaryHeap;

/// The total number of teaspoons of ingredients that we must use to create
/// cookies as provided by the prompt.
const TOTAL: i32 = 100;

/// The number of calories that a cookie must have to be a valid recipe as
/// provided by the prompt.
const CALORIES: i32 = 500;

/// Ingredient represents an ingredient parsed from the input.
#[derive(Debug)]
struct Ingredient {
    // name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

/// Mix represents an `Ingredient` and a quantity of that ingredient.
#[derive(Clone, Debug)]
struct Mix<'a> {
    ingredient: &'a Ingredient,
    amount: i32,
}

/// The solution for the day fifteen challenge.
///
/// As usual we take the input as a string and a variable to determine which
/// part we're computing (in part `2` we need to limit the options to cookies
/// that result in 500 calories). We start by processing the input and building
/// `Ingredients` to keep track of the various qualities of the ingredients.
/// We then create a vector of vectors, one for each ingredient where the
/// sub-vector are all of the possible teaspoons of the ingredient to use to
/// make the cookie (0 to 100). We then take the cartesian product of those
/// vectors to get all of the possible recipes (both valid and invalid) and
/// then filter the result to only the valid options (i.e., that use 100 total
/// teaspoons, and in part `2` are 500 calories). With the list of valid
/// recipes we compute the score of each one and add it to our
/// [`std::collections::BinaryHeap`]. To get the highest possible score we just
/// pop the heap.
///
/// # Example
/// ```rust
/// # use aoc::y15d15::y15d15;
/// // probably read this from the input file...
/// let input = concat!(
///     "Candy: capacity -1, durability 3, flavor 1, texture 1, calories 2\n",
///     "Chocolate: capacity 2, durability 2, flavor 1, texture 1, calories 8",
/// );
/// assert_eq!(y15d15(input, 1), 400000000);
/// assert_eq!(y15d15(input, 2), 125000000);
/// ```
pub fn y15d15(input: &str, part: u32) -> i32 {
    let lines: Vec<_> = input.lines().collect();
    let mut ingredients = Vec::new();
    let mut scores = BinaryHeap::new();

    for line in lines {
        let text: Vec<&str> = line.split_whitespace().collect();
        // let name = text[0].strip_suffix(':').unwrap().to_string();
        let capacity = text[2].strip_suffix(',').unwrap().parse().unwrap();
        let durability = text[4].strip_suffix(',').unwrap().parse().unwrap();
        let flavor = text[6].strip_suffix(',').unwrap().parse().unwrap();
        let texture = text[8].strip_suffix(',').unwrap().parse().unwrap();
        let calories = text[10].parse().unwrap();

        ingredients.push(Ingredient {
            // name,
            capacity,
            durability,
            flavor,
            texture,
            calories,
        });
    }

    let mut mixes = Vec::new();
    for ingredient in &ingredients {
        let mut totals = Vec::new();
        for amount in 0..TOTAL + 1 {
            totals.push(Mix { amount, ingredient });
        }
        mixes.push(totals);
    }

    let possibilities: Vec<Vec<Mix>> = mixes
        .into_iter()
        .multi_cartesian_product()
        .filter(|p| valid_possibility(p, part))
        .collect();

    for possibility in possibilities {
        scores.push(compute_score(&possibility));
    }

    scores.pop().unwrap()
}

/// This function determines whether the provided vector of ingredients and
/// their quantities are valid: the quantities should add up to 100 and if
/// we're in part `2` then the sum of the calories should add up to 500.
fn valid_possibility(possibility: &[Mix], part: u32) -> bool {
    let sum: i32 = possibility.iter().map(|p| p.amount).sum();
    if sum != TOTAL {
        return false;
    }

    if part == 2 {
        let calories: i32 = possibility
            .iter()
            .map(|p| p.ingredient.calories * p.amount)
            .sum();
        if calories != CALORIES {
            return false;
        }
    }

    true
}

/// This function computes the score of a combination of ingredients and their
/// quantities. For each of the qualities of an ingredient it's multiplied by
/// the quantity and then summed with the result of that quality for the other
/// ingredients in the combination. If any quality results in a negative
/// number then it turns to zero and the entire score is zero (as described by
/// the prompt).
fn compute_score(possibility: &[Mix]) -> i32 {
    let capacity: i32 = possibility
        .iter()
        .map(|p| p.ingredient.capacity * p.amount)
        .sum();
    if capacity <= 0 {
        return 0;
    }

    let durability: i32 = possibility
        .iter()
        .map(|p| p.ingredient.durability * p.amount)
        .sum();
    if durability <= 0 {
        return 0;
    }

    let flavor: i32 = possibility
        .iter()
        .map(|p| p.ingredient.flavor * p.amount)
        .sum();
    if flavor <= 0 {
        return 0;
    }

    let texture: i32 = possibility
        .iter()
        .map(|p| p.ingredient.texture * p.amount)
        .sum();
    if texture <= 0 {
        return 0;
    }

    capacity * durability * flavor * texture
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_compute_score() {
        let mut input = vec![Mix {
            ingredient: &Ingredient {
                capacity: 3,
                durability: 4,
                flavor: 5,
                texture: 6,
                calories: 7,
            },
            amount: 2,
        }];

        assert_eq!(compute_score(&input), 5760);

        input = vec![Mix {
            ingredient: &Ingredient {
                capacity: 2,
                durability: 4,
                flavor: 2,
                texture: -6,
                calories: 7,
            },
            amount: 3,
        }];

        assert_eq!(compute_score(&input), 0);
    }

    #[test]
    fn test_valid_possibility() {
        let mut input = vec![
            Mix {
                ingredient: &Ingredient {
                    capacity: 1,
                    durability: 1,
                    flavor: 1,
                    texture: 1,
                    calories: 1,
                },
                amount: 1,
            },
            Mix {
                ingredient: &Ingredient {
                    capacity: 1,
                    durability: 1,
                    flavor: 1,
                    texture: 1,
                    calories: 1,
                },
                amount: 1,
            },
        ];

        assert!(!valid_possibility(&input, 1));

        input = vec![
            Mix {
                ingredient: &Ingredient {
                    capacity: 1,
                    durability: 1,
                    flavor: 1,
                    texture: 1,
                    calories: 1,
                },
                amount: 50,
            },
            Mix {
                ingredient: &Ingredient {
                    capacity: 1,
                    durability: 1,
                    flavor: 1,
                    texture: 1,
                    calories: 1,
                },
                amount: 50,
            },
        ];

        assert!(valid_possibility(&input, 1));

        input = vec![
            Mix {
                ingredient: &Ingredient {
                    capacity: 1,
                    durability: 1,
                    flavor: 1,
                    texture: 1,
                    calories: 4,
                },
                amount: 50,
            },
            Mix {
                ingredient: &Ingredient {
                    capacity: 1,
                    durability: 1,
                    flavor: 1,
                    texture: 1,
                    calories: 5,
                },
                amount: 50,
            },
        ];

        assert!(!valid_possibility(&input, 2));

        input = vec![
            Mix {
                ingredient: &Ingredient {
                    capacity: 1,
                    durability: 1,
                    flavor: 1,
                    texture: 1,
                    calories: 1,
                },
                amount: 0,
            },
            Mix {
                ingredient: &Ingredient {
                    capacity: 1,
                    durability: 1,
                    flavor: 1,
                    texture: 1,
                    calories: 5,
                },
                amount: 100,
            },
        ];

        assert!(valid_possibility(&input, 2));
    }

    #[test]
    fn it_works() {
        let input = concat!(
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, ",
            "calories 8\n",
            "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, ",
            "calories 3\n",
        );

        assert_eq!(y15d15(input, 1), 62842880);
        assert_eq!(y15d15(input, 2), 57600000);
    }

    #[test]
    #[ignore]
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day15.txt").unwrap();

        assert_eq!(y15d15(&contents, 1), 13882464);
        assert_eq!(y15d15(&contents, 2), 11171160);
    }
}
