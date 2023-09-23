use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

#[derive(Eq, PartialEq, Debug)]
enum TokenType {
    Number(u64),
    MulTok,
    AddTok,
    ParenStart,
    ParenEnd,
    End,
}

fn parse_line(line: &str) -> Vec<TokenType> {
    let mut tokens = vec![];

    for c in line.chars() {
        match c {
            ' ' => {}
            '(' => tokens.push(TokenType::ParenStart),
            ')' => tokens.push(TokenType::ParenEnd),
            '*' => tokens.push(TokenType::MulTok),
            '+' => tokens.push(TokenType::AddTok),
            n => tokens.push(TokenType::Number((n.to_digit(10).unwrap()) as u64)),
        }
    }
    tokens.push(TokenType::End);
    tokens
}

fn parse(raw_inp: &str) -> Vec<Vec<TokenType>> {
    raw_inp.trim().lines().map(parse_line).collect()
}

fn consume(data: &[TokenType], typ: TokenType) -> usize {
    assert_eq!(data[0], typ);
    1
}

fn group<const PART: u8>(data: &[TokenType]) -> (usize, u64) {
    let mut consumed = 0;

    consumed += consume(&data[0..], TokenType::ParenStart);
    let expr = expression::<PART>(&data[1..]);
    consumed += expr.0;
    consumed += consume(&data[consumed..], TokenType::ParenEnd);

    (consumed, expr.1)
}

fn number(data: &[TokenType]) -> (usize, u64) {
    match data[0] {
        TokenType::Number(n) => (1, n),
        _ => panic!("not a number"),
    }
}

fn group_or_number<const PART: u8>(data: &[TokenType]) -> (usize, u64) {
    match data[0] {
        TokenType::ParenStart => group::<PART>(data),
        TokenType::Number(_) => number(data),
        _ => panic!("not a group or number"),
    }
}

fn expression<const PART: u8>(data: &[TokenType]) -> (usize, u64) {
    let mut consumed = 0;
    let mut val = None;

    loop {
        match data[consumed] {
            TokenType::ParenStart => {
                let (c, n) = group::<PART>(&data[consumed..]);
                consumed += c;
                val = Some(n);
            }
            TokenType::Number(_) => {
                let (c, n) = number(&data[consumed..]);
                consumed += c;
                val = Some(n);
            }
            TokenType::AddTok => {
                let prev = val.expect("add without val set");
                consumed += consume(&data[consumed..], TokenType::AddTok);
                let (c, n) = group_or_number::<PART>(&data[consumed..]);
                consumed += c;
                val = Some(prev + n);
            }
            TokenType::MulTok => {
                let prev = val.expect("mul without val set");
                consumed += consume(&data[consumed..], TokenType::MulTok);
                let (c, n) = if PART == 1 {
                    group_or_number::<PART>(&data[consumed..])
                } else {
                    expression::<PART>(&data[consumed..])
                };

                consumed += c;
                val = Some(prev * n);
            }
            _ => {
                break;
            }
        }
    }
    (consumed, val.expect("val not set"))
}

fn calculate<const PART: u8>(data: &[Vec<TokenType>]) -> u64 {
    data.iter().map(|line| expression::<PART>(line).1).sum()
}

fn main() {
    let args = Cli::parse();
    let raw_inp = fs::read_to_string(args.input).expect("can't open input file");
    let data = parse(&raw_inp);
    let p1 = calculate::<1>(&data);
    let p2 = calculate::<2>(&data);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_DATA: &str = include_str!("../../inputs/real/2020_18");

    #[test]
    fn test_p1_example0() {
        assert_eq!(calculate::<1>(&parse("1 + 2 * 3 + 4 * 5 + 6")), 71);
    }

    #[test]
    fn test_p1_example1() {
        assert_eq!(calculate::<1>(&parse("1 + (2 * 3) + (4 * (5 + 6))")), 51);
    }

    #[test]
    fn test_p1_example2() {
        assert_eq!(calculate::<1>(&parse("2 * 3 + (4 * 5)")), 26);
    }

    #[test]
    fn test_p1_example3() {
        assert_eq!(calculate::<1>(&parse("5 + (8 * 3 + 9 + 3 * 4 * 3)")), 437);
    }

    #[test]
    fn test_p1_example4() {
        assert_eq!(
            calculate::<1>(&parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")),
            12240
        );
    }

    #[test]
    fn test_p1_example5() {
        assert_eq!(
            calculate::<1>(&parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")),
            13632
        );
    }

    #[test]
    fn test_p2_example0() {
        assert_eq!(calculate::<2>(&parse("1 + 2 * 3 + 4 * 5 + 6")), 231);
    }

    #[test]
    fn test_p2_example1() {
        assert_eq!(calculate::<2>(&parse("1 + (2 * 3) + (4 * (5 + 6))")), 51);
    }

    #[test]
    fn test_p2_example2() {
        assert_eq!(calculate::<2>(&parse("2 * 3 + (4 * 5)")), 46);
    }

    #[test]
    fn test_p2_example3() {
        assert_eq!(calculate::<2>(&parse("5 + (8 * 3 + 9 + 3 * 4 * 3)")), 1445);
    }

    #[test]
    fn test_p2_example4() {
        assert_eq!(
            calculate::<2>(&parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")),
            669060
        );
    }

    #[test]
    fn test_p2_example5() {
        assert_eq!(
            calculate::<2>(&parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")),
            23340
        );
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate::<1>(&parse(REAL_DATA)), 3885386961962);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate::<2>(&parse(REAL_DATA)), 112899558798666);
    }
}
