use ahash::AHashSet;
use clap::Parser;
use std::cmp::{max, min};
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> Vec<&str> {
    raw_inp.trim().split('\n').collect()
}

fn get_seat_id(seat: &str) -> u64 {
    let mut id: u64 = 0;

    let chars: Vec<char> = seat.chars().collect();

    for (i, item) in chars.iter().enumerate().take(10) {
        if item == &'B' || item == &'R' {
            id += 1 << (9 - i);
        }
    }

    id
}

fn calculate(data: &[&str]) -> (u64, u64) {
    let mut max_seat = u64::MIN;
    let mut min_seat = u64::MAX;

    let mut occupied_seats: AHashSet<u64> = AHashSet::default();

    for item in data.iter() {
        if item.len() != 10 {
            panic!("invalid data length");
        }
        let seat_id = get_seat_id(item);
        max_seat = max(max_seat, seat_id);
        min_seat = min(min_seat, seat_id);
        occupied_seats.insert(seat_id);
    }

    let p1 = max_seat;
    let mut p2 = 0;

    for s in (min_seat + 1)..max_seat {
        if !occupied_seats.contains(&s) {
            p2 = s;
            break;
        }
    }

    (p1, p2)
}

fn main() {
    let args = Cli::parse();

    let raw_inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&raw_inp);

    let (p1, p2) = calculate(&data);

    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_DATA: &str = include_str!("../../inputs/real/2020_05");

    #[test]
    fn test_get_seat_id() {
        assert_eq!(get_seat_id(&"FBFBBFFRLR"), 357);
        assert_eq!(get_seat_id(&"BFFFBBFRRR"), 567);
        assert_eq!(get_seat_id(&"FFFBBBFRRR"), 119);
        assert_eq!(get_seat_id(&"BBFFBBFRLL"), 820);
    }

    #[test]
    fn test_real() {
        let (p1, p2) = calculate(&parse(REAL_DATA));
        assert_eq!(p1, 890);
        assert_eq!(p2, 651);
    }
}
