use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> Vec<i64> {
    raw_inp
        .trim()
        .split('\n')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part1(nums: &Vec<i64>) -> i64 {
    for x in 0..nums.len() {
        for y in (x + 1)..nums.len() {
            if (nums[x] + nums[y]) == 2020 {
                return nums[x] * nums[y];
            }
        }
    }
    panic!("p1 solution not found");
}

fn part2(nums: &Vec<i64>) -> i64 {
    for x in 0..nums.len() {
        for y in (x + 1)..nums.len() {
            for z in (y + 1)..nums.len() {
                if (nums[x] + nums[y] + nums[z]) == 2020 {
                    return nums[x] * nums[y] * nums[z];
                }
            }
        }
    }
    panic!("p2 solution not found");
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let nums: Vec<i64> = parse(&inp);
    println!("{}\n{}", part1(&nums), part2(&nums));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2020_01");
    const REAL_DATA: &str = include_str!("../../inputs/real/2020_01");

    #[test]
    fn test_p1_example() {
        assert_eq!(part1(&parse(&EXAMPLE_DATA)), 514579);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(part2(&parse(&EXAMPLE_DATA)), 241861950);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(part1(&parse(&REAL_DATA)), 538464);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(part2(&parse(&REAL_DATA)), 278783190);
    }
}
