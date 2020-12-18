use std::collections::{HashMap, HashSet};
use regex::Regex;

fn main() {
    let text = include_str!("../input.txt");

    let count = part_one(text, "shiny gold");
    println!("{}", count);

    let count = part_two(text, "shiny gold");
    println!("{}", count);
}

fn reverse_graph(text: & str) -> HashMap<&str, Vec<&str>> {
    let re = Regex::new(r"\d+ (.*?) bags?\b").unwrap();

    text.split("\n").fold(HashMap::new(), |mut hash, line| {
        let mut rule = line.split(" contain ");
        let parent = rule.next().unwrap().strip_suffix(" bags").unwrap();

        for cap in re.captures_iter(rule.next().unwrap()) {
            let color = cap.get(1).unwrap().as_str();
            match hash.get_mut(color) {
                Some(list) => { list.push(parent); },
                None => { hash.insert(color, vec![parent]); }
            }
        }

        hash
    })
}

fn forward_graph(text: & str) -> HashMap<&str, Vec<(u32, &str)>> {
    let re = Regex::new(r"(\d+) (.*?) bags?\b").unwrap();

    text.split("\n").fold(HashMap::new(), |mut hash, line| {
        let mut rule = line.split(" contain ");
        let parent = rule.next().unwrap().strip_suffix(" bags").unwrap();

        hash.insert(parent, re.captures_iter(rule.next().unwrap()).map(|cap| {
            let color = cap.get(2).unwrap().as_str();
            let count = cap.get(1).unwrap().as_str().parse().unwrap();
            (count, color)
        }).collect());

        hash
    })
}

fn part_one(text: &str, want: &str) -> usize {
    let graph = reverse_graph(text);
    let mut bags = HashSet::<&str>::new();
    let mut stack = vec![want];

    while !stack.is_empty() {
        let bag = stack.pop().unwrap();

        if let Some(list) = graph.get(bag) {
            stack.extend(list.iter().filter(|b| !bags.contains(**b)));
            bags.extend(list.iter().map(|b| *b));
        }
    }

    bags.len()
}

fn part_two(text: &str, want: &str) -> u64 {
    let graph = forward_graph(text);

    count_subtree(&graph, want) - 1
}

fn count_subtree(graph: &HashMap<&str, Vec<(u32, &str)>>, want: &str) -> u64 {
    let bag = graph.get(want).unwrap();

    1 + bag.iter().fold(0u64, |sum, (count, bag)| {
        sum + (*count as u64 * count_subtree(&graph, bag))
    })
}