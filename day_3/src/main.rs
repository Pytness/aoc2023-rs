use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
enum TokenValue {
    Number(usize),
    Symbol(char),
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Token {
    value: TokenValue,
    start: usize,
    length: usize,
    line_number: usize,
}

#[derive(Debug)]
struct Node<'a> {
    value: &'a Token,
    children: Vec<&'a Token>,
}

fn abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        return a - b;
    }

    b - a
}

impl Token {
    fn is_within(&self, other: &Token) -> bool {
        let self_end = self.start + self.length;
        let other_end = other.start + other.length;

        if self.start >= other.start && self.start <= other_end {
            return true;
        }

        if self_end >= other.start && self_end <= other_end {
            return true;
        }

        false
    }

    fn is_close(&self, other: &Token) -> bool {
        let self_end = self.start + self.length - 1;
        let other_end = other.start + other.length - 1;

        if abs_diff(self.start, other.start) == 1 {
            return true;
        }
        if abs_diff(self.start, other_end) == 1 {
            return true;
        }
        if abs_diff(self_end, other.start) == 1 {
            return true;
        }
        if abs_diff(self_end, other_end) == 1 {
            return true;
        }

        false
    }
}
fn are_connected(token_a: &Token, token_b: &Token) -> bool {
    // println!("comparing: {:?} {:?}", token_a, token_b);

    abs_diff(token_a.line_number, token_b.line_number) <= 1
        && (token_a.is_within(token_b) || token_b.is_within(token_a) || token_a.is_close(token_b))
}

fn line_to_tokens(line: &str, line_number: usize) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut left: usize = 0;
    let mut right: usize = 0;

    while right < line.len() {
        let c = line.chars().nth(right).unwrap();

        let is_digit = c.is_ascii_digit();

        if is_digit {
            right += 1;
        }

        // Handle numbers at the end of the line
        if !is_digit || right == line.len() {
            if left != right {
                let number = line[left..right].parse::<usize>().unwrap();
                tokens.push(Token {
                    value: TokenValue::Number(number),
                    start: left,
                    length: right - left,
                    line_number,
                });
            }

            if c != '.' && !is_digit {
                tokens.push(Token {
                    value: TokenValue::Symbol(c),
                    start: right,
                    length: 1,
                    line_number,
                });
            }

            right += 1;
            left = right;
        }
    }

    tokens
}

fn filter_symbol_connected_numbers<'a>(tokens: &[&'a Token]) -> Vec<&'a Token> {
    fn are_connected(token_a: &Token, token_b: &Token) -> bool {
        // println!("comparing: {:?} {:?}", token_a, token_b);
        token_a.is_within(token_b) || token_b.is_within(token_a) || token_a.is_close(token_b)
    }

    // Slow brute force solution

    let number_tokens: Vec<&Token> = tokens
        .iter()
        .cloned()
        .filter(|token| matches!(token.value, TokenValue::Number(_)))
        .collect();

    let symbol_tokens: Vec<&Token> = tokens
        .iter()
        .cloned()
        .filter(|token| matches!(token.value, TokenValue::Symbol(_)))
        .collect();

    let connected_tokens: Vec<&Token> = number_tokens
        .into_iter()
        .filter(|number| {
            symbol_tokens
                .iter()
                .any(|symbol| are_connected(number, symbol))
        })
        .collect();

    connected_tokens
}

fn connected_numbers_from_lines(lines: &[Vec<Token>]) -> Vec<Token> {
    let mut connected_tokens: HashSet<&Token> = HashSet::new();

    lines.windows(2).for_each(|lines| {
        let tokens: Vec<&Token> = lines.iter().flatten().collect();

        let local_connected_tokens: Vec<&Token> = filter_symbol_connected_numbers(&tokens);

        for token in local_connected_tokens {
            connected_tokens.insert(token);
        }
    });

    connected_tokens.into_iter().copied().collect()
}

fn connected_numbers_from_lines_with_gear(lines: &[Vec<Token>]) -> usize {
    let mut connected_tokens: HashSet<&Token> = HashSet::new();
    let tokens: Vec<&Token> = lines.iter().flatten().collect();

    let number_tokens: Vec<&Token> = tokens
        .iter()
        .cloned()
        .filter(|token| matches!(token.value, TokenValue::Number(_)))
        .collect();

    let symbol_tokens: Vec<&Token> = tokens
        .iter()
        .cloned()
        .filter(|token| matches!(token.value, TokenValue::Symbol(_)))
        .collect();

    let nodes: Vec<Node> = symbol_tokens
        .iter()
        .map(|symbol| {
            let children: Vec<&Token> = number_tokens
                .iter()
                .cloned()
                .filter(|number| are_connected(number, symbol))
                .filter(|number| connected_tokens.insert(number))
                .collect();

            Node {
                value: symbol,
                children,
            }
        })
        .filter(|node| !node.children.is_empty())
        .collect();

    nodes
        .iter()
        .map(|node| {
            let symbol_token = node.value;
            let mut result: usize = 0;

            if let TokenValue::Symbol(c) = symbol_token.value {
                let value = node.children.iter().map(|token| match token.value {
                    TokenValue::Number(number) => number,
                    _ => 0,
                });

                if c == '*' && node.children.len() == 2 {
                    result = value.product();
                }
            }

            result
        })
        .sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide a file name");
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.lines().map(|line| line.trim()).collect();

    let tokens_by_line: Vec<Vec<Token>> = lines
        .iter()
        .enumerate()
        .map(|(index, line)| line_to_tokens(line, index))
        .collect();

    let connected_tokens_sum: usize = connected_numbers_from_lines(&tokens_by_line)
        .iter()
        .map(|token| match token.value {
            TokenValue::Number(n) => n,
            _ => 0,
        })
        .sum();

    let connected_tokens_sum_with_gear: usize =
        connected_numbers_from_lines_with_gear(&tokens_by_line);

    println!("Total sum: {}", connected_tokens_sum);
    println!("Total sum gear: {}", connected_tokens_sum_with_gear);
}
