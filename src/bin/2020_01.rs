use clap::Parser;
use std::fs;
use fnv::FnvHashSet;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}
 
fn main() {
    let args = Cli::parse();

    let inp = match fs::read_to_string(args.input) {
        Ok(i) => i,
        Err(error) => panic!("Can't open input: {:?}", error),
    };

    let nums: FnvHashSet<i64> = inp.trim()
        .split("\n")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let mut p1 = 0;
    for n in nums.iter() {
        if nums.contains(&(2020 - n)) {
            p1 = n * (2020 - n);
            break;
        }
    }

    let mut p2 = 0;
    for x in nums.iter() {
        for y in nums.iter() {
            if nums.contains(&(2020 - x - y)) {
                p2 = x * y * (2020 - x - y);
                break;
            }
        }
    }

    println!("{:?}\n{:?}", p1, p2);
}

