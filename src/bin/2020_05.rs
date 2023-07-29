use clap::Parser;
use fnv::FnvHashSet;
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

fn main() {
    let args = Cli::parse();

    let raw_inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&raw_inp);

    let mut max_seat = u64::MIN;
    let mut min_seat = u64::MAX;

    let mut occupied_seats: FnvHashSet<u64> = FnvHashSet::default();

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

    println!("{}\n{}", p1, p2);
}

#[test]
fn test_get_seat_id() {
    assert_eq!(get_seat_id(&"FBFBBFFRLR"), 357);
    assert_eq!(get_seat_id(&"BFFFBBFRRR"), 567);
    assert_eq!(get_seat_id(&"FFFBBBFRRR"), 119);
    assert_eq!(get_seat_id(&"BBFFBBFRLL"), 820);
}
