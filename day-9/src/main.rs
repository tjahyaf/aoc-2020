use std::{collections::{HashSet, VecDeque}, fs};
use std::io::{self, BufRead};

const PREAMBLE : usize = 25;

fn main() {
    let lines = io::BufReader::new(fs::File::open("./input.txt").unwrap())
        .lines()
        .flatten()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<u64>>();

    let num = part_one(&lines).unwrap();
    println!("{:?}", num);

    let num = part_two(&lines, num);
    println!("{}", num);
}

#[derive(Debug)]
struct Sums {
    num: u64,
    sums: HashSet<u64>
}

fn part_one(lines: &[u64]) -> Option<u64> {
    let mut sums = VecDeque::<Sums>::with_capacity(PREAMBLE);

    for i in lines.iter().take(PREAMBLE) {
        for ns in sums.iter_mut() {
            ns.sums.insert(ns.num + i);
        }

        sums.push_back(Sums{ num: *i, sums: HashSet::with_capacity(PREAMBLE) });
    }

    for i in lines.iter().skip(PREAMBLE) {
        if ! sums.iter().any(|ns| ns.sums.contains(i)) {
            return Some(*i);
        }

        // Recycle the node
        let mut oldest = sums.pop_front().unwrap();
        oldest.num = *i;
        oldest.sums.clear();

        for ns in sums.iter_mut() {
            ns.sums.insert(ns.num + i);
        }

        sums.push_back(oldest);
    }

    None
}

fn part_two(lines: &[u64], want: u64) -> u64 {
    let mut start = 0usize;
    let mut end = 0usize;
    let mut sum = lines[start];

    while sum != want {
        while sum < want {
            end += 1;
            sum += lines[end];
        }

        while sum > want {
            sum -= lines[start];
            start += 1;
        }

        if start == end {
            panic!("Not found");
        }
    }

    lines[start..=end].iter().min().unwrap() + lines[start..=end].iter().max().unwrap()
}