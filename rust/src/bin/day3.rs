use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;

// https://users.rust-lang.org/t/how-to-breakup-an-iterator-into-chunks/87915/5
// props to steffahn
// I am spoiled by itertools in Python
fn batched<I>(a: impl IntoIterator<Item = I>, batch_size: usize) -> impl Iterator<Item = Vec<I>> {
    let mut a = a.into_iter();
    // closures yipee
    std::iter::from_fn(move || {
        Some(a.by_ref().take(batch_size).collect()).filter(|chunk: &Vec<_>| !chunk.is_empty())
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input = read_to_string(path).unwrap();
    // lookup
    let mut part_1: usize = 0;
    let priority: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    for line in input.lines() {
        let mid = line.len() / 2;
        let (first, second) = line.split_at(mid);
        for letter in first.chars() {
            // should be only one match
            if second.contains(letter) {
                let rank = priority.find(letter).unwrap() + 1;
                println!("item {} {}", letter, rank);
                part_1 += rank;
                break;
            }
        }
    }
    println!("part 1 {}", part_1);

    let mut part_2: usize = 0;

    for group in batched(input.lines(), 3) {
        println!("{group:?}");
        let one: HashSet<char> = HashSet::from_iter(group[0].chars());
        let two: HashSet<char> = HashSet::from_iter(group[1].chars());
        let three: HashSet<char> = HashSet::from_iter(group[2].chars());
        let items = &(&one & &two) & &three;
        for letter in items.iter() {
            let rank = priority.find(*letter).unwrap() + 1;
            part_2 += rank;
        }
    }

    println!("part 2 {}", part_2);
}
