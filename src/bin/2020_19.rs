use ahash::AHashMap;
use clap::Parser;
use std::cmp::min;
use std::fs;
use std::str::FromStr;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

#[derive(Debug)]
enum Rule {
    Subrules(Vec<Vec<u64>>),
    Char(char),
}

#[derive(Debug)]
struct ParseRuleError {}

impl FromStr for Rule {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().starts_with('"') {
            Ok(Rule::Char(s.trim().chars().collect::<Vec<char>>()[1]))
        } else {
            Ok(Rule::Subrules(
                s.trim()
                    .split('|')
                    .map(|item| {
                        item.trim()
                            .split(' ')
                            .filter_map(|i| i.trim().parse().ok())
                            .collect()
                    })
                    .collect(),
            ))
        }
    }
}

fn parse(raw_inp: &str) -> (AHashMap<u64, Rule>, Vec<&str>) {
    let (rules, items) = raw_inp.trim().split_once("\n\n").expect("invalid format");

    let mut rules_map = AHashMap::default();

    rules
        .lines()
        .map(|ln| ln.split_once(':').expect("invalid format"))
        .map(|(ruleno, ln)| {
            let rule = ln.parse().expect("rule parse failed");
            let ruleno = ruleno.parse().expect("invalid rule number");
            (ruleno, rule)
        })
        .for_each(|(ruleno, rule)| {
            rules_map.insert(ruleno, rule);
        });

    (rules_map, items.lines().collect())
}

fn char_matches(data: &str, ch: char) -> (usize, bool) {
    (min(1, data.len()), data.starts_with(ch))
}

fn group_matches(data: &str, group: &[u64], rules: &AHashMap<u64, Rule>) -> (usize, bool) {
    let mut consumed = 0;

    for rule_index in group.iter() {
        let rule = rules.get(rule_index).expect("bad rule ref");

        let (c, m) = rule_matches(&data[consumed..], rule, rules);
        if m {
            consumed += c;
        } else {
            return (0, false);
        }
    }

    (consumed, consumed > 0)
}

fn rule_matches(data: &str, rule: &Rule, rules: &AHashMap<u64, Rule>) -> (usize, bool) {
    match rule {
        Rule::Subrules(subrules) => subrules
            .iter()
            .map(|group| group_matches(data, group, rules))
            .find(|&(_, matches)| matches)
            .unwrap_or((0, false)),
        Rule::Char(c) => char_matches(data, *c),
    }
}

fn calculate_p1(rules: &AHashMap<u64, Rule>, lines: &[&str]) -> usize {
    let rule0 = rules.get(&0).expect("rule 0 should exist");
    lines
        .iter()
        .map(|line| (line, rule_matches(line, rule0, rules)))
        .filter(|&(line, (matched_size, matched))| matched_size == line.len() && matched)
        .count()
}

fn match_many(line: &str, rule: &Rule, rules: &AHashMap<u64, Rule>) -> (usize, usize) {
    let mut consumed = 0;
    let mut instances = 0;
    loop {
        let (chars, matches) = rule_matches(&line[consumed..], rule, rules);
        if matches {
            consumed += chars;
            instances += 1;
        } else {
            break;
        }
    }
    (consumed, instances)
}

fn calculate_p2(rules: AHashMap<u64, Rule>, lines: &[&str]) -> usize {
    let mut new_rules: AHashMap<u64, Rule> = rules;

    let rule8 = Rule::Subrules(vec![vec![42], vec![42, 8]]);
    let rule11 = Rule::Subrules(vec![vec![42, 31], vec![42, 11, 31]]);

    new_rules.insert(8, rule8);
    new_rules.insert(11, rule11);

    let rule0 = new_rules.get(&0).expect("rule 0 should exist");
    let rule42 = new_rules.get(&42).expect("rule 42 should exist");
    let rule31 = new_rules.get(&31).expect("rule 31 should exist");

    match rule0 {
        Rule::Subrules(val) => {
            assert_eq!(val, &vec![vec![8, 11]])
        }
        _ => panic!("unexpected rule 0"),
    }

    let mut valid_lines = 0;

    for line in lines.iter() {
        let (consumed_rule_42, instances_rule_42) = match_many(line, rule42, &new_rules);
        let (consumed_rule_31, instances_rule_31) =
            match_many(&line[consumed_rule_42..], rule31, &new_rules);

        let all_consumed = (consumed_rule_42 + consumed_rule_31) == line.len();
        let is_valid_permutation = instances_rule_31 < instances_rule_42
            && instances_rule_31 >= 1
            && instances_rule_42 >= 1;

        if all_consumed && is_valid_permutation {
            valid_lines += 1;
        }
    }

    valid_lines
}

fn main() {
    let args = Cli::parse();
    let raw_inp = fs::read_to_string(args.input).expect("can't open input file");
    let (rules, lines) = parse(&raw_inp);
    let p1 = calculate_p1(&rules, &lines);
    let p2 = calculate_p2(rules, &lines);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_EXAMPLE_P1: &str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb 
"#;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2020_19");

    const REAL_DATA: &str = include_str!("../../inputs/real/2020_19");

    #[test]
    fn test_p1_small_example() {
        let (rules, _) = parse(SMALL_EXAMPLE_P1);
        assert_eq!(calculate_p1(&rules, &["ababbb"]), 1);
    }

    #[test]
    fn test_p1_small_example_1() {
        let (rules, _) = parse(SMALL_EXAMPLE_P1);
        assert_eq!(calculate_p1(&rules, &["ababbb"]), 1);
    }

    #[test]
    fn test_p1_small_example_2() {
        let (rules, _) = parse(SMALL_EXAMPLE_P1);
        assert_eq!(calculate_p1(&rules, &["bababa"]), 0);
    }

    #[test]
    fn test_p1_small_example_3() {
        let (rules, _) = parse(SMALL_EXAMPLE_P1);
        assert_eq!(calculate_p1(&rules, &["abbbab"]), 1);
    }

    #[test]
    fn test_p1_small_example_4() {
        let (rules, _) = parse(SMALL_EXAMPLE_P1);
        assert_eq!(calculate_p1(&rules, &["aaabbb"]), 0);
    }

    #[test]
    fn test_p1_small_example_5() {
        let (rules, _) = parse(SMALL_EXAMPLE_P1);
        assert_eq!(calculate_p1(&rules, &["aaaabbb"]), 0);
    }

    #[test]
    fn test_p1_full_example() {
        let (rules, lines) = parse(EXAMPLE_DATA);
        assert_eq!(calculate_p1(&rules, &lines), 3);
    }

    #[test]
    fn test_p2_example() {
        let (rules, lines) = parse(EXAMPLE_DATA);
        assert_eq!(calculate_p2(rules, &lines), 12);
    }

    #[test]
    fn test_p1_real() {
        let (rules, lines) = parse(REAL_DATA);
        assert_eq!(calculate_p1(&rules, &lines), 180);
    }

    #[test]
    fn test_p2_real() {
        let (rules, lines) = parse(REAL_DATA);
        assert_eq!(calculate_p2(rules, &lines), 323);
    }
}
