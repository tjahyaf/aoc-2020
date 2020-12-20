use std::fs;
use std::io::{self, BufRead};

fn main() {
    let lines = io::BufReader::new(fs::File::open("./input.txt").unwrap())
        .lines()
        .flatten();

    let mut seats = vec![false;2usize.pow(10)];

    let max = lines.fold(0, |max, l| {
        let id = to_id(&l);
        seats[id] = true;

        if id > max {
            id
        } else {
            max
        }
    });

    println!("Max is {}", max);

    let myseat = seats
        .iter()
        .enumerate()
        .skip_while(|(_, o)| !**o)
        .find(|(_, o)| !**o)
        .unwrap()
        .0;
    println!("My seat is {}", myseat);
}

fn to_id(line: &str) -> usize {
    line
        .chars()
        .map(|c| match c {
            'B' | 'R' => 1usize,
            _ => 0usize,
        })
        .fold(0usize, |id, c| {
            (id << 1) | c
        })
}
