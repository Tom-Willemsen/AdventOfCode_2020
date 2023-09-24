use bitvec::prelude::*;
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> Vec<u32> {
    raw_inp
        .lines()
        .next()
        .expect("empty input")
        .split(',')
        .map(|i| i.parse().expect("parse failed"))
        .collect::<Vec<_>>()
}

fn populate_initial_state(data: &[u32], turns: u32) -> (Vec<u32>, BitVec<u32, Lsb0>) {
    let mut spoken = vec![0; turns as usize];
    let mut existence_sieve = bitvec![u32, Lsb0; 0; turns as usize];

    data.iter().zip(0..).for_each(|(&n, idx)| {
        spoken[n as usize] = idx;
        existence_sieve.set(n as usize, true);
    });

    (spoken, existence_sieve)
}

fn simulate<const TURNS: u32>(data: &[u32]) -> u32 {
    let mut last: u32 = data[data.len() - 1];

    let (mut spoken, mut existence_sieve) = populate_initial_state(data, TURNS);

    let start_turn: u32 = data.len().try_into().expect("num turns exceeded u32 range");
    for turn in start_turn..TURNS {
        let next = if existence_sieve[last as usize] {
            turn - 1 - spoken[last as usize]
        } else {
            existence_sieve.set(last as usize, true);
            0
        };
        spoken[last as usize] = turn - 1;

        last = next;
    }

    last
}

fn calculate_p1(data: &[u32]) -> u32 {
    simulate::<2020>(data)
}

fn calculate_p2(data: &[u32]) -> u32 {
    simulate::<30000000>(data)
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

    const EXAMPLE: &str = include_str!("../../inputs/examples/2020_15");

    const EXAMPLE2: &str = "1,3,2";
    const EXAMPLE3: &str = "2,1,3";
    const EXAMPLE4: &str = "1,2,3";
    const EXAMPLE5: &str = "2,3,1";
    const EXAMPLE6: &str = "3,2,1";
    const EXAMPLE7: &str = "3,1,2";

    const REAL_DATA: &str = include_str!("../../inputs/real/2020_15");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(EXAMPLE)), 436);
    }

    #[test]
    fn test_p1_example_2() {
        assert_eq!(calculate_p1(&parse(EXAMPLE2)), 1);
    }

    #[test]
    fn test_p1_example_3() {
        assert_eq!(calculate_p1(&parse(EXAMPLE3)), 10);
    }

    #[test]
    fn test_p1_example_4() {
        assert_eq!(calculate_p1(&parse(EXAMPLE4)), 27);
    }

    #[test]
    fn test_p1_example_5() {
        assert_eq!(calculate_p1(&parse(EXAMPLE5)), 78);
    }

    #[test]
    fn test_p1_example_6() {
        assert_eq!(calculate_p1(&parse(EXAMPLE6)), 438);
    }

    #[test]
    fn test_p1_example_7() {
        assert_eq!(calculate_p1(&parse(EXAMPLE7)), 1836);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&parse(EXAMPLE)), 175594);
    }

    #[test]
    fn test_p2_example_2() {
        assert_eq!(calculate_p2(&parse(EXAMPLE2)), 2578);
    }

    #[test]
    fn test_p2_example_3() {
        assert_eq!(calculate_p2(&parse(EXAMPLE3)), 3544142);
    }

    #[test]
    fn test_p2_example_4() {
        assert_eq!(calculate_p2(&parse(EXAMPLE4)), 261214);
    }

    #[test]
    fn test_p2_example_5() {
        assert_eq!(calculate_p2(&parse(EXAMPLE5)), 6895259);
    }

    #[test]
    fn test_p2_example_6() {
        assert_eq!(calculate_p2(&parse(EXAMPLE6)), 18);
    }

    #[test]
    fn test_p2_example_7() {
        assert_eq!(calculate_p2(&parse(EXAMPLE7)), 362);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(REAL_DATA)), 694);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(REAL_DATA)), 21768614);
    }
}
