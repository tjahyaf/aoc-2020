use std::io::{self, BufRead};
use std::{collections::HashSet, fs};

fn main() {
    let mut lines = io::BufReader::new(fs::File::open("./input.txt").unwrap())
        .lines()
        .flatten()
        .peekable();

    let mut votes = 0;

    while lines.peek().is_some() {
        // votes += part_one(
        //     lines
        //         .by_ref()
        //         .skip_while(|l| l.len() == 0)
        //         .take_while(|l| l.len() > 0),
        // );

        votes += part_two(
            lines
                .by_ref()
                .skip_while(|l| l.len() == 0)
                .take_while(|l| l.len() > 0),
        );
    }

    println!("{}", votes);
}

#[allow(dead_code)]
fn part_one(lines: impl Iterator<Item = String>) -> usize {
    let mut votes = HashSet::<char>::new();

    for line in lines {
        votes.extend(line.chars());
    }

    votes.len()
}

#[allow(dead_code)]
fn part_two(mut lines: impl Iterator<Item = String>) -> usize {
    let mut votes = HashSet::<char>::new();

    if let Some(line) = lines.next() {
        votes.extend(line.chars());
    }

    for line in lines {
        let intersect = line
            .chars()
            .filter(|c| votes.contains(c))
            .collect::<Vec<_>>();
        votes.clear();
        votes.extend(intersect);
    }

    votes.len()
}
