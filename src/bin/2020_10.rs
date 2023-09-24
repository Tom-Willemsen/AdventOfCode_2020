use ahash::{AHashMap, AHashSet};
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> AHashSet<u64> {
    raw_inp
        .trim()
        .split('\n')
        .map(|x| x.parse().expect("parse as int failed"))
        .collect()
}

fn calculate_p1(data: &AHashSet<u64>) -> u64 {
    let mut joltage: u64 = 0;
    let mut increases: [u64; 3] = [0, 0, 0];

    let mut any_changed = true;
    while any_changed {
        any_changed = false;
        for increase in 1..4 {
            if data.contains(&(joltage + increase)) {
                joltage += increase;
                increases[(increase - 1) as usize] += 1;
                any_changed = true;
                break;
            }
        }
    }

    increases[0] * (increases[2] + 1)
}

fn calculate_p2(data: &AHashSet<u64>) -> u64 {
    let mut ways: AHashMap<u64, u64> = AHashMap::default();
    ways.insert(0, 1);

    let max_joltage: u64 = *data.iter().max().expect("no data");

    for joltage in 1..max_joltage + 1 {
        let mut w = 0;
        if data.contains(&(joltage)) {
            for diff in 1..4 {
                if joltage >= diff {
                    w += ways.get(&(joltage - diff)).unwrap_or(&0);
                }
            }
        }
        ways.insert(joltage, w);
    }

    *ways.get(&max_joltage).expect("p2 calculation failed")
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

    const TEST_DATA_1: [u64; 11] = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    const TEST_DATA_2: [u64; 31] = [
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];
    const REAL_DATA: &str = include_str!("../../inputs/real/2020_10");

    #[test]
    fn test_p1_examples() {
        assert_eq!(
            calculate_p1(&AHashSet::from_iter(TEST_DATA_1.into_iter())),
            35
        );
        assert_eq!(
            calculate_p1(&AHashSet::from_iter(TEST_DATA_2.into_iter())),
            220
        );
    }

    #[test]
    fn test_p2_examples() {
        assert_eq!(
            calculate_p2(&AHashSet::from_iter(TEST_DATA_1.into_iter())),
            8
        );
        assert_eq!(
            calculate_p2(&AHashSet::from_iter(TEST_DATA_2.into_iter())),
            19208
        );
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(REAL_DATA)), 2240);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(REAL_DATA)), 99214346656768);
    }
}
