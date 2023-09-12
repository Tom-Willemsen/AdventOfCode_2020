use clap::Parser;
use std::cmp::Ordering;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> Vec<u64> {
    raw_inp
        .trim()
        .split('\n')
        .map(|x| x.parse().expect("parse as int failed"))
        .collect()
}

fn combination_adds_to(data: &[u64], target: u64) -> bool {
    for i in data {
        if i > &target {
            continue;
        } else if data.contains(&(target - i)) {
            return true;
        }
    }
    false
}

fn calculate_p1(data: &[u64], window_size: usize) -> u64 {
    for i in window_size..data.len() {
        if !combination_adds_to(&data[i - window_size..i], data[i]) {
            return data[i];
        }
    }
    panic!("p1: no solution");
}

fn calculate_p2(data: &[u64], target: u64) -> u64 {
    let mut begin: usize = 0;
    let mut end: usize = 0;
    let mut sum: u64 = 0;

    loop {
        match sum.cmp(&target) {
            Ordering::Less => {
                sum += data[end];
                end += 1;
            }
            Ordering::Greater => {
                sum -= data[begin];
                begin += 1;
            }
            Ordering::Equal => {
                let slice = &data[begin..end];
                let smallest = slice.iter().min().expect("empty slice (min)");
                let biggest = slice.iter().max().expect("empty slice (max)");
                return smallest + biggest;
            }
        }
    }
}

fn main() {
    let args = Cli::parse();
    let raw_inp = fs::read_to_string(args.input).expect("can't open input file");
    let data = parse(&raw_inp);
    let p1 = calculate_p1(&data, 25);
    let p2 = calculate_p2(&data, p1);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = include_str!("../../inputs/examples/2020_09");
    const REAL_DATA: &str = include_str!("../../inputs/real/2020_09");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(&TEST_DATA), 5), 127);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&parse(&TEST_DATA), 127), 62);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(&REAL_DATA), 25), 10884537);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(&REAL_DATA), 10884537), 1261309);
    }
}
