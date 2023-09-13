use clap::Parser;
use std::fs;
use ndarray::{Array2, Zip};

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> Array2<u8> {
    let rows = raw_inp.trim().split('\n').count();
    let columns = raw_inp.trim().split('\n').map(|x| x.len()).max().unwrap();
    let v: Vec<u8> = raw_inp
        .trim()
        .split('\n')
        .map(|x| x.bytes().into_iter())
        .flatten()
        .collect();
        
    Array2::from_shape_vec([rows, columns], v).unwrap()
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

const FLOOR: u8 = b'.';
const EMPTY: u8 = b'L';
const OCCUPIED: u8 = b'#';

fn validate_seat_offset(
    data: &Array2<u8>,
    x: usize,
    y: usize,
    x_offset: isize,
    y_offset: isize,
) -> Option<(usize, usize)> {
    let new_x = x.checked_add_signed(x_offset)?;
    let new_y = y.checked_add_signed(y_offset)?;

    if let Some(_) = data.get((new_y, new_x)) {
        return Some((new_x, new_y));
    }
    None
}

fn visible_occupied_seats_p1(data: &Array2<u8>, x: usize, y: usize) -> usize {
    DIRECTIONS.iter()
        .filter_map(|&(xd, yd)| validate_seat_offset(data, x, y, xd, yd))
        .filter(|&(nx, ny)| data[(ny, nx)] == OCCUPIED)
        .count()
}

fn visible_occupied_seats_p2(data: &Array2<u8>, x: usize, y: usize) -> usize {
    DIRECTIONS.iter()
        .filter(|&(xd, yd)| {
            let mut distance = 1;
            while let Some((nx, ny)) = validate_seat_offset(data, x, y, xd * distance, yd * distance) {
                if data[(ny, nx)] == FLOOR {
                    distance += 1;
                } else {
                    return data[(ny, nx)] == OCCUPIED;
                }
            }
            false
        })
        .count()
}

type VisibleSeatsType = fn(&Array2<u8>, usize, usize) -> usize;

fn simulate(
    data: &Array2<u8>,
    visible_seats: VisibleSeatsType,
    occupancy_tolerance: usize,
) -> usize {
    let mut old_data: Array2<u8>;
    let mut new_data: Array2<u8> = data.clone();
    let mut any_changed = true;

    while any_changed {
        any_changed = false;
        
        old_data = new_data.clone();
        
        old_data
            .indexed_iter()
            .filter(|(_, &old)| old != FLOOR)
            .for_each(|((y, x), &old)| {
                let visible_occupied = visible_seats(&old_data, x, y);
                if old == EMPTY && visible_occupied == 0 {
                    any_changed = true;
                    new_data[(y, x)] = OCCUPIED;
                } else if old == OCCUPIED && visible_occupied >= occupancy_tolerance  {
                    any_changed = true;
                    new_data[(y, x)] = EMPTY;
                } else {
                    new_data[(y, x)] = old;
                }
            });
    }

    new_data
        .iter()
        .filter(|&s| s == &OCCUPIED)
        .count()
}

fn calculate_p1(data: &Array2<u8>) -> usize {
    simulate(data, visible_occupied_seats_p1, 4)
}

fn calculate_p2(data: &Array2<u8>) -> usize {
    simulate(data, visible_occupied_seats_p2, 5)
}

fn main() {
    let args = Cli::parse();
    let raw_inp = fs::read_to_string(args.input).expect("can't open input file");
    let data = parse(&raw_inp);
    let p1 = calculate_p1(&data);
    let p2 = calculate_p2(&data);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = include_str!("../../inputs/examples/2020_11");
    const REAL_DATA: &str = include_str!("../../inputs/real/2020_11");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(TEST_DATA)), 37);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&parse(TEST_DATA)), 26);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(REAL_DATA)), 2289);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(REAL_DATA)), 2059);
    }
}
