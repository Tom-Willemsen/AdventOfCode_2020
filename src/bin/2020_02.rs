use clap::Parser;
use sscanf::sscanf;
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

fn parse(inp: &str) -> Password {
    let parsed = sscanf!(inp, "{usize}-{usize} {char}: {str}");
    let (start, end, needle, pwd) = parsed.expect("unable to parse input");
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

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let inputs: Vec<Password> = inp.trim().split('\n').map(str::trim).map(parse).collect();

    let p1 = inputs.iter().filter(|e| is_valid_part1(e)).count();
    let p2 = inputs.iter().filter(|e| is_valid_part2(e)).count();

    println!("{}\n{}", p1, p2);
}

#[test]
fn test_p1_examples() {
    assert_eq!(is_valid_part1(&parse(&"1-3 a: abcde")), true);
    assert_eq!(is_valid_part1(&parse(&"1-3 b: cdefg")), false);
    assert_eq!(is_valid_part1(&parse(&"2-9 c: ccccccccc")), true);
}

#[test]
fn test_p2_examples() {
    assert_eq!(is_valid_part2(&parse(&"1-3 a: abcde")), true);
    assert_eq!(is_valid_part2(&parse(&"1-3 b: cdefg")), false);
    assert_eq!(is_valid_part2(&parse(&"2-9 c: ccccccccc")), false);
}
