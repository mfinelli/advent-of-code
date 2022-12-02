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

use std::fs;

// use aoc::dayone;
use aoc::daytwo;

fn main() {
    // let contents = fs::read_to_string("src/input/day1.txt").expect("Unable to open input file.");
    let contents = fs::read_to_string("src/input/day2.txt").expect("Unable to open input file.");

    // let r = dayone::dayone(&contents, 1);
    // println!("{}", r);

    // let q = dayone::dayone(&contents, 3);
    // println!("{}", q);

    let r = daytwo::daytwo(&contents, 1);
    println!("{}", r);

    let q = daytwo::daytwo(&contents, 2);
    println!("{}", q);
}
