/* Copyright 2023-2024 Mario Finelli
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

//! Advent of Code 2023 Day 5: <https://adventofcode.com/2023/day/5>
//!
//! TODO

// use std::collections::HashMap;
use std::ops::Range;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn y23d05(input: &str, part: u8) -> i64 {
    if part == 2 {
        return 0
    }

    let mut locations = BinaryHeap::new();
    let mut seeds: Vec<i64> = Vec::new();

    let lines: Vec<_> = input.lines().collect();
    for number in lines[0].split_whitespace().skip(1) {
        seeds.push(number.parse().unwrap());
    }

    let mut conversions: Vec<Vec<(i64, i64, i64)>> = Vec::new();
    let mut current = Vec::new();
    for line in lines.iter().skip(3) {
        if  line == &"" {
            continue;
        }

        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() != 3 {
            conversions.push(current);
            current = Vec::new();
            continue;
        }

        let destination: i64 = parts[0].parse().unwrap();
        let start: i64 = parts[1].parse().unwrap();
        let delta: i64 = parts[2].parse().unwrap();

        current.push((destination, start, delta));
    }
    conversions.push(current);

    // println!("{:?}", conversions);
    // return 0;

    for seed in seeds {
        let mut f = seed;
        for conversion in &conversions {
            for (destination, start, delta) in conversion {
                // println!("{}, {}, {}", destination, start, delta);
                let range = *start..*start+*delta;
                if range.contains(&f) {
                    let diff = destination - start;
                    // println!("{} is in {}, {}", f, *start, *start+*delta);
                    // println!("adding {}", diff);
                    f += diff;
                    // println!("new f {}", f);
                    break;
                }
            }
        }

        locations.push(Reverse(f));
        // break;
    }



    let Reverse(shortest) = locations.pop().unwrap();
    shortest
}

/// The solution for the day five challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y23d05::y23d05;
/// // probably read this from the input file...
/// let input = "";
/// assert_eq!(y23d05(input, 1), 0);
/// assert_eq!(y23d05(input, 2), 0);
/// ```
pub fn y23d05old(input: &str, part: u32) -> u64 {
    let mut locations = BinaryHeap::new();
    let mut seeds: Vec<u64> = Vec::new();
    let mut seed_ranges: Vec<Range<u64>> = Vec::new();

    let mut seed_to_soil: Vec<(u64, u64, u64, u64)>= Vec::new();
    let mut soil_to_fertilizer: Vec<(u64, u64, u64, u64)>= Vec::new();
    let mut fertilizer_to_water: Vec<(u64, u64, u64, u64)>= Vec::new();
    let mut water_to_light: Vec<(u64, u64, u64, u64)>= Vec::new();
    let mut light_to_temperature: Vec<(u64, u64, u64, u64)>= Vec::new();
    let mut temperature_to_humidity: Vec<(u64, u64, u64, u64)>= Vec::new();
    let mut humidity_to_location: Vec<(u64, u64, u64, u64)>= Vec::new();

    let mut in_seed_to_soil = false;
    let mut in_soil_to_fertilizer = false;
    let mut in_fertilizer_to_water = false;
    let mut in_water_to_light = false;
    let mut in_light_to_temperature = false;
    let mut in_temperature_to_humidity = false;
    let mut in_humidity_to_location = false;

    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            if part == 1 {
                for number in line.split_whitespace().skip(1) {
                    seeds.push(number.parse().unwrap());
                }
            } else {
                for pair in line.split_whitespace().skip(1).collect::<Vec<_>>().chunks(2) {
                    let start: u64 = pair[0].parse().unwrap();
                    let len: u64 = pair[1].parse().unwrap();

                    // for number in start..start+len {
                        seed_ranges.push(start..start+len);
                    // }
                }
            }
        } else {
            if line == "seed-to-soil map:" {
                in_seed_to_soil = true;
                in_soil_to_fertilizer = false;
                in_fertilizer_to_water = false;
                in_water_to_light = false;
                in_light_to_temperature = false;
                in_temperature_to_humidity = false;
                in_humidity_to_location = false;
            } else if line == "soil-to-fertilizer map:" {
                in_seed_to_soil = false;
                in_soil_to_fertilizer = true;
                in_fertilizer_to_water = false;
                in_water_to_light = false;
                in_light_to_temperature = false;
                in_temperature_to_humidity = false;
                in_humidity_to_location = false;
            } else if line == "fertilizer-to-water map:" {
                in_seed_to_soil = false;
                in_soil_to_fertilizer = false;
                in_fertilizer_to_water = true;
                in_water_to_light = false;
                in_light_to_temperature = false;
                in_temperature_to_humidity = false;
                in_humidity_to_location = false;
            } else if line == "water-to-light map:" {
                in_seed_to_soil = false;
                in_soil_to_fertilizer = false;
                in_fertilizer_to_water = false;
                in_water_to_light = true;
                in_light_to_temperature = false;
                in_temperature_to_humidity = false;
                in_humidity_to_location = false;
            } else if line == "light-to-temperature map:" {
                in_seed_to_soil = false;
                in_soil_to_fertilizer = false;
                in_fertilizer_to_water = false;
                in_water_to_light = false;
                in_light_to_temperature = true;
                in_temperature_to_humidity = false;
                in_humidity_to_location = false;
            } else if line == "temperature-to-humidity map:" {
                in_seed_to_soil = false;
                in_soil_to_fertilizer = false;
                in_fertilizer_to_water = false;
                in_water_to_light = false;
                in_light_to_temperature = false;
                in_temperature_to_humidity = true;
                in_humidity_to_location = false;
            } else if line == "humidity-to-location map:" {
                in_seed_to_soil = false;
                in_soil_to_fertilizer = false;
                in_fertilizer_to_water = false;
                in_water_to_light = false;
                in_light_to_temperature = false;
                in_temperature_to_humidity = false;
                in_humidity_to_location = true;
            } else if line != "" {
                let parts: Vec<_> = line.split_whitespace().collect();
                let dest: u64 = parts[0].parse().unwrap();
                let src: u64 = parts[1].parse().unwrap();
                let len: u64 = parts[2].parse().unwrap();
                let tuple = (dest, dest+len, src, src+len);

                if in_seed_to_soil {
                    seed_to_soil.push(tuple.clone());
                    continue;
                }

                if in_soil_to_fertilizer {
                    soil_to_fertilizer.push(tuple.clone());
                    continue;
                }

                if in_fertilizer_to_water {
                    fertilizer_to_water.push(tuple.clone());
                    continue;
                }

                if in_water_to_light {
                    water_to_light.push(tuple.clone());
                    continue;
                }

                if in_light_to_temperature {
                    light_to_temperature.push(tuple.clone());
                    continue;
                }

                if in_temperature_to_humidity {
                    temperature_to_humidity.push(tuple.clone());
                    continue;
                }

                if in_humidity_to_location {
                    humidity_to_location.push(tuple.clone());
                    continue;
                }
            }
        }
    }

    println!("there are {} seeds", seeds.len());

    // let mut count = 1;
    if part == 1 {
    for seed in &seeds {
        let soil = map_value_to_range(*seed, &seed_to_soil);
        let fertilizer = map_value_to_range(soil, &soil_to_fertilizer);
        let water = map_value_to_range(fertilizer, &fertilizer_to_water);
        let light = map_value_to_range(water, &water_to_light);
        let temperature = map_value_to_range(light, &light_to_temperature);
        let humidity = map_value_to_range(temperature, &temperature_to_humidity);
        let location = map_value_to_range(humidity, &humidity_to_location);

        locations.push(Reverse(location));

        // println!("{}/{}", count, seeds.len());
        // count += 1;
    }
    } else {
        let mut count = 1;
        for range in seed_ranges {
            for seed in range {
        let soil = map_value_to_range(seed, &seed_to_soil);
        let fertilizer = map_value_to_range(soil, &soil_to_fertilizer);
        let water = map_value_to_range(fertilizer, &fertilizer_to_water);
        let light = map_value_to_range(water, &water_to_light);
        let temperature = map_value_to_range(light, &light_to_temperature);
        let humidity = map_value_to_range(temperature, &temperature_to_humidity);
        let location = map_value_to_range(humidity, &humidity_to_location);

        locations.push(Reverse(location));
        if count % 100000 == 0 {
            println!("processed {}", count);
        }
        count+=1;
            }
        }
    }

    // println!("{:?}", seeds);
    // println!("{:?}", seed_to_soil);
    // println!("{:?}", soil_to_fertilizer);
    // println!("{:?}", fertilizer_to_water);
    // println!("{:?}", water_to_light);
    // println!("{:?}", light_to_temperature);
    // println!("{:?}", temperature_to_humidity);
    // println!("{:?}", humidity_to_location);

    let Reverse(shortest) = locations.pop().unwrap();
    shortest
}

/// TODO
fn map_value_to_range(val: u64, mappings: &Vec<(u64, u64, u64, u64)>) -> u64 {
    for (dest, _, src, src_end) in mappings {
        if val >= *src && val < *src_end {
            return dest + (val - src);
        }
    }

    val
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_map_value_to_range() {
        assert_eq!(map_value_to_range(10 ,&vec![(50, 52, 98,100), (52, 100, 50,98)]), 10);
        assert_eq!(map_value_to_range(49,&vec![(50, 52, 98,100), (52, 100, 50,98)]), 49);
        assert_eq!(map_value_to_range(50,&vec![(50, 52, 98,100), (52, 100, 50,98)]), 52);
        assert_eq!(map_value_to_range(51,&vec![(50, 52, 98,100), (52, 100, 50,98)]), 53);
        assert_eq!(map_value_to_range(96,&vec![(50,52, 98,100), (52,100, 50,98)]), 98);
        assert_eq!(map_value_to_range(97,&vec![(50,52, 98,100), (52,100, 50,98)]), 99);
        assert_eq!(map_value_to_range(98,&vec![(50,52, 98,100), (52,100, 50,98)]), 50);
        assert_eq!(map_value_to_range(99,&vec![(50,52, 98,100), (52,100, 50,98)]), 51);
    }

    #[test]
    fn it_works() {
        let input = concat!(
            "seeds: 79 14 55 13\n",
            "\n",
            "seed-to-soil map:\n",
            "50 98 2\n",
            "52 50 48\n",
            "\n",
            "soil-to-fertilizer map:\n",
            "0 15 37\n",
            "37 52 2\n",
            "39 0 15\n",
            "\n",
            "fertilizer-to-water map:\n",
            "49 53 8\n",
            "0 11 42\n",
            "42 0 7\n",
            "57 7 4\n",
            "\n",
            "water-to-light map:\n",
            "88 18 7\n",
            "18 25 70\n",
            "\n",
            "light-to-temperature map:\n",
            "45 77 23\n",
            "81 45 19\n",
            "68 64 13\n",
            "\n",
            "temperature-to-humidity map:\n",
            "0 69 1\n",
            "1 0 69\n",
            "\n",
            "humidity-to-location map:\n",
            "60 56 37\n",
            "56 93 4\n",
        );

        assert_eq!(y23d05(input, 1), 35);
        assert_eq!(y23d05(input, 2), 46);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day05.txt").unwrap();

        assert_eq!(y23d05(&contents, 1), 324724204);
        assert_eq!(y23d05(&contents, 2), 150985364);
    }
}
