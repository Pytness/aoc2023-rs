use std::env;
use std::fs;

fn find_highest_hold_time(total_time: usize) -> usize {
    let a: isize = -1;
    let b: isize = total_time as isize;

    (-b / (2 * a)) as usize
}

fn find_hold_time_range(total_time: usize, distance: usize) -> (usize, usize) {
    // distance = (total_time - hold_time) * hold_time
    // distance = total_time * hold_time - hold_time^2

    let a = -1 as f64;
    let b = total_time as f64;
    let c = -(distance as f64);

    let delta = (b * b - 4.0 * a * c).sqrt();

    let x1 = (-b + delta) / (2.0 * a);
    let x2 = (-b - delta) / (2.0 * a);

    println!("x1: {}, x2: {}", x1, x2);
    (x1.ceil() as usize, x2.floor() as usize)
}

fn formula(total_time: usize, hold_time: usize) -> usize {
    (total_time - hold_time) * hold_time
}

fn get_possible_hold_times_count(total_time: usize, distance: usize) -> usize {
    let (min, max) = find_hold_time_range(total_time, distance);

    max - min + 1
}

fn get_winning_solution_from_lines(lines: &[&str]) -> usize {
    let times = lines[0]
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty()) // remove empty strings
        .map(|number| number.parse().unwrap())
        .collect::<Vec<usize>>();

    let distances = lines[1]
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty()) // remove empty strings
        .map(|number| number.parse().unwrap())
        .collect::<Vec<usize>>();

    let winning_solutions = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| get_possible_hold_times_count(*t, *d + 1))
        .collect::<Vec<usize>>();

    winning_solutions.iter().product::<usize>()
}

fn get_winning_solution_from_lines_kerning_corrected(lines: &[&str]) -> usize {
    let time: usize = lines[0]
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap();

    let distance: usize = lines[1]
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap();

    get_possible_hold_times_count(time, distance + 1)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide a file name");
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.lines().map(|line| line.trim()).collect();

    let total_winning_solutions = get_winning_solution_from_lines(&lines);
    let total_winning_solutions_kerning_corrected =
        get_winning_solution_from_lines_kerning_corrected(&lines);

    println!("Total winning solutions: {}", total_winning_solutions);
    println!(
        "Total winning solutions (kerning corrected): {}",
        total_winning_solutions_kerning_corrected
    );
}
