use ahash::AHashMap;
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> Vec<usize> {
    raw_inp
        .lines()
        .next()
        .expect("empty input")
        .split(',')
        .map(|i| i.parse().expect("parse failed"))
        .collect::<Vec<_>>()
}

fn simulate(data: &[usize], turns: usize) -> usize {
    // Lower 1/4 of numbers -> dense -> store in array cache
    // Remaining numbers -> relatively sparse -> hashmap
    let small: usize = turns / 4;
    let mut last: usize = data[data.len() - 1];

    let mut small_spoken = vec![usize::MAX; small];
    let mut large_spoken: AHashMap<usize, usize> = AHashMap::default();

    data.iter().enumerate().for_each(|(idx, &n)| {
        if n < small {
            small_spoken[n] = idx;
        } else {
            large_spoken.insert(n, idx);
        }
    });

    for turn in data.len()..turns {
        let old;

        if last < small {
            old = Some(small_spoken[last]);
            small_spoken[last] = turn - 1;
        } else {
            old = large_spoken.insert(last, turn - 1);
        }

        last = match old {
            Some(prev_turn) => (turn - 1).saturating_sub(prev_turn),
            None => 0,
        }
    }

    last
}

fn calculate_p1(data: &[usize]) -> usize {
    simulate(data, 2020)
}

fn calculate_p2(data: &[usize]) -> usize {
    simulate(data, 30000000)
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
