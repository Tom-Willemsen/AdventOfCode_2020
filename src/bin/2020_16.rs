use ahash::AHashMap;
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct ValidityRules {
    start_1: u64,
    end_1: u64,
    start_2: u64,
    end_2: u64,
    is_departure: bool,
}

fn parse(raw_inp: &str) -> (Vec<ValidityRules>, Vec<u64>, Vec<Vec<u64>>) {
    let (ticket_info, tickets) = raw_inp.split_once("your ticket:").expect("invalid format");

    let (my_ticket, nearby_tickets) = tickets
        .split_once("nearby tickets:")
        .expect("invalid format");

    let validity_rules = ticket_info
        .trim()
        .lines()
        .map(|info| {
            let (name, values) = info.split_once(": ").expect("invalid format");
            let (v1, v2) = values.split_once(" or ").expect("invalid format");
            let (start_1, end_1) = v1.split_once('-').expect("invalid format");
            let (start_2, end_2) = v2.split_once('-').expect("invalid format");

            ValidityRules {
                is_departure: name.starts_with("departure"),
                start_1: start_1.parse().expect("parse failed"),
                end_1: end_1.parse().expect("parse failed"),
                start_2: start_2.parse().expect("parse failed"),
                end_2: end_2.parse().expect("parse failed"),
            }
        })
        .collect();

    let my_ticket_parsed: Vec<u64> = my_ticket
        .trim()
        .split(',')
        .map(|v| v.parse().expect("parse failed"))
        .collect();

    let other_tickets: Vec<Vec<u64>> = nearby_tickets
        .trim()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|v| v.parse().expect("parse failed"))
                .collect()
        })
        .collect();

    (validity_rules, my_ticket_parsed, other_tickets)
}

fn rule_matches(rule: &ValidityRules, n: u64) -> bool {
    (rule.start_1 <= n && rule.end_1 >= n) || (rule.start_2 <= n && rule.end_2 >= n)
}

fn any_rule_matches(rules: &[ValidityRules], value: u64) -> bool {
    rules.iter().any(|rule| rule_matches(rule, value))
}

fn ticket_matches_any_rule(rules: &[ValidityRules], ticket: &[u64]) -> bool {
    ticket.iter().all(|&val| any_rule_matches(rules, val))
}

fn ticket_error_rate(rules: &[ValidityRules], ticket: &[u64]) -> u64 {
    ticket
        .iter()
        .map(|&val| if any_rule_matches(rules, val) { 0 } else { val })
        .sum()
}

fn calculate_p1(rules: &[ValidityRules], _my_ticket: &[u64], other_tickets: &[Vec<u64>]) -> u64 {
    other_tickets
        .iter()
        .map(|ticket| ticket_error_rate(rules, ticket))
        .sum()
}

fn calculate_p2(rules: &[ValidityRules], my_ticket: &[u64], other_tickets: &[Vec<u64>]) -> u64 {
    let valid_other_tickets: Vec<_> = other_tickets
        .iter()
        .filter(|ticket| ticket_matches_any_rule(rules, ticket))
        .collect();

    let num_fields = valid_other_tickets[0].len();

    let mut assignments: AHashMap<&ValidityRules, usize> = AHashMap::default();

    // Which rules could be valid in which positions?
    let mut possible_assignments: Vec<Vec<&ValidityRules>> = (0..num_fields)
        .map(|field| {
            rules
                .iter()
                .filter(|rule| {
                    valid_other_tickets
                        .iter()
                        .all(|ticket| rule_matches(rule, ticket[field]))
                })
                .collect()
        })
        .collect();

    // Repeatedly find a rule which is the only valid rule
    // in a particular position, assign that rule to that
    // position and remove it from being valid in other positions
    while assignments.len() != rules.len() {
        for field in 0..num_fields {
            if possible_assignments[field].len() == 1 {
                let key = possible_assignments[field][0];
                let new_assignments = possible_assignments
                    .iter()
                    .map(|assignments| assignments.iter().filter(|&x| x != &key).copied().collect())
                    .collect::<Vec<Vec<_>>>();

                assignments.insert(key, field);
                possible_assignments = new_assignments;
            }
        }
    }

    assignments
        .iter()
        .filter(|(k, _)| k.is_departure)
        .map(|(_, &v)| my_ticket[v])
        .product()
}

fn main() {
    let args = Cli::parse();
    let raw_inp = fs::read_to_string(args.input).expect("can't open input file");
    let (rules, ticket, other_tickets) = parse(&raw_inp);
    let p1 = calculate_p1(&rules, &ticket, &other_tickets);
    let p2 = calculate_p2(&rules, &ticket, &other_tickets);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../inputs/examples/2020_16");

    const REAL_DATA: &str = include_str!("../../inputs/real/2020_16");

    #[test]
    fn test_p1_example() {
        let (rules, ticket, other_tickets) = parse(&EXAMPLE);
        assert_eq!(calculate_p1(&rules, &ticket, &other_tickets), 71);
    }

    #[test]
    fn test_p2_example() {
        let (rules, ticket, other_tickets) = parse(&EXAMPLE);
        assert_eq!(calculate_p2(&rules, &ticket, &other_tickets), 1);
    }

    #[test]
    fn test_p1_real() {
        let (rules, ticket, other_tickets) = parse(&REAL_DATA);
        assert_eq!(calculate_p1(&rules, &ticket, &other_tickets), 20060);
    }

    #[test]
    fn test_p2_real() {
        let (rules, ticket, other_tickets) = parse(&REAL_DATA);
        assert_eq!(calculate_p2(&rules, &ticket, &other_tickets), 2843534243843);
    }
}
