use std::collections::HashSet;
use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Default)]
struct Card {
    id: usize,
    winning_numbers: HashSet<usize>,
    owned_numbers: HashSet<usize>,
}

impl Card {
    fn owned_winning_numbers(&self) -> Vec<usize> {
        self.winning_numbers
            .intersection(&self.owned_numbers)
            .copied()
            .collect()
    }

    fn count_owned_winning_numbers(&self) -> usize {
        self.owned_winning_numbers().len()
    }

    fn calculate_worth(&self) -> usize {
        let winners = self.count_owned_winning_numbers();

        if winners == 0 {
            0
        } else {
            1 << (winners - 1)
        }
    }
}

impl FromStr for Card {
    type Err = Box<dyn std::error::Error>;

    fn from_str(string: &str) -> Result<Card, Self::Err> {
        let (id_slice, numbers_slice) = string.split_once(':').unwrap();

        let id: usize = id_slice.split_once(' ').unwrap().1.trim().parse().unwrap();

        let (winning_numbers_slice, owned_numbers_slice) = numbers_slice.split_once('|').unwrap();

        let winning_numbers: HashSet<usize> = HashSet::from_iter(
            winning_numbers_slice
                .trim()
                .split(' ')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|number_str| number_str.parse::<usize>().unwrap()),
        );
        let owned_numbers: HashSet<usize> = HashSet::from_iter(
            owned_numbers_slice
                .trim()
                .split(' ')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|number_str| number_str.parse::<usize>().unwrap()),
        );

        Ok(Card {
            id,
            winning_numbers,
            owned_numbers,
        })
    }
}

fn cards_win_more_cards(cards: &[Card]) -> usize {
    let mut total_cards = 0;
    let mut cards_count: Vec<usize> = cards.iter().map(|_| 1).collect();

    for (index, card) in cards.iter().enumerate() {
        let winner_count = card.count_owned_winning_numbers();
        let multiplier = cards_count[index];

        let index = index + 1;
        for x in index..(index + winner_count) {
            cards_count[x] += multiplier;
        }
    }

    println!("cards count: {:?}", cards_count);
    cards_count.iter().sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide a file name");
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.lines().map(|line| line.trim()).collect();

    let cards: Vec<Card> = lines.iter().flat_map(|line| line.parse()).collect();

    let total_worth: usize = cards.iter().map(|card| card.calculate_worth()).sum();

    let total_won_cards: usize = cards_win_more_cards(&cards);

    println!("Total worth: {:?}", total_worth);
    println!("Total cards worth: {:?}", total_won_cards);
}
