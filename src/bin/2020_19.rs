use ahash::{AHashSet, AHashMap};
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

enum Rule {
    Subrules(Vec<Vec<u64>>),
    Char(char),
}

fn parse(raw_inp: &str) -> (AHashMap<u64, Rule>, Vec<&str>) {
    let (rules, items) = raw_inp
        .trim()
        .split_once("\n\n")
        .expect("invalid format");
        
    let mut rules_map = AHashMap::default();    
    
    rules
        .lines()
        .map(|ln| ln.split_once(':').expect("invalid format"))
        .map(|(ruleno, ln)| {
            let p: Rule;
            if ln.trim().starts_with('"') {
                p = Rule::Char(ln.trim().chars().collect::<Vec<char>>()[1]);
            } else {
                p = Rule::Subrules(ln
                    .trim()
                    .split('|')
                    .map(|item| 
                        item
                            .trim()
                            .split(' ')
                            .map(|i| i.trim().parse().expect("parse fail"))
                            .collect()
                    )
                    .collect());
            }
            let r = ruleno.parse().expect("invalid rule number");
            (r, p)
        })
        .for_each(|(ruleno, line)| {rules_map.insert(ruleno, line);});
        
    (rules_map, items.lines().collect())
}

fn char_matches(data: &str, ch: char) -> bool {
    data.starts_with(ch)
}

fn group_matches(data: &str, group: &Vec<u64>, rules: &AHashMap<u64, Rule>) -> bool {
    group.iter()
        .map(|rule_index| rules.get(rule_index).expect("bad rule ref"))
        .any(|(rule)| rule_matches(&data, rule, &rules))
}

fn rule_matches(data: &str, rule: &Rule, rules: &AHashMap<u64, Rule>) -> bool {
    match rule {
        Rule::Subrules(subrules) => {
            subrules.iter()
                .any(|group| group_matches(data, group, rules))
        },
        Rule::Char(c) => {
             char_matches(data, *c)
        }
    }
}

fn calculate_p1(rules: &AHashMap<u64, Rule>, lines: &[&str]) -> usize {
    // lines.iter()
    //     .filter(|line| rule_matches(line, rules.get(&0).expect("rule0"), rules))
    //     .count()
    
    let mut a_valid: AHashMap<u64, bool> = AHashMap::default();
    let mut b_valid: AHashMap<u64, bool> = AHashMap::default();
    
    let mut offset = 0;
    
    0
}

fn calculate_p2(rules: &AHashMap<u64, Rule>, lines: &[&str]) -> usize {
    0
}

fn main() {
    let args = Cli::parse();
    let raw_inp = fs::read_to_string(args.input).expect("can't open input file");
    let (rules, lines) = parse(&raw_inp);
    let p1 = calculate_p1(&rules, &lines);
    let p2 = calculate_p2(&rules, &lines);
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
