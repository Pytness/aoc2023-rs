use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Game {
    id: usize,
    max_red_seen: usize,
    max_green_seen: usize,
    max_blue_seen: usize,
}

impl FromStr for Game {
    type Err = Box<dyn std::error::Error>;
    /// Parses a string into a Game
    /// The string is in the form "Game #<id>: #n red, #n green, #n blue; #n red, #n green, #n blue;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game = Game {
            id: 0,
            max_red_seen: 0,
            max_green_seen: 0,
            max_blue_seen: 0,
        };

        const GAME_ID_START: usize = 5;
        let game_id_end = s.find(':').unwrap();

        let game_id: usize = s[GAME_ID_START..game_id_end].parse()?;
        game.id = game_id;

        let leaks = s[game_id_end + 1..].split(';').collect::<Vec<&str>>();

        for leak in leaks {
            let leak = leak.trim();
            let leak_parts = leak.split(',').collect::<Vec<&str>>();

            let color_counts =
                leak_parts
                    .iter()
                    .map(|part| part.trim().split(' '))
                    .map(|mut part| {
                        let count = part.next().unwrap().parse::<usize>().unwrap();
                        let color = part.next().unwrap().trim();

                        (color, count)
                    });

            for (color, count) in color_counts {
                match color {
                    "red" => {
                        if count > game.max_red_seen {
                            game.max_red_seen = count;
                        }
                    }
                    "green" => {
                        if count > game.max_green_seen {
                            game.max_green_seen = count;
                        }
                    }
                    "blue" => {
                        if count > game.max_blue_seen {
                            game.max_blue_seen = count;
                        }
                    }
                    _ => {
                        panic!("Unknown color: {}", color);
                    }
                }
            }
        }

        Ok(game)
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

    let mut games: Vec<Game> = lines
        .iter()
        .map(|line| line.parse::<Game>())
        .flatten()
        // .inspect(|game| println!("{:?}", game))
        .collect();

    const MAX_RED: usize = 12;
    const MAX_GREEN: usize = 13;
    const MAX_BLUE: usize = 14;

    let valid_games: Vec<&Game> = games
        .iter()
        .filter(|game| {
            game.max_red_seen <= MAX_RED
                && game.max_green_seen <= MAX_GREEN
                && game.max_blue_seen <= MAX_BLUE
        })
        .collect();

    let id_sum = valid_games.iter().map(|game| game.id).sum::<usize>();
    println!("Valid games: {:?}", id_sum);

    let total_power = games
        .iter()
        .map(|game| game.max_red_seen * game.max_green_seen * game.max_blue_seen)
        .sum::<usize>();

    println!("Total power: {:?}", total_power);
}
