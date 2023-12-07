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

//! Advent of Code 2023 Day 7: <https://adventofcode.com/2023/day/7>
//!
//! TODO

use std::collections::HashMap;
use std::cmp::Ordering;

/// TODO
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Ace(u64),
    King(u64),
    Queen(u64),
    Jack(u64),
    Ten(u64),
    Nine(u64),
    Eight(u64),
    Seven(u64),
    Six(u64),
    Five(u64),
    Four(u64),
    Three(u64),
    Two(u64),
}

/// TODO
#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
}

impl Hand {
    /// TODO
    fn count_cards(&self) -> HashMap<&Card, usize> {
        let mut map = HashMap::new();

        for card in &self.cards {
            *map.entry(card).or_default() += 1;
        }

        map
    }

    /// TODO
    fn is_five_of_a_kind(&self) -> bool {
        self.cards[0] == self.cards[1] && self.cards[1] ==self.cards[2] && self.cards[2] == self.cards[3] && self.cards[3] == self.cards[4]
    }

    /// TODO
    fn is_four_of_a_kind(&self) -> bool {
        // let mut map: HashMap<&Card, usize> = HashMap::new();
        // for card in &self.cards {
        //     *map.entry(card).or_default() += 1;
        // }

        // map.values().any(|v| *v == 4)
        self.count_cards().values().any(|v| *v == 4)
    }

    /// TODO
    fn is_full_house(&self) -> bool {
        let counts = self.count_cards();
        counts.values().any(|v| *v == 3) && counts.values().any(|v| *v == 2)
    }

    /// TODO
    fn is_three_of_a_kind(&self) -> bool {
        let counts = self.count_cards();
        counts.values().any(|v| *v == 3) && !counts.values().any(|v| *v == 2)
    }

    /// TODO
    fn is_two_pair(&self) -> bool {
        self.count_cards().values().filter(|v| **v == 2).count() == 2
    }

    /// TODO
    fn is_one_pair(&self) -> bool {
        let counts = self.count_cards();
        counts.values().any(|v| *v == 2) && !counts.values().any(|v| *v == 3)
    }

    /// TODO
    fn is_high_card(&self) -> bool {
        self.count_cards().values().all(|v| *v == 1)
    }

    /// TODO
    fn cmp_first_card(&self, other: &Self) -> Ordering {
        // println!("comparing {:?} against {:?}", self, other);
        for (i, card) in self.cards.iter().enumerate() {
            // println!("checking {:?}/{:?}", card, &other.cards[i]);
            if card == &other.cards[i] {
                // println!("they're the same!");
                continue;
            }

            // println!("they're not the same: {:?}", card.cmp(&other.cards[i]));
            // println!("they're not the same: {:?}", other.cards[i].cmp(card));

            // return card.cmp(&other.cards[i]);
            return other.cards[i].cmp(card);
        }

        // println!("they're all the same");
        Ordering::Equal
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_five_of_a_kind() {
            if other.is_five_of_a_kind() {
                return self.cmp_first_card(other);
            } else {
                return Ordering::Greater;
            }
        }

        if self.is_four_of_a_kind() {
            if other.is_five_of_a_kind() {
                return Ordering::Less;
            } else if other.is_four_of_a_kind() {
                return self.cmp_first_card(other);
            } else {
                return Ordering::Greater;
            }
        }

        if self.is_full_house() {
            if other.is_five_of_a_kind() || other.is_four_of_a_kind() {
                return Ordering::Less;
            } else if other.is_full_house() {
                return self.cmp_first_card(other);
            } else {
                return Ordering::Greater;
            }
        }

        if self.is_three_of_a_kind() {
            if other.is_five_of_a_kind() || other.is_four_of_a_kind() || other.is_full_house() {
                return Ordering::Less;
            } else if other.is_three_of_a_kind() {
                return self.cmp_first_card(other);
            } else {
                return Ordering::Greater;
            }
        }

        if self.is_two_pair() {
            if other.is_five_of_a_kind() || other.is_four_of_a_kind() || other.is_full_house() || other.is_three_of_a_kind() {
                return Ordering::Less;
            } else if other.is_two_pair() {
                return self.cmp_first_card(other);
            } else {
                return Ordering::Greater;
            }
        }

        if self.is_one_pair() {
            if other.is_five_of_a_kind() || other.is_four_of_a_kind() || other.is_full_house() || other.is_three_of_a_kind() || other.is_two_pair() {
                return Ordering::Less;
            } else if other.is_one_pair() {
                return self.cmp_first_card(other);
            } else {
                return Ordering::Greater;
            }
        }

        if self.is_high_card() {
            if other.is_five_of_a_kind() || other.is_four_of_a_kind() || other.is_full_house() || other.is_three_of_a_kind() || other.is_two_pair() || other.is_one_pair() {
                return Ordering::Less;
            } else {
                return self.cmp_first_card(other);
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {
}

/// The solution for the day seven challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y23d07::y23d07;
/// // probably read this from the input file...
/// let input = "";
/// assert_eq!(y23d07(input), 0);
/// ```
pub fn y23d07(input: &str) -> u64 {
    let mut hands = Vec::new();

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let cards: Vec<_> = parts[0].chars().collect();

        hands.push(Hand {
            cards: [parse_card(cards[0]), parse_card(cards[1]), parse_card(cards[2]), parse_card(cards[3]), parse_card(cards[4])],
            bid: parts[1].parse().unwrap(),
        });
    }

    hands.sort();
    // println!("{:?}", hands);
    let mut winnings = 0;
    for (i, hand) in hands.iter().enumerate() {
        // println!("{:?}", hand);
        winnings += (i as u64 + 1)*hand.bid;
    }

    winnings
}

/// TODO
fn parse_card(card: char) -> Card {
    match card {
        'A' => Card::Ace(14),
        'K' => Card::King(13),
        'Q' => Card::Queen(12),
        'J' => Card::Jack(11),
        'T' => Card::Ten(10),
        '9' => Card::Nine(9),
        '8' => Card::Eight(8),
        '7' => Card::Seven(7),
        '6' => Card::Six(6),
        '5' => Card::Five(5),
        '4' => Card::Four(4),
        '3' => Card::Three(3),
        '2' => Card::Two(2),
        _ => panic!("Invalid card!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_card() {}

    #[test]
    fn test_hand_count_cards() {}

    #[test]
    fn test_hand_is_five_of_a_kind() {
        let mut hand = Hand {
            cards: [Card::Ace(14), Card::Ace(14), Card::Ace(14), Card::Ace(14), Card::Ace(14)],
            bid: 1,
        };

        assert!(hand.is_five_of_a_kind());

        hand.cards = [Card::Ace(14), Card::King(13), Card::Ace(14), Card::Ace(14), Card::Ace(14)];
        assert!(!hand.is_five_of_a_kind());
    }

    #[test]
    fn test_hand_is_four_of_a_kind() {
        let mut hand = Hand {
            cards: [Card::Ace(14), Card::King(13), Card::Ace(14), Card::Ace(14), Card::Ace(14)],
            bid: 1,
        };

        assert!(hand.is_four_of_a_kind());

        hand.cards = [Card::Ace(14), Card::King(13), Card::King(13), Card::Ace(14), Card::Ace(14)];
        assert!(!hand.is_four_of_a_kind());
    }

    #[test]
    fn tit_works() {
        let input = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483\n";

        assert_eq!(y23d07(input), 6440);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day07.txt").unwrap();

        assert_eq!(y23d07(&contents), 0);
    }
}
