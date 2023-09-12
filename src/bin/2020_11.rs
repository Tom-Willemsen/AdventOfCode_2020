use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> Vec<Vec<u8>> {
    raw_inp
        .trim()
        .split('\n')
        .map(|x| x.bytes().collect())
        .collect()
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
    data: &Vec<Vec<u8>>,
    x: usize,
    y: usize,
    x_offset: isize,
    y_offset: isize,
) -> Option<(usize, usize)> {
    let new_x = x.checked_add_signed(x_offset)?;
    let new_y = y.checked_add_signed(y_offset)?;

    if new_y >= data.len() || new_x >= data[new_y].len() {
        return None;
    }
    Some((new_x, new_y))
}

fn get_visible_seats_p1(data: &Vec<Vec<u8>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut visible_seats: Vec<(usize, usize)> = Vec::with_capacity(8);

    for (xd, yd) in DIRECTIONS {
        if let Some((nx, ny)) = validate_seat_offset(data, x, y, xd, yd) {
            visible_seats.push((nx, ny));
        }
    }

    visible_seats
}

fn get_visible_seats_p2(data: &Vec<Vec<u8>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut visible_seats: Vec<(usize, usize)> = Vec::with_capacity(8);

    for (xd, yd) in DIRECTIONS {
        let mut distance = 1;

        while let Some((nx, ny)) = validate_seat_offset(data, x, y, xd * distance, yd * distance) {
            if data[ny][nx] == FLOOR {
                distance += 1;
            } else {
                visible_seats.push((nx, ny));
                break;
            }
        }
    }

    visible_seats
}

type VisibleSeatsType = fn(&Vec<Vec<u8>>, usize, usize) -> Vec<(usize, usize)>;

fn simulate(
    data: &Vec<Vec<u8>>,
    visible_seats: VisibleSeatsType,
    occupancy_tolerance: usize,
) -> usize {
    let mut old_data: Vec<Vec<u8>> = data.clone();
    let mut new_data: Vec<Vec<u8>> = data.clone();

    loop {
        let mut any_changed = false;

        for y in 0..data.len() {
            for x in 0..data[y].len() {
                if old_data[y][x] != FLOOR {
                    let occupied = visible_seats(data, x, y)
                        .iter()
                        .map(|&(nx, ny)| old_data[ny][nx])
                        .filter(|&item| item == OCCUPIED)
                        .count();

                    if occupied == 0 && old_data[y][x] == EMPTY {
                        new_data[y][x] = OCCUPIED;
                        any_changed = true;
                    } else if occupied >= occupancy_tolerance && old_data[y][x] == OCCUPIED {
                        new_data[y][x] = EMPTY;
                        any_changed = true;
                    }
                }
            }
        }

        if !any_changed {
            break;
        } else {
            old_data = new_data.clone();
        }
    }

    new_data
        .iter()
        .map(|x| x.iter().filter(|&s| s == &OCCUPIED).count())
        .sum()
}

fn calculate_p1(data: &Vec<Vec<u8>>) -> usize {
    simulate(data, get_visible_seats_p1, 4)
}

fn calculate_p2(data: &Vec<Vec<u8>>) -> usize {
    simulate(data, get_visible_seats_p2, 5)
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
