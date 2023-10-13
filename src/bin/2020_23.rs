use clap::Parser;
use mimalloc::MiMalloc;
use std::fs;

#[global_allocator]
static ALLOCATOR: MiMalloc = MiMalloc;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> Vec<u32> {
    raw_inp
        .trim()
        .chars()
        .map(|x| x.to_digit(10).expect("invalid digit"))
        .collect()
}

fn simulate<const TURNS: u64>(deck: &[u32]) -> Vec<u32> {
    let mut next_cups = vec![0u32; deck.len()];

    for (idx, &itm) in deck.iter().enumerate() {
        let itm: usize = itm.try_into().unwrap();
        next_cups[itm - 1] = deck[(idx + 1).rem_euclid(deck.len())];
    }

    let max: u32 = *deck.iter().max().unwrap();

    let mut current_cup = deck[0];

    for _ in 0..TURNS {
        let c1 = next_cups[(current_cup - 1) as usize];
        let c2 = next_cups[(c1 - 1) as usize];
        let c3 = next_cups[(c2 - 1) as usize];
        let next_cup = next_cups[(c3 - 1) as usize];

        let mut destination = (current_cup - 1).rem_euclid(next_cups.len() as u32);

        while destination == c1 || destination == c2 || destination == c3 || destination < 1 {
            if destination <= 1 {
                destination = max;
            } else {
                destination -= 1;
            }
        }

        next_cups[(current_cup - 1) as usize] = next_cup;
        next_cups[(c3 - 1) as usize] = next_cups[(destination - 1) as usize];
        next_cups[(destination - 1) as usize] = c1;

        current_cup = next_cup;
    }

    next_cups
}

fn calculate_p1(data: &[u32]) -> u64 {
    let next_cups = simulate::<100>(data);
    let mut ans: u64 = u64::from(next_cups[0]);
    let mut next = next_cups[0];

    while next_cups[next as usize - 1] != 1 {
        ans = ans * 10 + u64::from(next_cups[next as usize - 1]);
        next = next_cups[next as usize - 1];
    }

    ans
}

fn calculate_p2(data: &[u32]) -> u64 {
    let mut new_data: Vec<u32> = data.into();
    new_data.extend(10..=1000000);
    let next_cups = simulate::<10000000>(&new_data);
    let c1 = next_cups[0];
    let c2 = next_cups[c1 as usize - 1];

    u64::from(c1) * u64::from(c2)
}

fn main() {
    let args = Cli::parse();
    let raw_inp = fs::read_to_string(args.input).expect("can't open input file");
    let deck = parse(&raw_inp);
    let p1 = calculate_p1(&deck);
    let p2 = calculate_p2(&deck);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2020_23");
    const REAL_DATA: &str = include_str!("../../inputs/real/2020_23");

    #[test]
    fn test_p1_example() {
        let data = parse(EXAMPLE_DATA);
        assert_eq!(calculate_p1(&data), 67384529);
    }

    #[test]
    fn test_p1_real() {
        let data = parse(REAL_DATA);
        assert_eq!(calculate_p1(&data), 69852437);
    }

    #[test]
    fn test_p2_example() {
        let data = parse(EXAMPLE_DATA);
        assert_eq!(calculate_p2(&data), 149245887792);
    }

    #[test]
    fn test_p2_real() {
        let data = parse(REAL_DATA);
        assert_eq!(calculate_p2(&data), 91408386135);
    }
}
