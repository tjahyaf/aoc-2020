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

    let _ = adaptors.iter().fold(0usize, |prev, num| {
        match num - prev {
            1 => {
                diff1 += 1;
            },
            3 => {
                diff3 += 1;
            }
            _ => ()
        }

        *num
    });

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

        let items = end - start;    // Number of items in the batch

        // Number of possible combination depends on number of items: 2^items - 1
        let combo = (1 << items) - 1;

        // Number of invalid combinations depend on the gap with next batch
        let gap = if end == adaptors.len() {
            3
        } else {
            adaptors[end] - adaptors[end-1]
        };

        //  number  : 1 2 3 4 5 6 
        //  gap     :       1 2 3
        //  noreach : x x
        let unreachable = match items as isize + gap as isize - 4 {
            num if num > 0 => num as usize,
            _ => 0
        };

        let invalid = (1 << unreachable) - 1;

        total *= combo - invalid;
        start = end;
        highest = adaptors[end - 1];
    }

    total
}