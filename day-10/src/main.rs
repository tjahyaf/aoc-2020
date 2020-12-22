use std::fs;
use std::io::{self, BufRead};

fn main() {
    let mut lines = io::BufReader::new(fs::File::open("./input.txt").unwrap())
        .lines()
        .flatten()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<usize>>();

    lines.sort_unstable();

    let num = part_one(&lines);
    println!("{}", num);

    let num = part_two(&lines);
    println!("{}", num);
}

fn part_one(adaptors: &[usize]) -> usize {
    let mut diff1 = 0usize;
    let mut diff3 = 1usize;
    let mut prev = 0usize; 

    for i in adaptors {
        match i - prev {
            1 => {
                diff1 += 1;
            },
            3 => {
                diff3 += 1;
            }
            _ => ()
        }

        prev = *i;
    }

    diff1 * diff3
}

fn part_two(adaptors: &[usize]) -> usize {
    let mut total = 1usize;
    let mut start = 0usize;
    let mut end = 0usize;
    let mut highest = 0usize;

    while start < adaptors.len() {
        // Find the next batch of diff up to 3
        while end < adaptors.len() && adaptors[end] <= highest + 3 {
            end += 1;
        }

        // Number of possible combination depends on number of items
        let combo = match end - start {
            1 => 1,
            2 => 3,
            3 => 7,
            _ => panic!("Diff cannot be more than 3")
        };

        // Number of invalid combinations depend on the gap with next batch
        let gap = if end == adaptors.len() {
            3
        } else {
            adaptors[end] - adaptors[end-1]
        };

        let invalid = match combo {
            3 => match gap {
                3 => 1,
                _ => 0
            },
            7 => match gap {
                3 => 3,
                2 => 1,
                _ => 0
            },
            _ => 0
        };

        total *= combo - invalid;
        start = end;
        highest = adaptors[end - 1];
    }

    total
}