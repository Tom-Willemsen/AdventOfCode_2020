use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
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

    let nums: Vec<i64> = inp.trim().split('\n').map(|s| s.parse().unwrap()).collect();

    println!("{}\n{}", part1(&nums), part2(&nums));
}

#[test]
fn test_p1_example() {
    let nums = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(part1(&nums), 514579);
}

#[test]
fn test_p2_example() {
    let nums = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(part2(&nums), 241861950);
}
