use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("D:/projects/github/aoc-2020/day-1/src/input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut lines = lines.map(|l| {
        l.unwrap().parse::<i32>().unwrap()
    }).collect::<Vec<_>>();

    lines.sort_unstable();

    let pair = find_pair_sum(&lines, 2020);
    if let Some((first, second)) = pair {
        println!("Pair multiple {}", first * second);
    } else {
        println!("Pair not found!");
    }

    let triplet = find_triplet_sum(&lines, 2020);
    if let Some((first, second, third)) = triplet {
        println!("Triplet multiple {}", first * second * third);
    } else {
        println!("Triplet not found!");
    }
}

fn find_pair_sum(lines: &[i32], sum: i32) -> Option<(i32, i32)> {
    let mut head = 0;
    let mut tail = lines.len() - 1;

    while head < tail {
        match lines[head] + lines[tail] {
            n if n == sum => { return Some((lines[head], lines[tail])); },
            n if n < sum => { head += 1; },
            _ => { tail -= 1 }
        }
    }

    None
}

fn find_triplet_sum(lines: &[i32], sum: i32) -> Option<(i32, i32, i32)> {
    let last = lines.len() - 1;
    let mut pair_range = 2;

    for i in 0..(last - 1) {
        let third = lines[last - i];
        let pair_sum = sum - third;
        while pair_range < last && lines[pair_range] < pair_sum {
            pair_range += 1;
        }

        if let Some((first,second)) = find_pair_sum(&lines[0..pair_range], pair_sum) {
            return Some((first, second, third));
        }
    }

    None
}