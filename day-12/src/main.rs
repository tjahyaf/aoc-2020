use std::{fs};
use std::io::{self, BufRead};

fn main() {
    let lines = io::BufReader::new(fs::File::open("./input.txt").unwrap()).lines().flatten();

    // let dist = part_one(lines);
    // println!("{}", dist);

    let dist = part_two(lines);
    println!("{}", dist);
}

#[allow(dead_code)]
fn part_one(lines: impl Iterator<Item=String>) -> usize {
    let mut ship = Point{x: 0, y: 0};
    let mut dir = Direction::E;

    for steer in lines.map(|s| Steer::from(s)) {
        match steer {
            Steer::N(dist) => { 
                ship.move_facing(Direction::N, dist);
            },
            Steer::E(dist) => {
                ship.move_facing(Direction::E, dist);
            },
            Steer::W(dist) => {
                ship.move_facing(Direction::W, dist);
            },
            Steer::S(dist) => {
                ship.move_facing(Direction::S, dist);
            },
            Steer::F(dist) => {
                ship.move_facing(dir, dist);
            }
            Steer::L(deg) => {
                dir = dir.turn_left(deg);
            },
            Steer::R(deg) => {
                dir = dir.turn_right(deg);
            }
        }
    }

    ship.manhattan_distance()
}

fn part_two(lines: impl Iterator<Item=String>) -> usize {
    let mut ship = Point{ x: 0, y: 0 };
    let mut waypoint = Point{ x: 10, y: 1 };

    for steer in lines.map(|s| Steer::from(s)) {
        match steer {
            Steer::N(dist) => { 
                waypoint.move_facing(Direction::N, dist);
            },
            Steer::E(dist) => {
                waypoint.move_facing(Direction::E, dist);
            },
            Steer::W(dist) => {
                waypoint.move_facing(Direction::W, dist);
            },
            Steer::S(dist) => {
                waypoint.move_facing(Direction::S, dist);
            },
            Steer::F(mult) => {
                ship.move_by(waypoint.x * mult as isize, waypoint.y * mult as isize);
            },
            Steer::L(deg) => {
                waypoint.rotate_left(deg);
            },
            Steer::R(deg) => {
                waypoint.rotate_right(deg);
            }
        }
    }

    ship.manhattan_distance()
}

#[derive(Clone, Copy)]
enum Direction {
    E = 0,
    N,
    W,
    S
}

impl Direction {
    fn turn_left(&self, deg: usize) -> Self {
        let dir = (*self as usize + deg / 90) % 4;
        unsafe { std::mem::transmute(dir as u8) }
    }

    fn turn_right(&self, deg: usize) -> Self {
        let dir = (*self as usize + 4 - deg / 90) % 4;
        unsafe { std::mem::transmute(dir as u8) }
    }
}

enum Steer {
    N(usize),
    S(usize),
    E(usize),
    W(usize),
    L(usize),
    R(usize),
    F(usize)
}

impl From<String> for Steer {
    fn from(line: String) -> Self {
        let (first, dist) = line.split_at(1);
        match first {
            "N" => Steer::N(dist.parse().unwrap()),
            "S" => Steer::S(dist.parse().unwrap()),
            "E" => Steer::E(dist.parse().unwrap()),
            "W" => Steer::W(dist.parse().unwrap()),
            "L" => Steer::L(dist.parse().unwrap()),
            "R" => Steer::R(dist.parse().unwrap()),
            "F" => Steer::F(dist.parse().unwrap()),
            _ => panic!("Unknown direction {}", first)
        }
    }
}

struct Point {
    x: isize,
    y: isize
}

impl Point {
    fn move_facing(&mut self, dir: Direction, dist: usize) {
        match dir {
            Direction::N => { self.y += dist as isize; },
            Direction::S => { self.y -= dist as isize; },
            Direction::E => { self.x += dist as isize; },
            Direction::W => { self.x -= dist as isize; }
        }
    }

    fn move_by(&mut self, x: isize, y: isize) {
        self.x += x;
        self.y += y;
    }

    fn rotate_left(&mut self, deg: usize) {
        for _ in 0..deg/90 {
            let temp = self.x;
            self.x = -self.y;
            self.y = temp;
        }
    }

    fn rotate_right(&mut self, deg: usize) {
        for _ in 0..deg/90 {
            let temp = self.x;
            self.x = self.y;
            self.y = -temp;
        }
    }

    fn manhattan_distance(&self) -> usize {
        self.x.abs() as usize + self.y.abs() as usize
    }
}