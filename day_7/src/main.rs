use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum HandType {
    // * all five cards have the same label: `AAAAA`
    FiveOfAKind(String),
    // four cards have the same label and one card has a different label: `AA8AA`
    FourOfAKind(String),
    // three cards have the same label, and the remaining two cards share a different label: `23332`
    FullHouse(String),
    // three cards have the same label, and the remaining two cards are each different from any other card in the hand: `TTT98`
    ThreeOfAKind(String),
    // two cards share one label, two other cards share a second label, and the remaining card has a third label: `23432`
    TwoPair(String),
    // two cards share one label, and the other three cards have a different label from the pair and each other: `A23A4`
    OnePair(String),
    // where all cards' labels are distinct: `23456`
    HighCard(String),
}

impl HandType {
    fn discriminant(&self) -> usize {
        match self {
            Self::FiveOfAKind(_) => 0,
            Self::FourOfAKind(_) => 1,
            Self::FullHouse(_) => 2,
            Self::ThreeOfAKind(_) => 3,
            Self::TwoPair(_) => 4,
            Self::OnePair(_) => 5,
            Self::HighCard(_) => 6,
        }
    }

    fn cards(&self) -> &String {
        match self {
            Self::FiveOfAKind(s) => s,
            Self::FourOfAKind(s) => s,
            Self::FullHouse(s) => s,
            Self::ThreeOfAKind(s) => s,
            Self::TwoPair(s) => s,
            Self::OnePair(s) => s,
            Self::HighCard(s) => s,
        }
    }

    fn from_str_with_joker(s: &str) -> Result<Self, String> {
        const JOKER: char = 'J';
        const CARD_ORDER: [char; 13] = [
            'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
        ];

        if !s.contains(JOKER) {
            return Self::from_str(s);
        }

        let chars: Vec<char> = s.trim().chars().filter(|&c| c != JOKER).collect();

        let chars_len = chars.len();
        let joker_count = s.len() - chars_len;

        let mut chars_count: HashMap<char, usize> =
            chars.iter().fold(HashMap::new(), |mut acc, &c| {
                let count = acc.entry(c).or_insert(0);
                *count += 1;
                acc
            });

        let owned: String = s.to_string();

        let mut greatest_entry = chars_count
            .iter_mut()
            .max_by(|count_a, count_b| count_a.1.cmp(&count_b.1));

        if let Some(entry) = greatest_entry {
            *entry.1 += joker_count;
        } else {
            return Ok(HandType::FiveOfAKind(owned));
        }

        match chars_count.len() {
            1 => Ok(HandType::FiveOfAKind(owned)),
            2 => {
                if chars_count.values().any(|&v| v == 4) {
                    Ok(HandType::FourOfAKind(owned))
                } else {
                    Ok(HandType::FullHouse(owned))
                }
            }
            3 => {
                if chars_count.values().any(|&v| v == 3) {
                    Ok(HandType::ThreeOfAKind(owned))
                } else {
                    Ok(HandType::TwoPair(owned))
                }
            }
            4 => Ok(HandType::OnePair(owned)),
            5 => Ok(HandType::HighCard(owned)),
            _ => Err(format!("Invalid hand type: {chars:?}'")),
        }
    }

    fn partial_cmp_with_joker(&self, other: &Self) -> Option<std::cmp::Ordering> {
        const CARD_ORDER: [char; 13] = [
            'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
        ];

        let discriminant_self = self.discriminant();
        let discriminant_other = other.discriminant();

        if discriminant_self == discriminant_other {
            let self_cards = self.cards();
            let other_cards = other.cards();

            let order = self_cards.chars().zip(other_cards.chars()).fold(
                std::cmp::Ordering::Equal,
                |acc, (self_card, other_card)| {
                    if acc == std::cmp::Ordering::Equal {
                        let self_card_index = CARD_ORDER
                            .iter()
                            .position(|&c| c == self_card)
                            .expect("Invalid card");

                        let other_card_index = CARD_ORDER
                            .iter()
                            .position(|&c| c == other_card)
                            .expect("Invalid card");

                        other_card_index.cmp(&self_card_index)
                    } else {
                        acc
                    }
                },
            );

            Some(order)
        } else {
            discriminant_self.partial_cmp(&discriminant_other)
        }
    }
}

impl FromStr for HandType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.trim().chars().collect();

        if chars.len() != 5 {
            return Err(format!("Invalid hand type: '{chars:?}'"));
        }

        let chars_count: HashMap<char, usize> = chars.iter().fold(HashMap::new(), |mut acc, &c| {
            let count = acc.entry(c).or_insert(0);
            *count += 1;
            acc
        });

        let owned: String = s.to_string();

        match chars_count.len() {
            1 => Ok(HandType::FiveOfAKind(owned)),
            2 => {
                if chars_count.values().any(|&v| v == 4) {
                    Ok(HandType::FourOfAKind(owned))
                } else {
                    Ok(HandType::FullHouse(owned))
                }
            }
            3 => {
                if chars_count.values().any(|&v| v == 3) {
                    Ok(HandType::ThreeOfAKind(owned))
                } else {
                    Ok(HandType::TwoPair(owned))
                }
            }
            4 => Ok(HandType::OnePair(owned)),
            5 => Ok(HandType::HighCard(owned)),
            _ => Err(format!("Invalid hand type: {chars:?}'")),
        }
    }
}

impl std::cmp::PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        const CARD_ORDER: [char; 13] = [
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ];

        let discriminant_self = self.discriminant();
        let discriminant_other = other.discriminant();

        if discriminant_self == discriminant_other {
            let self_cards = self.cards();
            let other_cards = other.cards();

            let order = self_cards.chars().zip(other_cards.chars()).fold(
                std::cmp::Ordering::Equal,
                |acc, (self_card, other_card)| {
                    if acc == std::cmp::Ordering::Equal {
                        let self_card_index = CARD_ORDER
                            .iter()
                            .position(|&c| c == self_card)
                            .expect("Invalid card");

                        let other_card_index = CARD_ORDER
                            .iter()
                            .position(|&c| c == other_card)
                            .expect("Invalid card");

                        other_card_index.cmp(&self_card_index)
                    } else {
                        acc
                    }
                },
            );

            Some(order)
        } else {
            discriminant_self.partial_cmp(&discriminant_other)
        }
    }
}

#[derive(Debug, PartialEq)]
struct Bid(HandType, usize);

impl Bid {
    fn from_str_with_joker(s: &str) -> Result<Self, String> {
        let parts = s
            .split_whitespace()
            .map(|p| p.trim())
            .collect::<Vec<&str>>();

        if parts.len() != 2 {
            return Err("Bid string malformed".to_string());
        }

        let hand: HandType = HandType::from_str_with_joker(parts[0])?;
        let ammount: usize = parts[1]
            .parse()
            .map_err(|_| "Invalid bid ammount".to_string())?;

        Ok(Bid(hand, ammount))
    }
    fn partial_cmp_with_joker(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp_with_joker(&other.0)
    }
}

impl std::cmp::PartialOrd for Bid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl FromStr for Bid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split_whitespace()
            .map(|p| p.trim())
            .collect::<Vec<&str>>();

        if parts.len() != 2 {
            return Err("Bid string malformed".to_string());
        }

        let hand: HandType = parts[0].parse()?;
        let ammount: usize = parts[1]
            .parse()
            .map_err(|_| "Invalid bid ammount".to_string())?;

        Ok(Bid(hand, ammount))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide a file name");
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.lines().map(|line| line.trim()).collect();

    let mut bids: Vec<Bid> = lines.iter().map(|line| line.parse().unwrap()).collect();
    bids.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let result: usize = bids
        .iter()
        .enumerate()
        .map(|(i, bid)| bid.1 * (i + 1))
        .sum();

    println!("Result 1: {}", result);

    let mut bids: Vec<Bid> = lines
        .iter()
        .map(|line| Bid::from_str_with_joker(line).unwrap())
        .collect();

    bids.sort_by(|a, b| b.partial_cmp_with_joker(a).unwrap());

    let result: usize = bids
        .iter()
        .enumerate()
        .map(|(i, bid)| bid.1 * (i + 1))
        .sum();

    println!("Result 2: {}", result);
}
