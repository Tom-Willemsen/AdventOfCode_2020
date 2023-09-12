use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

struct Password {
    start: usize,
    end: usize,
    needle: char,
    pwd: String,
}

fn parse_single(inp: &str) -> Password {
    // takes ~6ms (!)
    // let parsed = sscanf!(inp, "{usize}-{usize} {char}: {str}");
    // let (start, end, needle, pwd) = parsed.expect("unable to parse input");

    let split: Vec<&str> = inp.split(' ').collect();
    let (start_s, end_s): (&str, &str) = split[0].split_once('-').expect("split - fail");
    let (start, end): (usize, usize) = (
        start_s.parse().expect("parse as usize fail"),
        end_s.parse().expect("parse as usize fail"),
    );
    let needle: char = split[1].chars().next().expect("needle empty");
    let pwd = split[2];

    Password {
        start,
        end,
        needle,
        pwd: pwd.to_string(),
    }
}

fn is_valid_part1(input: &Password) -> bool {
    let count = input.pwd.chars().filter(|ch| ch == &input.needle).count();
    count >= input.start && count <= input.end
}

fn is_valid_part2(input: &Password) -> bool {
    let chars: Vec<char> = input.pwd.chars().collect();

    let start_valid = chars[input.start - 1] == input.needle;
    let end_valid = chars[input.end - 1] == input.needle;

    (start_valid || end_valid) && !(start_valid && end_valid)
}

fn parse(raw_inp: &str) -> Vec<Password> {
    raw_inp
        .trim()
        .split('\n')
        .map(str::trim)
        .map(parse_single)
        .collect()
}

fn calculate(data: &[Password]) -> (usize, usize) {
    let p1 = data.iter().filter(|e| is_valid_part1(e)).count();
    let p2 = data.iter().filter(|e| is_valid_part2(e)).count();
    (p1, p2)
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);

    let (p1, p2) = calculate(&data);

    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2020_02");
    const REAL_DATA: &str = include_str!("../../inputs/real/2020_02");

    #[test]
    fn test_p1_examples() {
        assert_eq!(is_valid_part1(&parse_single(&"1-3 a: abcde")), true);
        assert_eq!(is_valid_part1(&parse_single(&"1-3 b: cdefg")), false);
        assert_eq!(is_valid_part1(&parse_single(&"2-9 c: ccccccccc")), true);
    }

    #[test]
    fn test_p2_examples() {
        assert_eq!(is_valid_part2(&parse_single(&"1-3 a: abcde")), true);
        assert_eq!(is_valid_part2(&parse_single(&"1-3 b: cdefg")), false);
        assert_eq!(is_valid_part2(&parse_single(&"2-9 c: ccccccccc")), false);
    }

    #[test]
    fn test_example() {
        let (p1, p2) = calculate(&parse(EXAMPLE_DATA));
        assert_eq!(p1, 2);
        assert_eq!(p2, 1);
    }

    #[test]
    fn test_real() {
        let (p1, p2) = calculate(&parse(REAL_DATA));
        assert_eq!(p1, 542);
        assert_eq!(p2, 360);
    }
}
