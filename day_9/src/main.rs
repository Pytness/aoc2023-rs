use std::env;
use std::fs;

fn extrapolate_next_in_sequence(sequence: &[isize]) -> isize {
    let current_change: Vec<isize> = sequence
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();

    let last = *sequence.last().unwrap();

    if current_change.iter().all(|v| *v == 0) {
        return last;
    }

    extrapolate_next_in_sequence(&current_change) + last
}

fn parse_line(line: &str) -> Vec<isize> {
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide a file name");
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();

    let numbers_list: Vec<Vec<isize>> = lines.into_iter().map(parse_line).collect();

    let result = numbers_list
        .iter()
        .map(|numbers| extrapolate_next_in_sequence(numbers))
        .reduce(|a, b| a + b)
        .unwrap();

    println!("Result 1: {result}");

    let result_inverted = numbers_list
        .iter()
        .map(|numbers| numbers.iter().rev().copied().collect::<Vec<isize>>())
        .map(|numbers| extrapolate_next_in_sequence(&numbers))
        .reduce(|a, b| a + b)
        .unwrap();

    println!("Result 2: {result_inverted}");
}
