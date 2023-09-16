use clap::Parser;
use modinverse::modinverse;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> (i64, Vec<Option<i64>>) {
    let (l1, l2) = raw_inp.split_once('\n').expect("invalid format");

    let buses: Vec<Option<i64>> = l2
        .split(',')
        .map(|s| s.trim().parse::<i64>().ok())
        .collect();

    (l1.parse().expect("invalid ts"), buses)
}

fn calculate_p1(ts: i64, data: &[Option<i64>]) -> i64 {
    let bus: i64 = data
        .iter()
        .filter_map(|&b| b)
        .min_by_key(|&b| b - (ts % b))
        .expect("no buses");

    bus * (bus - (ts % bus))
}

/// chinese remainder theorem.
fn calculate_p2(data: &[Option<i64>]) -> i64 {
    let moduli: Vec<(i64, i64)> = data
        .iter()
        .enumerate()
        .filter(|(_, n)| n.is_some())
        .map(|(a, n)| (n.unwrap() - (a as i64), n.unwrap()))
        .collect();

    let prod: i64 = moduli.iter().map(|(_, n)| n).product();

    let result: i64 = moduli
        .iter()
        .map(|&(a, n)| a * (prod / n) * (modinverse(prod / n, n).unwrap()))
        .sum();

    result % prod
}

fn main() {
    let args = Cli::parse();
    let raw_inp = fs::read_to_string(args.input).expect("can't open input file");
    let (ts, data) = parse(&raw_inp);
    let p1 = calculate_p1(ts, &data);
    let p2 = calculate_p2(&data);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = include_str!("../../inputs/examples/2020_13");
    const REAL_DATA: &str = include_str!("../../inputs/real/2020_13");

    #[test]
    fn test_p1_example() {
        let (ts, data) = parse(TEST_DATA);
        assert_eq!(calculate_p1(ts, &data), 295);
    }

    #[test]
    fn test_p2_example() {
        let (_, data) = parse(TEST_DATA);
        assert_eq!(calculate_p2(&data), 1068781);
    }

    #[test]
    fn test_p1_real() {
        let (ts, data) = parse(REAL_DATA);
        assert_eq!(calculate_p1(ts, &data), 246);
    }

    #[test]
    fn test_p2_real() {
        let (_, data) = parse(REAL_DATA);
        assert_eq!(calculate_p2(&data), 939490236001473);
    }
}
