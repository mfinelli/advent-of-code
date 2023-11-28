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
//! TODO

use std::collections::BinaryHeap;
use itertools::Itertools;

/// TODO
#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: u32,
}

/// TODO
#[derive(Clone,Debug)]
struct Mix<'a> {
    ingredient: &'a Ingredient,
    amount: u32,
}

// impl<'a> Mix<'a> {
//     fn compute_score(& self) {
//         let capacity_score = self.amount as i32 * self.ingredient.capacity;
//         let
//     }
// }

/// The solution for the day fifteen challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y15d15::y15d15;
/// // probably read this from the input file...
/// //let input = "";
/// //assert_eq!(y15d15(input), 0);
/// ```
pub fn y15d15(input: &str, part: u32) -> i32 {
    let total = 100;
    let lines: Vec<_> = input.lines().collect();
    let mut ingredients = Vec::new();
    let mut scores = BinaryHeap::new();

    for line in lines {
        let text: Vec<&str> = line.split_whitespace().collect();
        let name = text[0].strip_suffix(':').unwrap().to_string();
        let capacity = text[2].strip_suffix(',').unwrap().parse().unwrap();
        let durability = text[4].strip_suffix(',').unwrap().parse().unwrap();
        let flavor = text[6].strip_suffix(',').unwrap().parse().unwrap();
        let texture = text[8].strip_suffix(',').unwrap().parse().unwrap();
        let calories = text[10].parse().unwrap();

        ingredients.push(Ingredient {
            name: name,
            capacity: capacity,
            durability: durability,
            flavor: flavor,
            texture: texture,
            calories: calories,
        });
    }

    // let mut mixes: HashMap<&String, Vec<Mix>> = HashMap::new();
    let mut mixes = Vec::new();
    for ingredient in &ingredients {
        // let mix = mixes.entry(&ingredient.name).or_default();
        for amount in 0..total +1{
            // mix.push(Mix {
            //     amount: amount,
            //     ingredient: ingredient,
            // });
            mixes.push(Mix {
                amount: amount,
                ingredient: ingredient,
            });
        }
    }

    // println!("{:?}", ingredients);
    // println!("{:?}", mixes);
    let possibilities: Vec<Vec<Mix>> = mixes.into_iter().combinations(ingredients.len()).filter(|p| valid_possibility(&ingredients, p, part)).collect();

    // println!("{:?}", possibilities);
    for possibility in possibilities {
        scores.push(compute_score(&possibility));
    }

    scores.pop().unwrap()





    // 0
}

/// TODO
fn valid_possibility(ingredients: &Vec<Ingredient>, possibility: &Vec<Mix>, part: u32) -> bool {
    let total = 100;
    let sum: u32 = possibility.iter().map(|p| p.amount).sum();
    // println!("{:?}", sum);

    if sum != total {
        return false;
    }

    let names: Vec<&String> = possibility.iter().map(|p| &p.ingredient.name).unique().collect();

    // println!("{:?}", names);
    //
    if names.len() != ingredients.len() {
        return false;
    }

    if part == 2 {
        let calories: u32 = possibility.iter().map(|p| p.ingredient.calories * p.amount).sum();
        if calories != 500 {
            return false;
        }
    }

    true
}

/// TODO
fn compute_score(possibility: &Vec<Mix>) -> i32 {
    let capacity: i32= possibility.iter().map(|p| p.ingredient.capacity * p.amount as i32).sum();
    if capacity <= 0 {
        return 0;
    }

    let durability: i32 = possibility.iter().map(|p| p.ingredient.durability * p.amount as i32).sum();
    if durability <= 0 {
        return 0;
    }

    let flavor: i32 = possibility.iter().map(|p| p.ingredient.flavor * p.amount as i32).sum();
    if flavor <= 0  {
        return 0;
    }

    let texture: i32 = possibility.iter().map(|p| p.ingredient.texture * p.amount as i32).sum();
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
    fn the_solution() {
        let contents = fs::read_to_string("input/2015/day15.txt").unwrap();

        assert_eq!(y15d15(&contents, 1), 13882464);
        assert_eq!(y15d15(&contents, 2), 11171160);
    }
}
