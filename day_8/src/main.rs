use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug)]
struct BinaryNode {
    left: String,
    right: String,
}

impl FromStr for BinaryNode {
    type Err = ();

    /// Parses a string of the form "(BBB, CCC)" into a BinaryNode
    /// (BBB, CCC)
    /// (DDD, EEE)
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s[1..s.len() - 1].split(',');

        let left = parts.next().unwrap().trim();
        let right = parts.next().unwrap().trim();

        Ok(BinaryNode {
            left: left.to_string(),
            right: right.to_string(),
        })
    }
}

type NodeMap = HashMap<String, BinaryNode>;

/// Parses a string of the form "AAA = (BBB CCC)" into a NodeMap entry
fn entry_from_str(map: &mut NodeMap, s: &str) -> String {
    let mut parts = s.split(" = ");
    let key = parts.next().unwrap().trim().to_string();
    let value = parts.next().unwrap().trim();

    let node = BinaryNode::from_str(value).unwrap();

    map.insert(key.clone(), node);

    key
}

fn get_path_steps<IsTargetFn>(
    nodes: &NodeMap,
    directions: &[Direction],
    start: &str,
    is_target: IsTargetFn,
) -> usize
where
    IsTargetFn: Fn(&str) -> bool,
{
    let mut current_key = start;
    let mut count: usize = 0;

    for direction in directions.iter().cycle() {
        let node = nodes.get(current_key).unwrap();

        match direction {
            Direction::Left => {
                current_key = &node.left;
            }
            Direction::Right => {
                current_key = &node.right;
            }
        }

        count += 1;

        if is_target(current_key) {
            break;
        }
    }

    count
}

fn greatest_common_divisor(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

fn least_common_multiple(a: usize, b: usize) -> usize {
    (a * b) / greatest_common_divisor(a, b)
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

    let mut keys: Vec<String> = Vec::new();
    let mut nodes: HashMap<String, BinaryNode> = HashMap::new();

    let directions: Vec<Direction> = lines[0].chars().map(Direction::from).collect();

    for line in lines[1..].iter() {
        let key = entry_from_str(&mut nodes, line);
        keys.push(key);
    }

    let start_key = "AAA".to_string();
    let target_key = "ZZZ".to_string();

    let count: usize = get_path_steps(&nodes, &directions, &start_key, |key| key == target_key);

    println!("Part 1: {}", count);

    let start_keys: Vec<&String> = keys.iter().filter(|key| key.ends_with('A')).collect();

    // Get each unique path's step count. Each path may have a different step count.
    let steps_list: Vec<usize> = start_keys
        .iter()
        .map(|key| get_path_steps(&nodes, &directions, key, |key| key.ends_with('Z')))
        .collect();

    // Find when all paths would join together.
    // > Path of size 2: ++--++--++
    // > Path of size 5: +++++-----
    // The two paths join for the first time at 10, which is the least common multiple
    let count = steps_list.into_iter().reduce(least_common_multiple);

    println!("Part 2: {}", count.unwrap());
}
