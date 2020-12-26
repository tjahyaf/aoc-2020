use std::{collections::HashMap, convert::Infallible, fmt::Error, fs, str::FromStr};
use std::io::{self, BufRead};

fn main() {
    let insructions = io::BufReader::new(fs::File::open("./input.txt").unwrap())
        .lines()
        .flatten()
        .map(|s| Instruction::from_str(&s))
        .flatten()
        .collect::<Vec<_>>();
    
    let num = part_one(&insructions);
    println!("{}", num);

    let num = part_two(&insructions);
    println!("{}", num);
}


fn part_one(instructions: &[Instruction]) -> u64 {
    let mut mask = Mask { and_bits: !0, or_bits: 0 };
    let mut memory : HashMap<u64, u64> = HashMap::new();

    for instr in instructions {
        match instr {
            Instruction::Mask(bits) => {
                mask = Mask::from_str(bits).unwrap();
            },
            Instruction::Mem(loc, value) => {
                let value = mask.apply(*value);
                memory.insert(*loc, value);
            }
        }
    }

    memory.values().sum()
}

fn part_two(instructions: &[Instruction]) -> u64 {
    let mut mask = Mask2 { or_bits: 0, positions: vec![] };
    let mut memory : HashMap<u64, u64> = HashMap::new();

    for instr in instructions {
        match instr {
            Instruction::Mask(bits) => {
                mask = Mask2::from_str(bits).unwrap();
            },
            Instruction::Mem(loc, value) => {
                let locs = mask.apply(*loc);
                for loc in locs {
                    memory.insert(loc, *value);
                }
            }
        }
    }

    memory.values().sum()
}


enum Instruction {
    Mask(String),
    Mem(u64, u64)
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" = ");
        let instr = parts.next().ok_or(Error)?;
        let val = parts.next().ok_or(Error)?;

        if instr == "mask" {
            Ok(Instruction::Mask(val.into()))
        } else if instr.starts_with("mem") {
            let pos = instr
                .split("[")
                .skip(1)
                .map(|s| s.strip_suffix("]"))
                .flatten()
                .next()
                .ok_or(Error)?;
            Ok(Instruction::Mem(
                pos.parse().or(Err(Error))?,
                val.parse().or(Err(Error))?
            ))
        } else {
            Err(Error)
        }
    }
}

#[derive(Debug)]
struct Mask {
    and_bits: u64,
    or_bits: u64
}

impl Mask {
    fn apply(&self, value: u64) -> u64 {
        value & self.and_bits | self.or_bits
    }
}

impl FromStr for Mask {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut and_bits = 0u64;
        let mut or_bits = 0u64;

        for c in s.chars() {
            and_bits <<= 1;
            or_bits <<= 1;

            match c {
                'X' => and_bits |= 1,
                '1' => or_bits |= 1,
                _ => (),
            }
        }

        Ok(Self {
            and_bits,
            or_bits
        })
    }
}

#[derive(Debug)]
struct Mask2 {
    or_bits: u64,
    positions: Vec<usize>,
}

impl FromStr for Mask2 {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut or_bits = 0u64;
        let mut positions : Vec<usize> = vec![];
        let mut pos = 36usize;

        for c in s.chars() {
            pos = pos.checked_sub(1).unwrap();
            or_bits <<= 1;

            match c {
                'X' => positions.push(pos),
                '1' => or_bits |= 1,
                _ => (),
            }
        }

        Ok(Mask2 {
            or_bits,
            positions
        })
    }
}

impl Mask2 {
    fn apply(&self, memory: u64) -> Vec<u64> {
        let memory = memory | self.or_bits;
        let mut memories = vec![memory; 2usize.pow(self.positions.len() as u32)];
        let mut check_bit = 1usize << (self.positions.len() - 1);

        for pos in self.positions.iter() {
            let and_mask = !0u64 ^ (1 << *pos);
            let or_mask = 0u64 | (1 << *pos);
            
            for (i, mem) in memories.iter_mut().enumerate() {
                if i & check_bit == 0 {
                    *mem &= and_mask;
                } else {
                    *mem |= or_mask;
                }
            }

            check_bit >>= 1;
        }

        memories
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask() {
        let mask = Mask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap();
        println!("{:?}", mask);

        println!("{}", mask.apply(11));
    }

    #[test]
    fn test_mask2() {
        let mask = Mask2::from_str("000000000000000000000000000000X1001X").unwrap();
        println!("{:#?}", mask);

        println!("{:#?}", mask.apply(42));
    }
}