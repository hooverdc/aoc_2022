use std::cmp::max;
use std::cmp::min;
use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input = read_to_string(path).unwrap();

    let mut part_1 = 0;
    for line in input.lines() {
        let s: Vec<u32> = line
            .split(|c: char| !c.is_ascii_digit())
            .map(|c| c.parse::<u32>().unwrap())
            .take(4)
            .collect();
        let (ls, le, rs, re) = (s[0], s[1], s[2], s[3]);
        if (ls >= rs && le <= re) || (rs >= ls && re <= le) {
            part_1 += 1;
        }
    }

    println!("part 1 {}", part_1);

    let mut part_2 = 0;
    for line in input.lines() {
        let s: Vec<i32> = line
            .split(|c: char| !c.is_ascii_digit())
            .map(|c| c.parse::<i32>().unwrap())
            .take(4)
            .collect();
        let (ls, le, rs, re) = (s[0], s[1], s[2], s[3]);
        // simpler !
        if max(ls, rs) - min(le, re) <= 0 {
            part_2 += 1;
        }
    }

    println!("part 2 {}", part_2);
}
