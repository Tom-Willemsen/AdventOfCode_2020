use ahash::{AHashSet, AHashMap};
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> Vec<&str> {
    raw_inp
        .trim()
        .lines()
        .collect()
}

fn calc_line_p1(_data: &str) -> usize {
    0
}

fn calc_line_p2(_data: &str) -> usize {
    0
}

fn calculate_p1(data: &[&str]) -> usize {
    data.iter()
        .map(|line| calc_line_p1(line))
        .sum()
}

fn calculate_p2(data: &[&str]) -> usize {
    data.iter()
        .map(|line| calc_line_p2(line))
        .sum()
}

fn main() {
    let args = Cli::parse();
    let raw_inp = fs::read_to_string(args.input).expect("can't open input file");
    let data = parse(&raw_inp);
    let p1 = calculate_p1(&data);
    let p2 = calculate_p2(&data);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    // const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2020_17");
    // 
    // const REAL_DATA: &str = include_str!("../../inputs/real/2020_17");
    // 
    // #[test]
    // fn test_p1_example() {
    //     assert_eq!(calculate_p1(&parse(EXAMPLE_DATA)), 112);
    // }
    // 
    // #[test]
    // fn test_p2_example() {
    //     assert_eq!(calculate_p2(&parse(EXAMPLE_DATA)), 848);
    // }
    // 
    // #[test]
    // fn test_p1_real() {
    //     assert_eq!(calculate_p1(&parse(REAL_DATA)), 242);
    // }
    // 
    // #[test]
    // fn test_p2_real() {
    //     assert_eq!(calculate_p2(&parse(REAL_DATA)), 2292);
    // }
}
