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
//! Today's challenge was not terribly difficult to understand or to implement
//! but ended up taking me quite a bit of time to actually complete because
//! the implementation started to grow quite large. In part two I had a bug
//! where I was returning `false` from the `is_kind_of_hand` functions when I
//! should have been just letting the value equal `0` if there weren't any
//! jokers. The general strategy that I adopted today is to build a custom
//! type `Hand` which implements its own sorting functions based on the input
//! from the prompt. Then the actual daily function just needs to parse the
//! input and sort the result and the work is mostly done.

use std::cmp::Ordering;
use std::collections::HashMap;

/// The Card enum is just an easier way to encode the cards in the code. Their
/// specific value isn't important as long as it respects the ordering rule as
/// defined in the prompt (e.g., in part two `Joker` must have the lowest
/// value).
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
    Joker(u64),
}

/// Just used to represent the value of the Joker card consistently where it's
/// used.
const JOKER: Card = Card::Joker(1);

/// Represents a Hand as defined by the input: includes the card and the bid
/// for that hand. Also tracks the problem part `1` or `2` to make use of the
/// joker rules when sorting hands.
#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
    part: u32,
}

impl Hand {
    /// This function returns a [`std::collections::HashMap`] of all of the
    /// cards in a hand (as the keys) with the number of times that they
    /// appear (as the values).
    fn count_cards(&self) -> HashMap<&Card, usize> {
        let mut map = HashMap::new();

        for card in &self.cards {
            *map.entry(card).or_default() += 1;
        }

        map
    }

    /// Returns true if the hand is a five-of-a-kind and false otherwise.
    fn is_five_of_a_kind(&self) -> bool {
        if self.part == 1 {
            self.cards[0] == self.cards[1]
                && self.cards[1] == self.cards[2]
                && self.cards[2] == self.cards[3]
                && self.cards[3] == self.cards[4]
        } else {
            let counts = self.count_cards();
            let jokers = match counts.get(&JOKER) {
                None => 0,
                Some(jokers) => *jokers,
            };

            if counts.values().any(|v| *v == 5) {
                true
            } else if (counts.values().any(|v| *v == 4) && jokers == 1)
                || (counts.values().any(|v| *v == 3) && jokers == 2)
                || (counts.values().any(|v| *v == 2) && jokers == 3)
            {
                return true;
            } else {
                return counts.values().any(|v| *v == 1) && jokers == 4;
            }
        }
    }

    /// Returns true of the hand is a four-of-a-kind and false otherwise.
    fn is_four_of_a_kind(&self) -> bool {
        if self.is_five_of_a_kind() {
            return false;
        }

        if self.part == 1 {
            self.count_cards().values().any(|v| *v == 4)
        } else {
            let counts = self.count_cards();
            let jokers = match counts.get(&JOKER) {
                None => 0,
                Some(jokers) => *jokers,
            };

            if counts.values().any(|v| *v == 4) {
                true
            } else if (counts.values().any(|v| *v == 3) && jokers == 1)
                || (counts.values().filter(|v| **v == 2).count() == 2
                    && jokers == 2)
            {
                // we have three of a kind plus one joker or
                // we have two pairs, one is jokers so we can make 4
                return true;
            } else {
                return counts.values().any(|v| *v == 1) && jokers == 3;
            }
        }
    }

    /// Returns true if the hand is a full house and false otherwise.
    fn is_full_house(&self) -> bool {
        if self.is_five_of_a_kind() || self.is_four_of_a_kind() {
            return false;
        }

        let counts = self.count_cards();

        if self.part == 1 {
            counts.values().any(|v| *v == 3) && counts.values().any(|v| *v == 2)
        } else {
            let jokers = match counts.get(&JOKER) {
                None => 0,
                Some(jokers) => *jokers,
            };

            if counts.values().any(|v| *v == 3)
                && counts.values().any(|v| *v == 2)
            {
                true
            } else if counts.values().filter(|v| **v == 2).count() == 2
                && jokers == 1
            {
                // two pairs plus a joker to make a full house
                return true;
            } else {
                return false;
            }
        }
    }

    /// Returns true if the hand is a three of a kind and false otherwise.
    fn is_three_of_a_kind(&self) -> bool {
        if self.is_five_of_a_kind()
            || self.is_four_of_a_kind()
            || self.is_full_house()
        {
            return false;
        }

        let counts = self.count_cards();

        if self.part == 1 {
            counts.values().any(|v| *v == 3)
        } else {
            let jokers = match counts.get(&JOKER) {
                None => 0,
                Some(jokers) => *jokers,
            };

            if counts.values().any(|v| *v == 3) {
                true
            } else if counts.values().any(|v| *v == 2) && jokers == 1 {
                return true;
            } else {
                return counts.values().any(|v| *v == 1) && jokers == 2;
            }
        }
    }

    /// Returns true if the hand is two pairs and false otherwise.
    fn is_two_pair(&self) -> bool {
        if self.is_five_of_a_kind()
            || self.is_four_of_a_kind()
            || self.is_full_house()
            || self.is_three_of_a_kind()
        {
            return false;
        }

        if self.part == 1 {
            self.count_cards().values().filter(|v| **v == 2).count() == 2
        } else {
            let counts = self.count_cards();
            let jokers = match counts.get(&JOKER) {
                None => 0,
                Some(jokers) => *jokers,
            };

            if counts.values().filter(|v| **v == 2).count() == 2 {
                true
            } else {
                counts.values().any(|v| *v == 2) && jokers == 1
            }
        }
    }

    /// Returns true if the hand only has a single pair and false otherwise.
    fn is_one_pair(&self) -> bool {
        if self.is_five_of_a_kind()
            || self.is_four_of_a_kind()
            || self.is_full_house()
            || self.is_three_of_a_kind()
            || self.is_two_pair()
        {
            return false;
        }

        let counts = self.count_cards();

        if self.part == 1 {
            counts.values().any(|v| *v == 2)
        } else {
            let counts = self.count_cards();
            let jokers = match counts.get(&JOKER) {
                None => 0,
                Some(jokers) => *jokers,
            };

            if counts.values().any(|v| *v == 2) {
                true
            } else {
                counts.values().all(|v| *v == 1) && jokers == 1
            }
        }
    }

    /// Returns true if the hand has all different cards and false otherwise.
    fn is_high_card(&self) -> bool {
        if self.is_five_of_a_kind()
            || self.is_four_of_a_kind()
            || self.is_full_house()
            || self.is_three_of_a_kind()
            || self.is_two_pair()
            || self.is_one_pair()
        {
            return false;
        }

        if self.part == 1 {
            self.count_cards().values().all(|v| *v == 1)
        } else {
            let counts = self.count_cards();
            match counts.get(&JOKER) {
                None => counts.values().all(|v| *v == 1),
                Some(_) => false,
            }
        }
    }

    /// This function implements the second part of the ordering as described
    /// in the prompt and is only used when two hands have the same type (e.g.,
    /// two five-of-a-kinds): it compares the first card of each hand and if
    /// they're the same moving on to the second and the third if those are the
    /// same, etc. Compared to e.g., poker the comparison is simply done on
    /// the _first_ card which is not necessarily the highest or most valuable
    /// card.
    fn cmp_first_card(&self, other: &Self) -> Ordering {
        for (i, card) in self.cards.iter().enumerate() {
            if card == &other.cards[i] {
                continue;
            }

            return other.cards[i].cmp(card);
        }

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
            if other.is_five_of_a_kind()
                || other.is_four_of_a_kind()
                || other.is_full_house()
            {
                return Ordering::Less;
            } else if other.is_three_of_a_kind() {
                return self.cmp_first_card(other);
            } else {
                return Ordering::Greater;
            }
        }

        if self.is_two_pair() {
            if other.is_five_of_a_kind()
                || other.is_four_of_a_kind()
                || other.is_full_house()
                || other.is_three_of_a_kind()
            {
                return Ordering::Less;
            } else if other.is_two_pair() {
                return self.cmp_first_card(other);
            } else {
                return Ordering::Greater;
            }
        }

        if self.is_one_pair() {
            if other.is_five_of_a_kind()
                || other.is_four_of_a_kind()
                || other.is_full_house()
                || other.is_three_of_a_kind()
                || other.is_two_pair()
            {
                return Ordering::Less;
            } else if other.is_one_pair() {
                return self.cmp_first_card(other);
            } else {
                return Ordering::Greater;
            }
        }

        if self.is_high_card() {
            if other.is_five_of_a_kind()
                || other.is_four_of_a_kind()
                || other.is_full_house()
                || other.is_three_of_a_kind()
                || other.is_two_pair()
                || other.is_one_pair()
            {
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

impl Eq for Hand {}

/// The solution for the day seven challenge.
///
/// TODO
///
/// # Example
/// ```rust
/// # use aoc::y23d07::y23d07;
/// // probably read this from the input file...
/// let input = concat!(
///     "2345A 2\n2345J 5\nJ345A 3\n32T3K 7\nT55J5 17\nKK677 11\nKTJJT 23\n",
///     "QQQJA 19\nJJJJJ 29\nJAAAA 37\nAAAAJ 43\nAAAAA 53\n2AAAA 13\n",
///     "2JJJJ 41\nJJJJ2 31",
/// );
/// assert_eq!(y23d07(input, 1), 3542);
/// assert_eq!(y23d07(input, 2), 3667);
/// ```
pub fn y23d07(input: &str, part: u32) -> u64 {
    let mut hands = Vec::new();

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let cards: Vec<_> = parts[0].chars().collect();

        hands.push(Hand {
            cards: [
                parse_card(cards[0], part),
                parse_card(cards[1], part),
                parse_card(cards[2], part),
                parse_card(cards[3], part),
                parse_card(cards[4], part),
            ],
            bid: parts[1].parse().unwrap(),
            part,
        });
    }

    hands.sort();
    let mut winnings = 0;
    for (i, hand) in hands.iter().enumerate() {
        winnings += (i as u64 + 1) * hand.bid;
    }

    winnings
}

/// This function returns an element from our `Card` enum based on its
/// character representation.
fn parse_card(card: char, part: u32) -> Card {
    match card {
        'A' => Card::Ace(14),
        'K' => Card::King(13),
        'Q' => Card::Queen(12),
        'J' => {
            if part == 1 {
                Card::Jack(11)
            } else {
                JOKER
            }
        }
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
    fn test_parse_card() {
        assert_eq!(parse_card('A', 1), Card::Ace(14));
        assert_eq!(parse_card('K', 1), Card::King(13));
        assert_eq!(parse_card('K', 2), Card::King(13));
        assert_eq!(parse_card('J', 1), Card::Jack(11));
        assert_eq!(parse_card('J', 2), JOKER);
        assert_eq!(parse_card('9', 1), Card::Nine(9));
        assert_eq!(parse_card('4', 2), Card::Four(4));
    }

    #[test]
    fn test_hand_count_cards() {
        let mut hand = Hand {
            cards: [
                Card::Ace(14),
                Card::King(13),
                Card::King(13),
                Card::Two(2),
                Card::Nine(9),
            ],
            bid: 1,
            part: 1,
        };

        let result = HashMap::from([
            (&Card::Ace(14), 1),
            (&Card::King(13), 2),
            (&Card::Two(2), 1),
            (&Card::Nine(9), 1),
        ]);

        assert_eq!(hand.count_cards(), result);
        hand.part = 2;
        assert_eq!(hand.count_cards(), result);
    }

    #[test]
    fn test_hand_is_five_of_a_kind() {
        let mut hand = Hand {
            cards: [
                Card::Ace(14),
                Card::Ace(14),
                Card::Ace(14),
                Card::Ace(14),
                Card::Ace(14),
            ],
            bid: 1,
            part: 1,
        };

        assert!(hand.is_five_of_a_kind());

        hand.cards = [
            Card::Ace(14),
            Card::King(13),
            Card::Ace(14),
            Card::Ace(14),
            Card::Ace(14),
        ];
        assert!(!hand.is_five_of_a_kind());

        let mut hand = Hand {
            cards: [
                Card::Ace(14),
                Card::Ace(14),
                Card::Ace(14),
                Card::Ace(14),
                Card::Ace(14),
            ],
            bid: 1,
            part: 2,
        };

        assert!(hand.is_five_of_a_kind());

        hand.cards = [
            Card::Ace(14),
            Card::King(13),
            Card::Ace(14),
            Card::Ace(14),
            Card::Ace(14),
        ];
        assert!(!hand.is_five_of_a_kind());

        hand.cards = [
            Card::Ace(14),
            JOKER,
            Card::Ace(14),
            Card::Ace(14),
            Card::Ace(14),
        ];
        assert!(hand.is_five_of_a_kind());

        hand.cards =
            [Card::Ace(14), JOKER, JOKER, Card::Ace(14), Card::Ace(14)];
        assert!(hand.is_five_of_a_kind());

        hand.cards = [Card::Ace(14), JOKER, JOKER, JOKER, Card::Ace(14)];
        assert!(hand.is_five_of_a_kind());

        hand.cards = [Card::Ace(14), JOKER, JOKER, JOKER, JOKER];
        assert!(hand.is_five_of_a_kind());
    }

    #[test]
    fn test_hand_is_four_of_a_kind() {
        let mut hand = Hand {
            cards: [
                Card::Ace(14),
                Card::King(13),
                Card::Ace(14),
                Card::Ace(14),
                Card::Ace(14),
            ],
            bid: 1,
            part: 1,
        };

        assert!(hand.is_four_of_a_kind());

        hand.cards = [
            Card::Ace(14),
            Card::King(13),
            Card::King(13),
            Card::Ace(14),
            Card::Ace(14),
        ];
        assert!(!hand.is_four_of_a_kind());

        let mut hand = Hand {
            cards: [
                Card::Ace(14),
                Card::King(13),
                Card::Ace(14),
                Card::Ace(14),
                Card::Ace(14),
            ],
            bid: 1,
            part: 2,
        };

        assert!(hand.is_four_of_a_kind());

        hand.cards = [
            Card::Ace(14),
            Card::King(13),
            Card::King(13),
            Card::Ace(14),
            Card::Ace(14),
        ];
        assert!(!hand.is_four_of_a_kind());

        hand.cards = [
            Card::Ace(14),
            JOKER,
            Card::King(13),
            Card::Ace(14),
            Card::Ace(14),
        ];
        assert!(hand.is_four_of_a_kind());

        hand.cards =
            [JOKER, JOKER, Card::King(13), Card::Ace(14), Card::Ace(14)];
        assert!(hand.is_four_of_a_kind());

        hand.cards = [JOKER, JOKER, Card::King(13), JOKER, Card::Ace(14)];
        assert!(hand.is_four_of_a_kind());

        hand.cards =
            [JOKER, JOKER, Card::King(13), Card::Queen(12), Card::Ace(14)];
        assert!(!hand.is_four_of_a_kind());

        hand.cards = [
            JOKER,
            Card::Ace(14),
            Card::Ace(14),
            Card::Ace(14),
            Card::Ace(14),
        ];
        assert!(!hand.is_four_of_a_kind());
    }

    #[test]
    fn test_hand_is_full_house() {
        let mut hand = Hand {
            cards: [
                Card::Ace(14),
                Card::King(13),
                Card::King(13),
                Card::Ace(14),
                Card::Ace(14),
            ],
            bid: 1,
            part: 1,
        };

        assert!(hand.is_full_house());

        hand.cards = [
            Card::Ace(14),
            Card::King(13),
            Card::Queen(12),
            Card::Ace(14),
            Card::Ace(14),
        ];
        assert!(!hand.is_four_of_a_kind());

        let mut hand = Hand {
            cards: [
                Card::Ace(14),
                Card::King(13),
                Card::King(13),
                Card::Ace(14),
                Card::Ace(14),
            ],
            bid: 1,
            part: 2,
        };

        assert!(hand.is_full_house());

        hand.cards = [
            Card::Ace(14),
            Card::King(13),
            Card::Queen(12),
            Card::Ace(14),
            Card::Ace(14),
        ];
        assert!(!hand.is_four_of_a_kind());

        hand.cards = [
            Card::Ace(14),
            JOKER,
            Card::King(13),
            Card::King(13),
            Card::Ace(14),
        ];
        assert!(hand.is_full_house());
    }

    #[test]
    fn test_hand_is_three_of_a_kind() {
        let mut hand = Hand {
            cards: [
                Card::Ace(14),
                Card::Queen(12),
                Card::King(13),
                Card::Ace(14),
                Card::Ace(14),
            ],
            bid: 1,
            part: 1,
        };

        assert!(hand.is_three_of_a_kind());

        hand.cards = [
            Card::Ace(14),
            Card::King(13),
            Card::Queen(12),
            Card::Ten(10),
            Card::Ace(14),
        ];
        assert!(!hand.is_three_of_a_kind());

        let mut hand = Hand {
            cards: [
                Card::Ace(14),
                Card::Queen(12),
                Card::King(13),
                Card::Ace(14),
                Card::Ace(14),
            ],
            bid: 1,
            part: 2,
        };

        assert!(hand.is_three_of_a_kind());

        hand.cards = [
            Card::Ace(14),
            Card::King(13),
            Card::Queen(12),
            Card::Ten(10),
            Card::Ace(14),
        ];
        assert!(!hand.is_three_of_a_kind());

        hand.cards = [
            Card::Ace(14),
            Card::King(13),
            JOKER,
            Card::Ten(10),
            Card::Ace(14),
        ];
        assert!(hand.is_three_of_a_kind());
    }

    #[test]
    fn test_hand_is_two_pair() {
        let mut hand = Hand {
            cards: [
                Card::Ace(14),
                Card::Queen(12),
                Card::King(13),
                Card::King(13),
                Card::Ace(14),
            ],
            bid: 1,
            part: 1,
        };

        assert!(hand.is_two_pair());

        hand.cards = [
            Card::Ace(14),
            Card::King(13),
            Card::Queen(12),
            Card::Ten(10),
            Card::Ace(14),
        ];
        assert!(!hand.is_two_pair());

        let hand = Hand {
            cards: [
                Card::Ace(14),
                Card::Queen(12),
                JOKER,
                Card::King(13),
                Card::Ace(14),
            ],
            bid: 1,
            part: 2,
        };

        assert!(!hand.is_two_pair()); // three-of-a-kind
    }

    #[test]
    fn test_hand_is_one_pair() {
        let mut hand = Hand {
            cards: [
                Card::Ace(14),
                Card::Queen(12),
                Card::King(13),
                Card::Ten(10),
                Card::Ace(14),
            ],
            bid: 1,
            part: 1,
        };

        assert!(hand.is_one_pair());

        hand.cards = [
            Card::Ace(14),
            Card::King(13),
            Card::Queen(12),
            Card::Ten(10),
            Card::Nine(9),
        ];
        assert!(!hand.is_one_pair());

        let mut hand = Hand {
            cards: [
                Card::Ace(14),
                Card::Queen(12),
                Card::King(13),
                Card::Ten(10),
                Card::Ace(14),
            ],
            bid: 1,
            part: 2,
        };

        assert!(hand.is_one_pair());

        hand.cards = [
            Card::Ace(14),
            Card::King(13),
            Card::Queen(12),
            Card::Ten(10),
            Card::Nine(9),
        ];
        assert!(!hand.is_one_pair());

        hand.cards = [
            Card::Ace(14),
            Card::King(13),
            Card::Queen(12),
            Card::Ten(10),
            JOKER,
        ];
        assert!(hand.is_one_pair());
    }

    #[test]
    fn test_hand_is_high_card() {
        let mut hand = Hand {
            cards: [
                Card::Ace(14),
                Card::Queen(12),
                Card::King(13),
                Card::Ten(10),
                Card::Nine(9),
            ],
            bid: 1,
            part: 1,
        };

        assert!(hand.is_high_card());

        hand.cards = [
            Card::Ace(14),
            Card::King(13),
            Card::Queen(12),
            Card::Ten(10),
            Card::Ace(14),
        ];
        assert!(!hand.is_high_card());

        let mut hand = Hand {
            cards: [
                Card::Ace(14),
                Card::Queen(12),
                Card::King(13),
                Card::Ten(10),
                Card::Nine(9),
            ],
            bid: 1,
            part: 2,
        };

        assert!(hand.is_high_card());

        hand.cards = [
            Card::Ace(14),
            Card::King(13),
            Card::Queen(12),
            Card::Ten(10),
            JOKER,
        ];
        assert!(!hand.is_high_card());
    }

    #[test]
    fn test_hand_cmp_first_card() {
        let hand = Hand {
            cards: [
                Card::Five(5),
                Card::Queen(12),
                Card::King(13),
                Card::Ten(10),
                Card::Nine(9),
            ],
            bid: 1,
            part: 1,
        };

        let mut other = Hand {
            cards: [
                Card::Five(5),
                Card::Queen(12),
                Card::King(13),
                Card::Ten(10),
                Card::Nine(9),
            ],
            bid: 2,
            part: 2,
        };

        assert_eq!(hand.cmp_first_card(&other), Ordering::Equal);

        other.cards = [
            Card::Six(6),
            Card::Queen(12),
            Card::King(13),
            Card::Ten(10),
            Card::Nine(9),
        ];
        assert_eq!(hand.cmp_first_card(&other), Ordering::Less);

        other.cards = [
            Card::Four(4),
            Card::Queen(12),
            Card::King(13),
            Card::Ten(10),
            Card::Nine(9),
        ];
        assert_eq!(hand.cmp_first_card(&other), Ordering::Greater);

        other.cards = [
            Card::Five(5),
            Card::King(13),
            Card::King(13),
            Card::Ten(10),
            Card::Nine(9),
        ];
        assert_eq!(hand.cmp_first_card(&other), Ordering::Less);

        other.cards = [
            Card::Five(5),
            Card::Ten(10),
            Card::King(13),
            Card::Ten(10),
            Card::Nine(9),
        ];
        assert_eq!(hand.cmp_first_card(&other), Ordering::Greater);
    }

    #[test]
    fn it_works() {
        let input = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483\n";

        assert_eq!(y23d07(input, 1), 6440);
        assert_eq!(y23d07(input, 2), 5905);
    }

    #[test]
    fn the_solution() {
        let contents = fs::read_to_string("input/2023/day07.txt").unwrap();

        assert_eq!(y23d07(&contents, 1), 249748283);
        assert_eq!(y23d07(&contents, 2), 248029057);
    }
}
