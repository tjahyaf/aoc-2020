use std::{io::{self, BufRead}, str::FromStr};
use std::{convert::Infallible, fs};

fn main() {
    let lines = io::BufReader::new(fs::File::open("./input.txt").unwrap())
        .lines()
        .flatten()
        .collect::<Vec<_>>();

    let num = part_one(&lines);
    println!("{}", num);

    let num = part_two(&lines);
    println!("{}", num);
}

fn part_one(lines: &[String]) -> usize {
    lines
        .iter()
        .map(|s| {
            let expr = Expression::from_str(s).unwrap();
            evaluate(&expr)
        })
        .sum()
}

fn part_two(lines: &[String]) -> usize {
    lines
        .iter()
        .map(|s| {
            let expr = Expression::from_str(s).unwrap();
            evaluate2(&expr)
        })
        .sum()
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Number(usize),
    OpenBracket,
    CloseBracket,
    Plus,
    Multiply,
}

#[derive(Debug)]
struct Expression(Vec<Token>);

impl FromStr for Expression {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            input
                .split(' ')
                .fold(Vec::<Token>::new(), |mut list, s| {
                    match s {
                        "+" => list.push(Token::Plus),
                        "*" => list.push(Token::Multiply),
                        _ => {
                            if s.starts_with('(') {
                                let trimmed = s.trim_start_matches('(');
                                for _ in 0..s.len() - trimmed.len() {
                                    list.push(Token::OpenBracket);
                                }

                                list.push(Token::Number(trimmed.parse::<usize>().unwrap())); 
                            } else if s.ends_with(')') {
                                let trimmed = s.trim_end_matches(')');
                                list.push(Token::Number(trimmed.parse::<usize>().unwrap()));
                                for _ in 0..s.len() - trimmed.len() {
                                    list.push(Token::CloseBracket);
                                }
                            } else {
                                list.push(Token::Number(s.parse::<usize>().unwrap()));
                            }
                        },
                    };

                    list
                })
        ))
    }
}

fn evaluate(expr: &Expression) -> usize {
    let mut values = Vec::<usize>::new();
    let mut operators = Vec::<&Token>::new();

    for token in expr.0.iter() {
        match token {
            Token::OpenBracket | Token::Plus | Token::Multiply => { operators.push(token); },
            Token::Number(num) => { 
                values.push(*num);
                calculate_once(&mut values, &mut operators); 
            },
            Token::CloseBracket => {
                operators.pop();    // pop open bracket
                calculate_once(&mut values, &mut operators)
            }
        }
    }

    values.pop().unwrap()
}

fn evaluate2(expr: &Expression) -> usize {
    let mut values = Vec::<usize>::new();
    let mut operators = Vec::<&Token>::new();

    for token in expr.0.iter() {
        match token {
            Token::OpenBracket | Token::Plus => { operators.push(token); },
            Token::Number(num) => { values.push(*num); },
            Token::Multiply => {
                while let Some(Token::Plus) = operators.last() {
                    calculate_once(&mut values, &mut operators);
                }

                operators.push(token);
            },
            Token::CloseBracket => {
                loop {
                    match operators.last() {
                        Some(Token::OpenBracket) => {
                            operators.pop();
                            break;
                        },
                        Some(_) => {
                            calculate_once(&mut values, &mut operators);
                        },
                        None => { panic!("Uneven open bracket"); }
                    }
                }
            }
        }
    }

    while !operators.is_empty() {
        calculate_once(&mut values, &mut operators);
    }

    values.pop().unwrap()
}

fn calculate_once(values: &mut Vec<usize>, operators: &mut Vec<&Token>) {
    let num = values.pop().unwrap();

    match operators.last() {
        Some(Token::OpenBracket) => values.push(num),
        Some(Token::Plus) => {
            operators.pop();
            let left_hand = values.pop().unwrap();
            values.push(left_hand + num);
        },
        Some(Token::Multiply) => {
            operators.pop();
            let left_hand = values.pop().unwrap();
            values.push(left_hand * num);
        },
        None => values.push(num),
        Some(_) => { panic!("Shouldn't have close bracket!"); },
    };
}