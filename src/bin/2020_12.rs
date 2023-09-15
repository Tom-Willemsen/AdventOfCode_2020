use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> Vec<(u8, i64)> {
    raw_inp.trim()
        .split('\n')
        .map(|s| (s.bytes().next().expect("empty instruction"), s[1..].parse().expect("parse failed")))
        .collect()
}

fn calculate_p1(data: &Vec<(u8, i64)>) -> i64 {
    let mut x_pos: i64 = 0;
    let mut y_pos: i64 = 0;
    let mut dir: i64 = 90;
    
    for (command, n) in data {
        match command {
            b'N' => y_pos += n,
            b'S' => y_pos -= n,
            b'E' => x_pos += n,
            b'W' => x_pos -= n,
            b'L' => dir = (dir - n).rem_euclid(360),
            b'R' => dir = (dir + n).rem_euclid(360),
            b'F' => {
                match dir {
                    0 => y_pos += n,
                    90 => x_pos += n,
                    180 => y_pos -= n,
                    270 => x_pos -= n,
                    _ => panic!("invalid dir"),
                }
            },
            _ => panic!("invalid command"),
        }
    }
    
    x_pos.abs() + y_pos.abs()
}

fn calculate_p2(data: &Vec<(u8, i64)>) -> i64 {
    let mut wp_x: i64 = 10;
    let mut wp_y: i64 = 1;
    let mut ship_x: i64 = 0;
    let mut ship_y: i64 = 0;
    
    for (command, n) in data {
        match command {
            b'N' => wp_y += n,
            b'S' => wp_y -= n,
            b'E' => wp_x += n,
            b'W' => wp_x -= n,
            b'L' => {
                (wp_x, wp_y) = match n.rem_euclid(360) {
                    0 => (wp_x, wp_y),
                    90 => (-wp_y, wp_x),
                    180 => (-wp_x, -wp_y),
                    270 => (wp_y, -wp_x),
                    _ => panic!("invalid dir"),
                };
            },
            b'R' => {
                (wp_x, wp_y) = match n.rem_euclid(360) {
                    0 => (wp_x, wp_y),
                    90 => (wp_y, -wp_x),
                    180 => (-wp_x, -wp_y),
                    270 => (-wp_y, wp_x),
                    _ => panic!("invalid dir"),
                };
            },
            b'F' => {
                ship_x += wp_x * n;
                ship_y += wp_y * n;
            },
            _ => panic!("invalid command"),
        }
    }
    
    ship_x.abs() + ship_y.abs()
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

    const TEST_DATA: &str = include_str!("../../inputs/examples/2020_12");
    const REAL_DATA: &str = include_str!("../../inputs/real/2020_12");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(TEST_DATA)), 25);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&parse(TEST_DATA)), 286);
    }
    
    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(REAL_DATA)), 582);
    }
    
    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(REAL_DATA)), 52069);
    }
}
 
