use std::{collections::HashMap, fs, io::{self, BufRead}};

fn main() {
    // let lines = io::BufReader::new(fs::File::open("./input.txt").unwrap())
    //     .lines()
    //     .flatten();
    // let (grammar, inputs) = parse(lines);

    // let num = part_one(&grammar, &inputs);
    // println!("{}", num);

    let lines = io::BufReader::new(fs::File::open("./input2.txt").unwrap())
        .lines()
        .flatten();
    let (grammar, inputs) = parse(lines);

    let num = part_one(&grammar, &inputs);
    println!("{}", num);
}

fn parse(mut lines: impl Iterator<Item=String>) -> (Grammar, Vec<String>) {
    let rules = lines.by_ref().take_while(|l| !l.is_empty()).collect::<Vec<_>>();

    let grammar = parse_grammar(&rules);

    (grammar, lines.skip(1).collect())
}

fn parse_grammar(rules: &[String]) -> Grammar {
    let grammar = rules.iter().map(|line| {
        let mut parts = line.split(": ");
        let id = parts.next().unwrap().parse::<usize>().unwrap();
        let content = parts.next().unwrap();

        let rule = if content.starts_with('"') {
            Rule::Terminal(content.trim_matches('"').to_owned())
        } else if content.contains('|') {
            let mut parts = content.split(" | ");
            let rule1 = parse_expansion_rule(parts.next().unwrap());
            let rule2 = parse_expansion_rule(parts.next().unwrap());
            
            Rule::Choice(Box::new(rule1), Box::new(rule2))
        } else {
            parse_expansion_rule(content)
        };

        (id, rule)
    })
    .collect::<HashMap<usize, Rule>>();

    Grammar(grammar)
}

fn parse_expansion_rule(content: &str) -> Rule {
    let ids = content.split(" ").map(|s| {
        s.parse::<usize>().unwrap()
    }).collect::<Vec<_>>();

    Rule::Expansion(ids)
}

#[allow(dead_code)]
fn part_one(grammar: &Grammar, inputs: &[String]) -> usize {
    let parser = RuleParser(grammar);

    inputs
        .iter()
        .filter(|l| {
            parser.follows_rule(l, 0)
        })
        .count()
}

#[derive(Debug, Clone)]
enum Rule {
    Terminal(String),
    Expansion(Vec<usize>),
    Choice(Box<Rule>, Box<Rule>)
}

struct Grammar(HashMap<usize, Rule>);

impl Grammar {
    fn get(&self, id: usize) -> &Rule {
        self.0.get(&id).unwrap()
    }
}

struct ParsedCache(HashMap<(usize, usize, usize), bool>);

impl ParsedCache {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn set_result(&mut self, start: usize, end: usize, rule: usize, result: bool) {
        self.0.insert((start, end, rule), result);
    }

    fn get_result(&self, start: usize, end: usize, rule: usize) -> Option<bool> {
        self.0.get(&(start, end, rule)).cloned()
    }
}
struct RuleParser<'a>(&'a Grammar);

impl<'a> RuleParser<'a> {
    fn follows_rule(&self, input: &str, id: usize) -> bool {
        let mut parsed = ParsedCache::new();

        self.check_recursive(input, 0, input.len()-1, id, &mut parsed)
    }

    fn check_recursive(&self, input: &str, start: usize, end: usize, id: usize, mut parsed: &mut ParsedCache) -> bool {
        if let Some(status) = parsed.get_result(start, end, id) {
            return status;
        }

        let rule = self.0.get(id);
        let valid = match rule {
            Rule::Choice(one, two) => {
                self.process_rule(input, start, end, &one, &mut parsed) ||
                self.process_rule(input, start, end, &two, &mut parsed)
            },
            _ => {
                self.process_rule(input, start, end, rule, parsed)
            }
        };

        parsed.set_result(start, end, id, valid);

        valid
    }

    fn process_rule(&self, input: &str, start: usize, end: usize, rule: &Rule, mut parsed: &mut ParsedCache) -> bool {
        match rule {
            Rule::Terminal(c) => {
                input[start..=end] == *c
            },
            Rule::Expansion(ids) => {
                match ids.len() {
                    1 => self.check_recursive(input, start, end, ids[0], parsed),
                    2 => end-start+1 >= ids.len() && (start..=end-1).any(|k| {
                        self.check_recursive(input, start, k, ids[0], &mut parsed) &&
                        self.check_recursive(input, k+1, end, ids[1], &mut parsed)
                    }),
                    _ => end-start+1 >= ids.len() && (start..=end-1).any(|k| {
                        let rest = Rule::Expansion(ids[1..].to_owned());
                        self.check_recursive(input, start, k, ids[0], &mut parsed) &&
                        self.process_rule(input, k+1, end, &rest, &mut parsed)
                    })
                }
            },
            _ => panic!("Shouldn't happen")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar() {
        let grammar = parse_grammar(&[
            "0: 4 1 5".into(),
            "1: 2 3 | 3 2".into(),
            "2: 4 4 | 5 5".into(),
            "3: 4 5 | 5 4".into(),
            "4: \"a\"".into(),
            "5: \"b\"".into()
        ]);
        let parser = RuleParser(&grammar);
        
        assert!(parser.follows_rule("ababbb", 0));
    }
}