use std::env;
use std::fs;

fn first_spelled_number_or_digit_to_usize(chars: Vec<char>, reversed: bool) -> usize {
    const NUMBERS_BY_INDEX: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    const SHORTEST_NUMBER: usize = 3;
    const LONGEST_NUMBER: usize = 6;

    let chars_len = chars.len();
    let mut result: usize = 0;

    if reversed {
        let mut left: usize = chars_len - 1;
        let mut right: usize = chars_len - 1;

        loop {
            let char = chars[left];

            if char.is_ascii_digit() {
                result = char.to_digit(10).unwrap() as usize;
                break;
            }

            if right - left + 1 < SHORTEST_NUMBER {
                left -= 1;
                continue;
            }

            let word = chars[left..=right].iter().collect::<String>();
            let number = NUMBERS_BY_INDEX
                .iter()
                .enumerate()
                .find(|(_, &number)| word.contains(number));

            if let Some(number) = number {
                result = number.0;
                break;
            }

            left -= 1;

            if right - left > LONGEST_NUMBER {
                right -= 1;
            }
        }
    } else {
        let mut left: usize = 0;
        let mut right: usize = 0;

        loop {
            let char = chars[right];

            if char.is_ascii_digit() {
                result = char.to_digit(10).unwrap() as usize;
                break;
            }

            if right - left + 1 < SHORTEST_NUMBER {
                right += 1;
                continue;
            }

            let word = chars[left..=right].iter().collect::<String>();
            let number = NUMBERS_BY_INDEX
                .iter()
                .enumerate()
                .find(|(_, &number)| word.contains(number));

            if let Some(number) = number {
                result = number.0;
                break;
            }

            right += 1;

            if right - left > LONGEST_NUMBER {
                left += 1;
            }
        }
    }

    result
}
fn calibration_values_with_digits(lines: &[&str]) -> Vec<usize> {
    let calibration_values: Vec<usize> = lines
        .iter()
        .map(|line| {
            let first = line.chars().find(|c| c.is_ascii_digit()).unwrap_or('0');
            let second = line
                .chars()
                .rev()
                .find(|c| c.is_ascii_digit())
                .unwrap_or('0');

            let number = format!("{first}{second}");
            let number: usize = number.parse().unwrap();

            number
        })
        .collect();
    calibration_values
}

fn calibration_values_with_spelling(lines: &[&str]) -> Vec<usize> {
    let calibration_values: Vec<usize> = lines
        .iter()
        .map(|line| {
            let clean_line: Vec<char> = line.chars().collect();

            println!("SEARCHING FIRST");
            let first: usize = first_spelled_number_or_digit_to_usize(clean_line.clone(), false);
            println!("SEARCHING SECOND");
            let second: usize = first_spelled_number_or_digit_to_usize(clean_line.clone(), true);
            println!("first: {}, second: {}", first, second);

            let number = format!("{first}{second}");
            let number: usize = number.parse().unwrap();

            number
        })
        .collect();

    calibration_values
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide a file name");
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.lines().map(|line| line.trim()).collect();

    let calibration_values = calibration_values_with_digits(&lines);
    let calibration_sum: usize = calibration_values.iter().sum();

    println!("Calibration sum: {}", calibration_sum);

    let calibration_values = calibration_values_with_spelling(&lines);
    let calibration_sum: usize = calibration_values.iter().sum();

    println!("Calibration sum: {}", calibration_sum);
}
