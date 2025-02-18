use std::cmp::max;
use std::cmp::min;
use std::env;
use std::fs::read_to_string;
use std::iter::repeat;

enum Instruction {
    ADDX { cycles: u32, value: i64 },
    NOOP { cycles: u32 },
}

impl Instruction {
    fn from_string(s: &str) -> Self {
        if s.contains("addx") {
            let value = s.split(" ").collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap();
            return Instruction::ADDX { cycles: 2, value };
        } else {
            return Instruction::NOOP { cycles: 1 };
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input = read_to_string(path).unwrap();
    let mut part_1 = 0;
    let mut part_2: Vec<&str> = repeat(".").take(240).collect();

    let mut tape: Vec<Instruction> = input
        .lines()
        .map(|line| Instruction::from_string(line))
        .rev()
        .collect();

    // always dec
    let mut clock = 0;
    // always inc
    let mut wait = 0;
    // X
    let mut register: i64 = 1;
    // Placeholder
    let mut tmp: i64 = 0;

    let cycles: Vec<u32> = vec![20, 60, 100, 140, 180, 220];

    loop {
        if wait == 0 {
            // start / end what's the difference really
            register += tmp;
            if let Some(op) = tape.pop() {
                match op {
                    Instruction::ADDX { cycles, value } => {
                        // println!("cycles={} value={}", cycles, value);
                        wait = cycles;
                        tmp = value;
                    }
                    Instruction::NOOP { cycles } => {
                        wait = cycles;
                        tmp = 0;
                    }
                }
            } else {
                break;
            }
        }

        // drawing for part 2
        let l = max(0, register-1);
        let r = min(239, register+1);
        let row = (clock / 40) as usize;
        // use inclusive range
        let position = (clock as i64).rem_euclid(40);
        // println!("clock={} row={} pos={} l={} r={} register={}", clock, row, position, l, r, register);
        if (l..=r).contains(&position) {
            if register > 0 {
                let idx = (position as usize) + (row * 40);
                part_2[idx] = "#";
            }
        }

        // part 1
        wait -= 1;
        clock += 1;
        if cycles.contains(&clock) {
            part_1 += register * clock as i64;
        }
    }

    println!("part 1: {}", part_1);
    println!("part 2:");
    let _: Vec<_> = part_2.chunks(40).map(|c| {
        println!("{}", c.join(""));
    }).collect();
}   
