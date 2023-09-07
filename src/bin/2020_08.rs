use ahash::AHashSet;
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> Vec<(&str, isize)> {
    raw_inp
        .trim()
        .split('\n')
        .map(|x| {
            let (inst, n) = x.split_once(' ').expect("parse fail");
            let ni: isize = n.parse().expect("parse as int failed");
            (inst, ni)
        })
        .collect()
}

fn simulate(data: &[(&str, isize)], swap: Option<usize>) -> (isize, bool) {
    let mut acc: isize = 0;
    let mut offset: usize = 0;

    let mut hs: AHashSet<usize> = AHashSet::default();

    while !hs.contains(&offset) {
        hs.insert(offset);

        let inst = data[offset].0;
        let n = data[offset].1;

        let do_swap = swap.map_or(false, |s| offset == s);
        let (was_jmp, was_nop) = (inst == "jmp", inst == "nop");
        let (is_jmp, is_nop) = if do_swap {
            (was_nop, was_jmp)
        } else {
            (was_jmp, was_nop)
        };

        if is_jmp {
            offset = offset.checked_add_signed(n).expect("offset under/overflow")
        } else if is_nop {
            offset += 1
        } else {
            debug_assert!(inst == "acc");
            acc += n;
            offset += 1
        }

        if offset == data.len() {
            return (acc, true);
        }
    }
    (acc, false)
}

fn calculate_p1(data: &[(&str, isize)]) -> isize {
    simulate(data, None).0
}

fn calculate_p2(data: &[(&str, isize)]) -> isize {
    for swap in 0..data.len() {
        if data[swap].0 != "acc" {
            let (acc_result, finished) = simulate(data, Some(swap));

            if finished {
                return acc_result;
            }
        }
    }
    panic!("p2: no solution found");
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
const TEST_DATA: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

#[test]
fn test_p1_example() {
    assert_eq!(calculate_p1(&parse(TEST_DATA)), 5);
}

#[test]
fn test_p2_example() {
    assert_eq!(calculate_p2(&parse(TEST_DATA)), 8);
}
