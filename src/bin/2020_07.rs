use ahash::{AHashMap, AHashSet};
use clap::Parser;
use std::fs;

const MY_BAG: &str = "shiny gold";

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse_bag_contents(inp: &str) -> Vec<(usize, &str)> {
    if inp == "no other bags" {
        return vec![];
    }

    inp.trim()
        .split(", ")
        .map(|i| i.rsplit_once(' ').expect("parse failed: split last word").0) // get rid of "bag" / "bags"
        .map(|i| {
            let (start, end) = i.split_once(' ').expect("parse failed: split n/bags");
            let n: usize = start.parse().expect("parse failed: parse n");
            (n, end)
        })
        .collect()
}

fn parse(raw_inp: &str) -> Vec<(&str, Vec<(usize, &str)>)> {
    raw_inp
        .trim()
        .split('\n')
        .map(|item| item.split_once(" bags contain "))
        .map(|item| item.expect("invalid input"))
        .map(|(bag, subbags)| (bag, subbags.trim_end_matches('.')))
        .map(|(bag, subbags)| (bag, parse_bag_contents(subbags)))
        .collect()
}

fn calculate_p1(data: &[(&str, Vec<(usize, &str)>)]) -> usize {
    let mut reachable: AHashSet<&str> = AHashSet::default();
    reachable.insert(MY_BAG);

    let mut any_changed = true;
    while any_changed {
        any_changed = false;
        for (bag, subbags) in data {
            if !reachable.contains(bag) && subbags.iter().any(|x| reachable.contains(x.1)) {
                any_changed |= reachable.insert(bag);
            }
        }
    }
    reachable.len() - 1
}

fn calculate_p2(data: &[(&str, Vec<(usize, &str)>)]) -> usize {
    let mut map: AHashMap<&str, usize> = AHashMap::with_capacity(data.len());

    let mut any_changed = true;
    while any_changed {
        any_changed = false;
        for (bag, subbags) in data {
            if !map.contains_key(bag) && subbags.iter().all(|x| map.contains_key(x.1)) {
                let mut c = 1;
                for (n, bag_type) in subbags.iter() {
                    c += n * map.get(bag_type).unwrap();
                }
                map.insert(bag, c);
                any_changed = true;
            }
        }
        if let Some(result) = map.get(MY_BAG) {
            return result - 1;
        }
    }

    panic!("p2 calculation failed");
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
const P1_TEST_DATA: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

#[test]
fn test_p1_example() {
    assert_eq!(calculate_p1(&parse(P1_TEST_DATA)), 4);
}

#[test]
fn test_p2_example_1() {
    assert_eq!(calculate_p2(&parse(P1_TEST_DATA)), 32);
}

#[test]
fn test_p2_example_2() {
    let test_data = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    assert_eq!(calculate_p2(&parse(test_data)), 126);
}
