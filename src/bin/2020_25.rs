use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> (u64, u64) {
    raw_inp
        .trim()
        .split_once('\n')
        .map(|(k1, k2)| {
            (
                k1.parse().expect("invalid number"),
                k2.parse().expect("invalid number"),
            )
        })
        .expect("invalid format")
}

const SUBJECT: u64 = 7;

fn calculate(k1: u64, k2: u64) -> u64 {
    let mut current = SUBJECT;
    
    let mut transformed_k1 = k1;
    let mut transformed_k2 = k2;

    loop {
        current = (current * SUBJECT) % 20201227;
        transformed_k1 = (transformed_k1 * k1) % 20201227;
        transformed_k2 = (transformed_k2 * k2) % 20201227;

        if current == k1 {
            return transformed_k2;
        } else if current == k2 {
            return transformed_k1;
        }
    }
}

fn main() {
    let args = Cli::parse();
    let raw_inp = fs::read_to_string(args.input).expect("can't open input file");
    let (k1, k2) = parse(&raw_inp);
    let p1 = calculate(k1, k2);
    println!("{}", p1);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2020_25");
    const REAL_DATA: &str = include_str!("../../inputs/real/2020_25");

    #[test]
    fn test_example() {
        let (k1, k2) = parse(EXAMPLE_DATA);
        assert_eq!(calculate(k1, k2), 14897079);
    }

    #[test]
    fn test_real() {
        let (k1, k2) = parse(REAL_DATA);
        assert_eq!(calculate(k1, k2), 8329514);
    }
}
