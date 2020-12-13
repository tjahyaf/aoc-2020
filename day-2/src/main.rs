use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct PasswordEntry {
    num1: usize,
    num2: usize,
    letter: u8,
    password: Vec<u8>
}

impl From<String> for PasswordEntry {
    fn from(line: String) -> Self {
        let mut parts = line.split(" ");

        let range = parts.next().unwrap().split('-').map(|f| f.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let letter = parts.next().unwrap().as_bytes()[0];
        let password = parts.next().unwrap();

        Self {
            num1: range[0],
            num2: range[1],
            letter: letter,
            password: password.as_bytes().into()
        }
    }
}

fn main() {
    let file = File::open("./input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let entries = lines.map(|l| l.unwrap().into()).collect::<Vec<_>>();

    let correct = entries.iter().filter(|e| is_password1(e)).count();
    println!("{}", correct);

    let correct = entries.iter().filter(|e| is_password2(e)).count();
    println!("{}", correct);
}

fn is_password1(entry: &PasswordEntry) -> bool {
    let count = entry.password.iter().filter(|c| **c == entry.letter).count();

    entry.num1 <= count && count <= entry.num2
}

fn is_password2(entry: &PasswordEntry) -> bool {
    let first = entry.password.get(entry.num1 - 1).map_or(false, |c| *c == entry.letter);
    let second = entry.password.get(entry.num2 - 1).map_or(false, |c| *c == entry.letter);

    first ^ second
}