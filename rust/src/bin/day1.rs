use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let mut f = File::open(path)?;
    let mut input = String::new();
    // don't care about the output of this function ... ?
    let _ = f.read_to_string(&mut input);
    let mut totals: Vec<u32> = Vec::new();
    for elf in input.split("\n\n") {
        let mut total: u32 = 0;
        for cal in elf.lines() {
            match cal.parse::<u32>() {
                Ok(s) => {
                    total = total + s;
                    totals.push(total);
                }
                _ => (),
            }
        }
    }
    // reverse sort
    totals.sort_by(|a, b| b.cmp(a));

    if let Some(max) = totals.iter().max() {
        println!("part 1 solution {}", max)
    }

    let top: u32 = totals[0..3].iter().sum();
    println!("part 2 solution {}", top);

    Ok(())
}
