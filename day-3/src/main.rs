use std::fs::File;
use std::io::{self, BufRead};

const TREE : u8 = '#' as u8;

fn main() {
    let file = File::open("./input.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let width = lines.next().unwrap().unwrap().len();
    
    let steps = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut trees = vec![0, 0, 0, 0, 0];

    for (i, line) in lines.enumerate() {
        let i = i + 1;
        let bytes = line.as_ref().unwrap().as_bytes();

        for (j, step) in steps.iter().enumerate() {
            if i % step.1 == 0 && bytes[(step.0 * i) % width] == TREE {
                trees[j] += 1;
            }
        }
    }

    println!("{:?}", trees);
    println!("{}", trees.iter().fold(1u64, |a, b| a * *b as u64))
}
