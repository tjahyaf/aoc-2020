use std::fs;
use std::io::{self, BufRead};

fn main() {
    let lines = io::BufReader::new(fs::File::open("./input.txt").unwrap())
        .lines()
        .flatten()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let count = count_seats(&lines, part_one);
    println!("{}", count);

    let count = count_seats(&lines, part_two);
    println!("{}", count);
}

fn count_seats(seats: &Vec<Vec<char>>, round: fn(&[Vec<char>], &mut [Vec<char>], &mut [Vec<usize>]) -> bool) -> usize {
    let width = seats[0].len();

    let mut buffer1 = seats.iter().map(|chars| chars.clone()).collect::<Vec<_>>();
    let mut buffer2 = vec![vec!['.'; width]; seats.len()];
    let mut counts = vec![vec![0; width]; seats.len()];

    loop {
        let changed = round(&buffer1, &mut buffer2, &mut counts);

        if !changed {
            break;
        }

        std::mem::swap(&mut buffer1, &mut buffer2);
        
        for row in counts.iter_mut() {
            for i in 0..row.len() {
                row[i] = 0;
            }
        }

        // println!("====================================");
        // print(&buffer1);
    }

    buffer1.iter().flatten().filter(|c| **c == '#').count()
}

#[allow(dead_code)]
fn part_one(seats: &[Vec<char>], mut next: &mut [Vec<char>], counts: &mut [Vec<usize>]) -> bool {
    for i in 0..seats.len() {
        count_left(&seats[i], &seats[i], &mut counts[i]);   // left
        count_right(&seats[i], &seats[i], &mut counts[i]);  // right
    }
    for i in 0..seats.len() - 1 {
        count_left(&seats[i], &seats[i+1], &mut counts[i]); // bottom left
        count_center(&seats[i],&seats[i+1], &mut counts[i]); // bottom
        count_right(&seats[i],&seats[i+1], &mut counts[i]);  // bottom right
    }
    for i in 1..seats.len() {
        count_left(&seats[i],&seats[i-1], &mut counts[i]);  // top left
        count_center(&seats[i],&seats[i-1], &mut counts[i]);    // top
        count_right(&seats[i],&seats[i-1], &mut counts[i]); // top right
    }

    update_seats(&seats, &mut next, &counts, 4)
}

#[allow(dead_code)]
fn part_two(seats: &[Vec<char>], mut next: &mut [Vec<char>], counts: &mut [Vec<usize>]) -> bool {
    for i in 0..seats.len() {
        count_first_left(&seats[i], &seats[i], &mut counts[i]);   // left
        count_first_right(&seats[i], &seats[i], &mut counts[i]);  // right
    }

    let mut row_buffer = seats.first().unwrap().clone();
    for i in 1..seats.len() {
        count_center(&seats[i], &row_buffer, &mut counts[i]);  // top
        merge_seats(&mut row_buffer, &seats[i]);
    }

    row_buffer.copy_from_slice(seats.first().unwrap());
    for i in 1..seats.len() {
        count_right(&seats[i], &row_buffer, &mut counts[i]); // top right
        shift_seats_left(&mut row_buffer);
        merge_seats(&mut row_buffer, &seats[i]);
    }

    row_buffer.copy_from_slice(seats.first().unwrap());
    for i in 1..seats.len() {
        count_left(&seats[i], &row_buffer, &mut counts[i]); // top left
        shift_seats_right(&mut row_buffer);
        merge_seats(&mut row_buffer, &seats[i]);
    }

    row_buffer.copy_from_slice(seats.last().unwrap());
    for i in (0..seats.len() - 1).rev() {
        count_center(&seats[i], &row_buffer, &mut counts[i]); // bottom
        merge_seats(&mut row_buffer, &seats[i]);
    }

    row_buffer.copy_from_slice(seats.last().unwrap());
    for i in (0..seats.len() - 1).rev() {
        count_right(&seats[i], &row_buffer, &mut counts[i]); // bottom right
        shift_seats_left(&mut row_buffer);
        merge_seats(&mut row_buffer, &seats[i]);
    }

    row_buffer.copy_from_slice(seats.last().unwrap());
    for i in (0..seats.len() - 1).rev() {
        count_left(&seats[i], &row_buffer, &mut counts[i]); // bottom left
        shift_seats_right(&mut row_buffer);
        merge_seats(&mut row_buffer, &seats[i]);
    }

    update_seats(&seats, &mut next, &counts, 5)
}

fn update_seats(seats: &[Vec<char>], next: &mut [Vec<char>], counts: &[Vec<usize>], threshold: usize) -> bool {
    let mut changed = false;

    for i in 0..seats.len() {
        for j in 0..seats[i].len() {
            match seats[i][j] {
                '.' => {
                    next[i][j] = '.';
                },
                'L' => {
                    if counts[i][j] == 0 {
                        next[i][j] = '#';
                        changed = true;
                    } else {
                        next[i][j] = 'L';
                    }
                },
                '#' => {
                    if counts[i][j] >= threshold {
                        next[i][j] = 'L';
                        changed = true;
                    } else {
                        next[i][j] = '#';
                    }
                },
                _ => {
                    panic!("Invalid character");
                }
            }
        }
    }

    changed
}

fn merge_seats(target: &mut[char], source: &[char]) {
    for i in 0..target.len() {
        if let '#'|'L' = source[i] {
            target[i] = source[i];
        }
    }
}

fn shift_seats_left(seats: &mut [char]) {
    for i in 0..seats.len() - 1 {
        seats[i] = seats[i+1];
    }

    seats[seats.len() - 1] = '.';
}

fn shift_seats_right(seats: &mut [char]) {
    for i in (1..seats.len()).rev() {
        seats[i] = seats[i-1];
    }

    seats[0] = '.';
}

#[allow(dead_code)]
fn count_first_right(seats: &[char], check: &[char], counts: &mut [usize]) {
    let mut last = check[seats.len()-1];
    for col in (0..seats.len()-1).rev() {
        if let 'L'|'#' = seats[col] {
            if last == '#' {
                counts[col] += 1;
            }
        }

        if check[col] != '.' {
            last = check[col];
        }
    }
}

#[allow(dead_code)]
fn count_first_left(seats: &[char], check: &[char], counts: &mut [usize]) {
    let mut first = check[0];
    for col in 1..seats.len() {
        if let 'L'|'#' = seats[col] {
            if first == '#' {
                counts[col] += 1;
            }
        }

        if check[col] != '.' {
            first = check[col];
        }
    }
}

fn count_right(seats: &[char], check: &[char], counts: &mut [usize]) {
    for col in 0..seats.len() - 1 {
        if let 'L'|'#' = seats[col] {
            if check[col+1] == '#' {
                counts[col] += 1;
            }
        }
    }
}

fn count_left(seats: &[char], check: &[char], counts: &mut [usize]) {
    for col in 1..seats.len() {
        if let 'L'|'#' = seats[col] {
            if check[col-1] == '#' {
                counts[col] += 1;
            }
        }
    }
}

fn count_center(seats: &[char], check: &[char], counts: &mut [usize]) {
    for col in 0..seats.len() {
        if let 'L'|'#' = seats[col] {
            if check[col] == '#' {
                counts[col] += 1;
            }
        }
    }
}

#[allow(dead_code)]
fn print(seats: &[Vec<char>]) {
    for row in seats {
        let s: String = row.iter().collect();
        println!("{}", s);
    }
}