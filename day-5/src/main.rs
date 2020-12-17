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
    let binary = line
        .chars()
        .map(|c| match c {
            'B' | 'R' => b'1',
            _ => b'0',
        })
        .collect::<Vec<_>>();
    let binary = String::from_utf8(binary).unwrap();

    usize::from_str_radix(&binary, 2).unwrap()
}
