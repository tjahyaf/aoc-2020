use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines = io::BufReader::new(File::open("./input.txt").unwrap()).lines().flatten();
    
    let mut parsing = false;
    let mut attrs = 0;
    let mut valid = 0;

    for line in lines {
        if line.len() != 0 {
            parsing = true;
            attrs += line.split(' ').filter(|s| is_valid_attr(s)).count();
        } else if parsing {
            if attrs == 7 {
                valid += 1;
            }

            attrs = 0;
            parsing = false;
        }
    }

    if parsing && attrs == 7 {
        valid += 1;
    }

    println!("{}", valid);
}

static COLORS : [&str;7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]; 

fn is_valid_attr(attr: &str) -> bool {
    let sep = attr.find(':').unwrap();
    let key = &attr[0..sep];
    let val = &attr[(sep+1)..];

    match key {
        "byr" => val.parse().map_or(false, |yr| 1920 <= yr && yr <= 2002),
        "iyr" => val.parse().map_or(false, |yr| 2010 <= yr && yr <= 2020),
        "eyr" => val.parse().map_or(false, |yr| 2020 <= yr && yr <= 2030),
        "hgt" if val.ends_with("cm") =>
            val[0..(val.len() - 2)].parse().map_or(false, |h| 150 <= h && h <= 193),
        "hgt" if val.ends_with("in") =>
            val[0..(val.len() - 2)].parse().map_or(false, |h| 59 <= h && h <= 76),
        "hcl" => val.starts_with("#") && val.len() == 7 && val[1..].is_alphanumeric(),
        "ecl" => COLORS.contains(&val),
        "pid" => val.len() == 9 && val.is_digit(),
        _ => false
    }
}

trait AlphaNum {
    fn is_alphanumeric(&self) -> bool;
    fn is_digit(&self) -> bool;
}

impl AlphaNum for str {
    fn is_digit(&self) -> bool {
        self.as_bytes().iter().all(|c| (b'0' <= *c && *c <= b'9'))
    }

    fn is_alphanumeric(&self) -> bool {
        self.as_bytes().iter().all(|c| (b'0' <= *c && *c <= b'9') || (b'A' <= *c && *c <= b'Z') || (b'a' <= *c && *c <= b'z'))
    }
}