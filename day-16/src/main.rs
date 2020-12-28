use std::{collections::HashMap, fs};
use std::{
    collections::HashSet,
    io::{self, BufRead},
};

#[allow(unused_variables)]
fn main() {
    let lines = io::BufReader::new(fs::File::open("./input.txt").unwrap())
        .lines()
        .flatten();
    let (ranges, mine, others) = parse(lines);

    let num = part_one(&ranges, &others);
    println!("{}", num);

    let num = part_two(&ranges, &mine, &others);
    println!("{}", num);
}

fn parse(
    mut lines: impl Iterator<Item = String>,
) -> (
    HashMap<String, Vec<(usize, usize)>>,
    Vec<usize>,
    Vec<Vec<usize>>,
) {
    let mut fields: HashMap<String, Vec<(usize, usize)>> = HashMap::new();
    let mut mine: Vec<usize> = vec![];
    let mut others: Vec<Vec<usize>> = vec![];

    // parse fields
    for line in lines.by_ref().take_while(|l| !l.is_empty()) {
        let mut parts = line.split(": ");
        let name = parts.next().unwrap();
        let ranges = parts
            .next()
            .unwrap()
            .split(" or ")
            .map(|r| {
                let mut limits = r.split('-');
                (
                    limits.next().unwrap().parse::<usize>().unwrap(),
                    limits.next().unwrap().parse::<usize>().unwrap(),
                )
            })
            .collect::<Vec<_>>();

        fields.insert(name.to_owned(), ranges);
    }

    mine.extend(
        lines
            .by_ref()
            .skip_while(|l| *l != "your ticket:")
            .skip(1)
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse::<usize>().unwrap()),
    );

    for line in lines.skip_while(|l| *l != "nearby tickets:").skip(1) {
        others.push(
            line.split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>(),
        );
    }

    (fields, mine, others)
}

fn part_one(ranges: &HashMap<String, Vec<(usize, usize)>>, others: &[Vec<usize>]) -> usize {
    let merged_ranges = merge_ranges(&ranges);

    others
        .iter()
        .map(|ticket| get_error_rate(&merged_ranges, ticket))
        .sum()
}

fn part_two(
    ranges: &HashMap<String, Vec<(usize, usize)>>,
    mine: &Vec<usize>,
    others: &[Vec<usize>],
) -> usize {
    let merged_ranges = merge_ranges(&ranges);

    let mut guesses: Vec<HashSet<&str>> = vec![HashSet::new(); mine.len()];

    guess_fields(ranges, mine)
        .iter()
        .zip(&mut guesses)
        .for_each(|(guess, current)| {
            current.extend(guess);
        });

    for ticket in others
        .iter()
        .filter(|ticket| get_error_rate(&merged_ranges, ticket) == 0)
    {
        guess_fields(ranges, ticket)
            .iter()
            .zip(&mut guesses)
            .for_each(|(guess, current)| {
                let intersect: Vec<&str> = guess
                    .iter()
                    .filter_map(|field| {
                        if current.contains(*field) {
                            Some(*field)
                        } else {
                            None
                        }
                    })
                    .collect();

                current.clear();
                current.extend(intersect);
            });
    }

    deduce_fields(&mut guesses)
        .iter()
        .zip(mine)
        .filter(|(field, _)| field.starts_with("departure"))
        .fold(1, |total, (_, num)| total * num)
}

fn merge_ranges(ranges: &HashMap<String, Vec<(usize, usize)>>) -> Vec<(usize, usize)> {
    let mut ranges = ranges
        .values()
        .flatten()
        .cloned()
        .collect::<Vec<(usize, usize)>>();

    ranges.sort_unstable_by(|l, r| l.0.cmp(&r.0));

    ranges.iter().fold(vec![], |mut list, (start, end)| {
        match list.last_mut() {
            Some((_, prev_end)) => {
                if *start > *prev_end {
                    list.push((start.to_owned(), end.to_owned()));
                } else {
                    *prev_end = *end;
                }
            }
            None => {
                list.push((start.to_owned(), end.to_owned()));
            }
        };

        list
    })
}

fn get_error_rate(merged_ranges: &[(usize, usize)], ticket: &[usize]) -> usize {
    ticket
        .iter()
        .map(
            |num| match merged_ranges.binary_search_by(|(start, _)| start.cmp(num)) {
                Err(pos) if pos == 0 => *num,
                Err(pos) if *num > merged_ranges[pos - 1].1 => *num,
                _ => 0,
            },
        )
        .sum()
}

fn guess_fields<'a>(
    rules: &'a HashMap<String, Vec<(usize, usize)>>,
    ticket: &[usize],
) -> Vec<Vec<&'a str>> {
    ticket
        .iter()
        .map(|num| {
            rules
                .iter()
                .filter(|(_, ranges)| {
                    ranges[0].0 <= *num && *num <= ranges[0].1
                        || ranges[1].0 <= *num && *num <= ranges[1].1
                })
                .map(|(field, _)| field.as_str())
                .collect()
        })
        .collect()
}

fn deduce_fields<'a>(guesses: &'a mut [HashSet<&str>]) -> Vec<&'a str> {
    assert!(guesses.iter().all(|guess| !guess.is_empty()));

    let mut positions: HashSet<usize> = (0..guesses.len()).collect();
    let mut turns = guesses.len() + 1;

    while turns != 0 && !positions.is_empty() {
        let singles: Vec<(usize, &str)> = positions
            .iter()
            .filter_map(|i| {
                if guesses[*i].len() == 1 {
                    let field = guesses[*i].iter().next().unwrap();
                    Some((*i, *field))
                } else {
                    None
                }
            })
            .collect();

        for (pos, _) in singles.iter() {
            positions.remove(&pos);
        }

        for i in positions.iter() {
            let guess = guesses.get_mut(*i).unwrap();
            for (_, field) in singles.iter() {
                guess.remove(*field);
            }
        }

        turns -= 1;
    }

    guesses.iter().flatten().map(|s| *s).collect()
}
