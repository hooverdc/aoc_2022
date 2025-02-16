use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let values: HashMap<&str, i32> = HashMap::from_iter(vec![
        ("A", 1),
        ("B", 2),
        ("C", 3),
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
    ]);

    let part_1: i32 = read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            // println!("line {}", line);
            let score: i32;
            if let Some((left, right)) = line.split_once(" ") {
                let lv = values.get(left).unwrap();
                let rv = values.get(right).unwrap();
                // println!("{} - {}", lv, rv);
                if lv.rem_euclid(3) + 1 == *rv {
                    score = 6 + *rv;
                } else if lv == rv {
                    score = 3 + *rv;
                } else {
                    score = 0 + *rv;
                }
                return score;
            }
            return 0;
        })
        .reduce(|total, score| total + score)
        .unwrap();

    println!("part 1: {}", part_1);

    let part_2: i32 = read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            // println!("line {}", line);
            let score: i32;
            if let Some((left, right)) = line.split_once(" ") {
                let lv = values.get(left).unwrap();
                let rv = values.get(right).unwrap();
                println!("{} {}", lv, rv);
                if *rv == 1 {
                    // lose
                    score = ((lv + 1) -3).rem_euclid(3) + 1
                } else if *rv == 2 {
                    // draw
                    score = 3 + lv
                } else {
                    // win
                    score = 6 + (lv.rem_euclid(3) + 1)
                }
                println!("score {}", score);
                return score;
            }
            return 0;
        })
        .reduce(|total, score| total + score)
        .unwrap();

    println!("part 2: {}", part_2);
}
