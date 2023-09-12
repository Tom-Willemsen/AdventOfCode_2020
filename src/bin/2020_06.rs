use ahash::AHashSet;
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> Vec<Vec<&str>> {
    raw_inp
        .trim()
        .split("\n\n")
        .map(|item| item.split('\n').map(|l| l.trim()).collect())
        .collect()
}

fn calculate(data: &[Vec<&str>]) -> (usize, usize) {
    let mut p1 = 0;
    let mut p2 = 0;

    for group in data.iter() {
        let mut hs: AHashSet<char> = AHashSet::default();
        for person in group.iter() {
            for c in person.chars() {
                hs.insert(c);
            }
        }
        p1 += hs.len();

        for item in hs.iter() {
            if group.iter().all(|person| person.contains(|c| &c == item)) {
                p2 += 1;
            }
        }
    }
    (p1, p2)
}

fn main() {
    let args = Cli::parse();
    let raw_inp = fs::read_to_string(args.input).expect("can't open input file");
    let data = parse(&raw_inp);
    let (p1, p2) = calculate(&data);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2020_06");
    const REAL_DATA: &str = include_str!("../../inputs/real/2020_06");

    #[test]
    fn test_example() {
        let (p1, p2) = calculate(&parse(&EXAMPLE_DATA));
        assert_eq!(p1, 11);
        assert_eq!(p2, 6);
    }

    #[test]
    fn test_real() {
        let (p1, p2) = calculate(&parse(&REAL_DATA));
        assert_eq!(p1, 6782);
        assert_eq!(p2, 3596);
    }
}
