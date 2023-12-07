use rayon::prelude::*;
use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct TransformRange {
    source: usize,
    target: usize,
    length: usize,
}

impl TransformRange {
    fn includes(&self, index: usize) -> bool {
        let source_end = self.source + self.length - 1;
        self.source <= index && index <= source_end
    }

    /// Transforms a number to the defined range if its included in it.
    /// If not, returns the initial number
    fn transform(&self, number: usize) -> usize {
        if self.includes(number) {
            number - self.source + self.target
        } else {
            number
        }
    }
}
impl FromStr for TransformRange {
    type Err = ();

    /// Parse a string like "target source length" into a Range
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 3 {
            return Err(());
        }

        let target = parts[0].parse::<usize>().unwrap();
        let source = parts[1].parse::<usize>().unwrap();
        let length = parts[2].parse::<usize>().unwrap();

        Ok(TransformRange {
            source,
            target,
            length,
        })
    }
}

#[derive(Debug)]
struct Mapper {
    from: String,
    to: String,
    ranges: Vec<TransformRange>,
}

impl Mapper {
    fn transform(&self, number: usize) -> usize {
        let mut result = number;

        for range in &self.ranges {
            if range.includes(result) {
                result = range.transform(result);
                break;
            }
        }
        result
    }
}

impl FromStr for Mapper {
    type Err = ();

    /// Parse a string like:
    /// <form>-to-<to> map:
    /// <range1>
    /// <range2>
    /// ...
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();

        if lines.len() < 2 {
            return Err(());
        }

        let (from, to) = lines[0]
            .split_once(' ')
            .unwrap()
            .0
            .split_once("-to-")
            .unwrap();

        let from = from.to_string();
        let to = to.to_string();

        let ranges: Vec<TransformRange> = lines[1..]
            .iter()
            .map(|line| line.parse::<TransformRange>().unwrap())
            .collect();

        Ok(Mapper { from, to, ranges })
    }
}

fn get_mappers_from_lines(lines: &[&str]) -> Vec<Mapper> {
    lines
        .join("\n")
        .split("\n\n")
        .map(|text| text.trim().parse().unwrap())
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide a file name");
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.lines().map(|line| line.trim()).collect();

    let seeds: Vec<usize> = lines[0]
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(' ')
        .map(|number| number.parse().unwrap())
        .collect();

    let mappers: Vec<Mapper> = get_mappers_from_lines(&lines[1..]);

    let transformed_seeds: Vec<usize> = seeds
        .iter()
        .map(|&seed| {
            // print!("{seed} -> ");
            let mut result: usize = seed;

            for mapper in &mappers {
                result = mapper.transform(result);
                // print!("{result} -> ");
            }

            // println!("{result}");
            result
        })
        .collect();

    let min_seed = transformed_seeds.iter().min().unwrap();
    println!("{min_seed}");

    // PERF: The second answer is really slow
    let seed_ranges = seeds.par_iter().chunks(2).map(|chunk| {
        let start = *chunk[0];
        let length = *chunk[1];
        start..(start + length)
    });

    let min_seed_ranges: usize = seed_ranges
        .map(|seeds| {
            seeds
                .into_par_iter()
                .map(|seed| {
                    let mut result: usize = seed;

                    for mapper in &mappers {
                        result = mapper.transform(result);
                        // print!("{result} -> ");
                    }

                    // println!("{result}");
                    result
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap();

    println!("{min_seed_ranges}");
}
