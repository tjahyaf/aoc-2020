use std::fmt;
use std::fs;
use std::io::{self, BufRead};

#[allow(unused_variables)]
fn main() {
    let mut lines = io::BufReader::new(fs::File::open("./input.txt").unwrap())
        .lines()
        .flatten();

    let time: usize = lines.next().unwrap().parse().unwrap();
    let busses = lines.next().unwrap();
    let busses = busses.split(",").collect::<Vec<_>>();

    // let num = part_one(time, &busses);
    // println!("{}", num);

    let num = part_two(&busses);
    println!("{}", num);
}

#[allow(dead_code)]
fn part_one(time: usize, busses: &[&str]) -> usize {
    let (id, wait) = busses
        .iter()
        .filter_map(|bus| {
            if *bus == "x" {
                return None;
            }

            Some(bus.parse::<usize>())
        })
        .map(|bus| {
            let id = bus.unwrap();
            (id, id - time % id)
        })
        .min_by(|l, r| l.1.cmp(&r.1))
        .unwrap();

    id * wait
}

#[allow(unused_variables)]
fn part_two(busses: &[&str]) -> i64 {
    let loops = busses
        .iter()
        .enumerate()
        .filter(|(_, s)| **s != "x")
        .map(|(i, s)| Loop {
            offset: i as i64 * -1,
            duration: s.parse().unwrap(),
        })
        .collect::<Vec<_>>();

    get_sync_time(&loops)
}

#[derive(Clone, Copy)]
struct Loop {
    offset: i64,
    duration: i64,
}

impl fmt::Debug for Loop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}n + {}", self.duration, self.offset))
    }
}

fn find_next_sync(duration1: i64, duration2: i64, offset: i64, completed_loops: i64) -> i64 {
    let mut loop1 = completed_loops;
    let mut sum_duration1 = duration1 * completed_loops;
    let mut sum_duration2 = duration2 as i64;

    while sum_duration1 + offset != sum_duration2 {
        if sum_duration1 + offset < sum_duration2 {
            let mult = (sum_duration2 - sum_duration1 - offset + duration1 - 1) / duration1;
            loop1 += mult;
            sum_duration1 += mult * duration1;
        } else {
            let mult = (sum_duration1 + offset - sum_duration2 + duration2 - 1) / duration2;
            sum_duration2 += mult * duration2;
        }
    }

    loop1
}

fn get_sync_pattern(loop1: &Loop, loop2: &Loop) -> Loop {
    let offset = loop1.offset - loop2.offset;

    let first = find_next_sync(loop1.duration, loop2.duration, offset, 1);
    let second = find_next_sync(loop1.duration, loop2.duration, offset, first + 1);

    Loop {
        offset: first,
        duration: second - first,
    }
}

fn get_sync_time(loops: &[Loop]) -> i64 {
    assert!(loops.len() >= 2);

    if loops.len() == 2 {
        let offset = loops[0].offset - loops[1].offset;
        let cycle = find_next_sync(loops[0].duration, loops[1].duration, offset, 1);
        return cycle * loops[0].duration + loops[0].offset;
    }

    let first = loops[0];
    let sync_patterns = loops
        .iter()
        .skip(1)
        .map(|l| get_sync_pattern(&first, l))
        .collect::<Vec<_>>();

    let cycle = get_sync_time(&sync_patterns);
    first.duration * cycle + first.offset
}
