use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> Vec<Vec<char>> {
    raw_inp
        .trim()
        .split('\n')
        .map(|x| x.chars().collect())
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

fn validate_seat_offset(data: &Vec<Vec<char>>, x: usize, y: usize, x_offset: isize, y_offset: isize) -> Option<(usize, usize)> {
    let new_x = x.checked_add_signed(x_offset)?;
    let new_y = y.checked_add_signed(y_offset)?;
    
    if new_y >= data.len() || new_x >= data[new_y].len() {
        return None;
    }
    Some((new_x, new_y))
}

fn get_visible_seats_p1(data: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut visible_seats: Vec<(usize, usize)> = Vec::with_capacity(8);
    
    for (xd, yd) in DIRECTIONS {
        if let Some((nx, ny)) = validate_seat_offset(data, x, y, xd, yd) {
            visible_seats.push((nx, ny));
        }
    }
    
    visible_seats
}

fn get_visible_seats_p2(data: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut visible_seats: Vec<(usize, usize)> = Vec::with_capacity(8);
    
    for (xd, yd) in DIRECTIONS {
        let mut distance = 1;
        
        while let Some((nx, ny)) = validate_seat_offset(data, x, y, xd * distance, yd * distance) {
            if data[ny][nx] == '.' {
                distance += 1;
            } else {
                visible_seats.push((nx, ny));
                break;
            }
        }
    }
    
    visible_seats
}

type VisibleSeatsType = fn(&Vec<Vec<char>>, usize, usize) -> Vec<(usize, usize)>;

fn simulate(data: &Vec<Vec<char>>, visible_seats: VisibleSeatsType, occupancy_tolerance: usize) -> usize {
    let mut old_data: Vec<Vec<char>> = data.clone();
    let mut new_data: Vec<Vec<char>> = data.clone();
    
    loop {
        let mut any_changed = false;
        
        for y in 0..data.len() {
            for x in 0..data[y].len() {
                if old_data[y][x] != '.' {
                    let occupied = visible_seats(data, x, y)
                        .iter()
                        .map(|&(nx, ny)| old_data[ny][nx])
                        .filter(|&item| item == '#')
                        .count();
                    
                    if occupied == 0 && old_data[y][x] == 'L' {
                        new_data[y][x] = '#';
                        any_changed = true;
                    } else if occupied >= occupancy_tolerance && old_data[y][x] == '#' {
                        new_data[y][x] = 'L';
                        any_changed = true;
                    } else {
                        new_data[y][x] = old_data[y][x];
                    }
                }
            }
        }
        
        if !any_changed {
            break;
        } else {
            new_data = std::mem::replace(&mut old_data, new_data);
        }
    }
    
    new_data.iter().map(|x| x.iter().filter(|&s| s == &'#').count()).sum()
}

fn calculate_p1(data: &Vec<Vec<char>>) -> usize {
    simulate(data, get_visible_seats_p1, 4)
}

fn calculate_p2(data: &Vec<Vec<char>>) -> usize {
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

// #[cfg(test)]
// const TEST_DATA: [u64; 20] = [
//     35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
// ];
// 
// #[test]
// fn test_p1_example() {
//     assert_eq!(calculate_p1(&TEST_DATA, 5), 127);
// }
// 
// #[test]
// fn test_p2_example() {
//     assert_eq!(calculate_p2(&TEST_DATA, 127), 62);
// }
