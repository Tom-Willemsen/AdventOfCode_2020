use clap::Parser;
use ndarray::{Array2, Zip};
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> Array2<u8> {
    let rows = raw_inp.trim().split('\n').count();
    let columns = raw_inp.trim().split('\n').map(|x| x.len()).max().unwrap();
    let v: Vec<u8> = raw_inp.trim().split('\n').flat_map(|x| x.bytes()).collect();

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

    if data.get((new_y, new_x)).is_some() {
        return Some((new_y, new_x));
    }
    None
}

fn visible_occupied_seats_p1(data: &Array2<u8>, x: usize, y: usize) -> Vec<(usize, usize)> {
    DIRECTIONS
        .iter()
        .filter_map(|&(xd, yd)| validate_seat_offset(data, x, y, xd, yd))
        .collect()
}

fn visible_occupied_seats_p2(data: &Array2<u8>, x: usize, y: usize) -> Vec<(usize, usize)> {
    DIRECTIONS
        .iter()
        .filter_map(|&(xd, yd)| {
            let mut distance = 1;
            while let Some((ny, nx)) =
                validate_seat_offset(data, x, y, xd * distance, yd * distance)
            {
                if data[(ny, nx)] == FLOOR {
                    distance += 1;
                } else {
                    return Some((ny, nx));
                }
            }
            None
        })
        .collect()
}

type VisibleSeatsType = fn(&Array2<u8>, usize, usize) -> Vec<(usize, usize)>;

fn simulate(
    data: &Array2<u8>,
    visible_seats: VisibleSeatsType,
    occupancy_tolerance: usize,
) -> usize {
    let mut old_data: Array2<u8> = data.clone();
    let mut new_data: Array2<u8> = data.clone();

    // Precompute visible seats
    let vseats: Array2<Vec<(usize, usize)>> = Array2::from_shape_fn(data.raw_dim(), |(y, x)| {
        if data[(y, x)] != FLOOR {
            visible_seats(data, x, y)
        } else {
            vec![]
        }
    });

    let mut any_changed = true;
    while any_changed {
        std::mem::swap(&mut old_data, &mut new_data);

        Zip::from(&mut new_data)
            .and(&old_data)
            .and(&vseats)
            .for_each(|new, &old, vseats| {
                if old != FLOOR {
                    *new = match old {
                        EMPTY => {
                            let no_visible_occupied = vseats
                                .iter()
                                .all(|&coord| old_data[coord] != OCCUPIED);
                            if no_visible_occupied { OCCUPIED } else { EMPTY }
                        },
                        OCCUPIED => {
                            let many_visible_occupied = vseats
                                .iter()
                                .filter(|&&coord| old_data[coord] == OCCUPIED)
                                .count() >= occupancy_tolerance;
                            if many_visible_occupied { EMPTY } else { OCCUPIED }
                        },
                        _ => old,
                    }
                }
            });

        any_changed = old_data != new_data;
    }

    new_data.iter().filter(|&s| s == &OCCUPIED).count()
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
