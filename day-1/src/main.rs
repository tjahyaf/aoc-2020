use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("./input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut lines = lines.map(|l| {
        l.unwrap().parse::<i32>().unwrap()
    }).collect::<Vec<_>>();

    lines.sort_unstable();

    let pair = find_pair_sum(&lines, 2020);
    if let Some((first, second)) = pair {
        println!("{} * {} = {}", first, second, first * second);
    } else {
        println!("Pair not found!");
    }

    let triplet = find_triplet_sum(&lines, 2020);
    if let Some((first, second, third)) = triplet {
        println!("{} * {} * {} = {}", first, second, third, first * second * third);
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
    let mut third = lines.len() - 1;

    for i in 0..(third-1) {
        let pair_sum = sum - lines[i];
        let second = i + 1;
        while third > second && lines[third] >= pair_sum {
            third -= 1;
        }

        if let Some((second,third)) = find_pair_sum(&lines[second..=third], pair_sum) {
            return Some((lines[i], second, third));
        }
    }

    None
}