use ahash::AHashSet;
use clap::Parser;
use std::collections::VecDeque;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> (Vec<u32>, Vec<u32>) {
    raw_inp
        .trim()
        .split_once("\n\n")
        .map(|(p1, p2)| (parse_deck(p1), parse_deck(p2)))
        .expect("invalid input format")
}

fn parse_deck(inp: &str) -> Vec<u32> {
    debug_assert!(inp.starts_with("Player "));
    inp.lines()
        .skip(1)
        .map(|x| x.parse().expect("couldn't parse card"))
        .collect()
}

fn score(data: impl DoubleEndedIterator<Item = u32>) -> u32 {
    data.rev().zip(1u32..).map(|(card, pos)| pos * card).sum()
}

fn create_deques(p1_cards: &[u32], p2_cards: &[u32]) -> (VecDeque<u32>, VecDeque<u32>) {
    let total_cards = p1_cards.len() + p2_cards.len();
    let mut p1: VecDeque<u32> = VecDeque::with_capacity(total_cards);
    let mut p2: VecDeque<u32> = VecDeque::with_capacity(total_cards);

    p1_cards.iter().for_each(|&itm| p1.push_back(itm));
    p2_cards.iter().for_each(|&itm| p2.push_back(itm));

    (p1, p2)
}

fn calculate_p1(p1_cards: &[u32], p2_cards: &[u32]) -> u32 {
    let (mut p1, mut p2) = create_deques(p1_cards, p2_cards);

    while !p1.is_empty() && !p2.is_empty() {
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }

    if p2.is_empty() {
        score(p1.into_iter())
    } else {
        score(p2.into_iter())
    }
}

#[derive(PartialEq, Eq)]
enum Winner {
    P1,
    P2,
}

fn p2_game_recursive(p1: &mut VecDeque<u32>, p2: &mut VecDeque<u32>) -> Winner {
    let mut seen: AHashSet<(VecDeque<u32>, VecDeque<u32>)> = AHashSet::with_capacity(512);

    while !p1.is_empty() && !p2.is_empty() {
        if !seen.insert((p1.clone(), p2.clone())) {
            return Winner::P1;
        }

        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        let round_winner =
            if u32::try_from(p1.len()).unwrap() >= c1 && u32::try_from(p2.len()).unwrap() >= c2 {
                let mut new_p1_cards: VecDeque<u32> = p1
                    .iter()
                    .take(usize::try_from(c1).unwrap())
                    .cloned()
                    .collect();
                let mut new_p2_cards: VecDeque<u32> = p2
                    .iter()
                    .take(usize::try_from(c2).unwrap())
                    .cloned()
                    .collect();
                p2_game_recursive(&mut new_p1_cards, &mut new_p2_cards)
            } else if c1 > c2 {
                Winner::P1
            } else {
                Winner::P2
            };

        if round_winner == Winner::P1 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }
    // println!("seen len {:?}", seen.len());

    if p2.is_empty() {
        Winner::P1
    } else {
        Winner::P2
    }
}

fn calculate_p2(p1_cards: &[u32], p2_cards: &[u32]) -> u32 {
    let (mut p1, mut p2) = create_deques(p1_cards, p2_cards);

    let winner = p2_game_recursive(&mut p1, &mut p2);

    if winner == Winner::P1 {
        score(p1.into_iter())
    } else {
        score(p2.into_iter())
    }
}

fn main() {
    let args = Cli::parse();
    let raw_inp = fs::read_to_string(args.input).expect("can't open input file");
    let (p1_deck, p2_deck) = parse(&raw_inp);
    let p1 = calculate_p1(&p1_deck, &p2_deck);
    let p2 = calculate_p2(&p1_deck, &p2_deck);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2020_22");

    const REAL_DATA: &str = include_str!("../../inputs/real/2020_22");

    const TERMINATION_EXAMPLE: &str = "Player 1:
43
19

Player 2:
2
29
14";

    #[test]
    fn test_p1_example() {
        let (p1, p2) = parse(EXAMPLE_DATA);
        assert_eq!(calculate_p1(&p1, &p2), 306);
    }

    #[test]
    fn test_p1_real() {
        let (p1, p2) = parse(REAL_DATA);
        assert_eq!(calculate_p1(&p1, &p2), 34566);
    }

    #[test]
    fn test_p2_example() {
        let (p1, p2) = parse(EXAMPLE_DATA);
        assert_eq!(calculate_p2(&p1, &p2), 291);
    }

    #[test]
    fn test_p2_should_terminate() {
        let (p1, p2) = parse(TERMINATION_EXAMPLE);
        assert_eq!(calculate_p2(&p1, &p2), 105);
    }

    #[test]
    fn test_p2_real() {
        let (p1, p2) = parse(REAL_DATA);
        assert_eq!(calculate_p2(&p1, &p2), 31854);
    }
}
