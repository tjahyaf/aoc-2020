use std::fs;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Cmd {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn main() {
    let commands: Vec<Cmd> = io::BufReader::new(fs::File::open("./input.txt").unwrap())
        .lines()
        .flatten()
        .map(parse_cmd)
        .collect();

    let accum = part_one(&commands);
    println!("{}", accum);

    let accum = part_two(&commands);
    println!("{}", accum);
}

fn parse_cmd(line: String) -> Cmd {
    match &line[0..3] {
        "acc" => Cmd::Acc(parse_number(&line)),
        "jmp" => Cmd::Jmp(parse_number(&line)),
        _ => Cmd::Nop(parse_number(&line)),
    }
}

fn parse_number(line: &str) -> i32 {
    let num = line[5..].parse().unwrap();
    if &line[4..5] == "-" {
        num * -1
    } else {
        num
    }
}

fn part_one(commands: &[Cmd]) -> i32 {
    let mut reg = Register { accum: 0, next: 0 };
    let mut ran = vec![false; commands.len()];

    while reg.next < commands.len() && !ran[reg.next] {
        ran[reg.next] = true;
        reg = reg.exec(&commands[reg.next]);
    }

    reg.accum
}

fn part_two(commands: &[Cmd]) -> i32 {
    let mut reg = Register { accum: 0, next: 0 };
    let mut ran = vec![false; commands.len()];
    let mut history = Vec::<Register>::with_capacity(commands.len());

    reg = execute_all(reg, commands, &mut ran, &mut history);

    let mut last_rewind = history.len();

    while reg.next < commands.len() {
        // Rewind
        let rewind_to = history
            .iter()
            .enumerate()
            .take(last_rewind)
            .rev()
            .find(|(_, reg)| match commands[reg.next] {
                Cmd::Nop(_) | Cmd::Jmp(_) => true,
                _ => false,
            })
            .map(|(pos, _)| pos)
            .expect("Unsolvable infinite loop");

        for reg in history.iter().skip(rewind_to+1) {
            ran[reg.next] = false;
        }

        history.truncate(rewind_to+1);
        last_rewind = rewind_to;

        // Rewrite history
        let prev_cmd = history[rewind_to].next;
        let flip_command = match commands[prev_cmd] {
            Cmd::Nop(num) => Cmd::Jmp(num),
            Cmd::Jmp(num) => Cmd::Nop(num),
            _ => panic!("Cannot flip command"),
        };
        reg = history[rewind_to].exec(&flip_command);

        // Onward
        reg = execute_all(reg, commands, &mut ran, &mut history);
    }

    reg.accum
}

fn execute_all(
    mut reg: Register,
    commands: &[Cmd],
    ran: &mut [bool],
    history: &mut Vec<Register>,
) -> Register {
    while reg.next < commands.len() && !ran[reg.next] {
        history.push(reg);
        ran[reg.next] = true;
        reg = reg.exec(&commands[reg.next]);
    }

    reg
}

#[derive(Clone, Copy, Debug)]
struct Register {
    accum: i32,
    next: usize,
}

impl Register {
    fn exec(&self, command: &Cmd) -> Self {
        match command {
            Cmd::Acc(num) => Self {
                accum: self.accum + num,
                next: self.next + 1,
            },
            Cmd::Jmp(num) => Self {
                accum: self.accum,
                next: (self.next as i32 + num) as usize,
            },
            Cmd::Nop(_) => Self {
                accum: self.accum,
                next: self.next + 1,
            },
        }
    }
}
