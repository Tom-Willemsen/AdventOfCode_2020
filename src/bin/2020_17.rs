use ahash::AHashSet;
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> AHashSet<(i64, i64)> {
    let mut data: AHashSet<(i64, i64)> = AHashSet::default();
    raw_inp
        .trim()
        .split('\n')
        .map(|line| line.bytes().collect::<Vec<u8>>())
        .enumerate()
        .for_each(|(row, line)| {
            line.iter()
                .enumerate()
                .filter(|&(_, &byte)| byte == b'#')
                .for_each(|(column, _)| {
                    data.insert((column as i64, row as i64));
                });
        });

    data
}

fn count_active_neighbours_p1(frame: &AHashSet<(i64, i64, i64)>, x: i64, y: i64, z: i64) -> usize {
    let mut count = 0;
    for xd in -1..=1 {
        for yd in -1..=1 {
            for zd in -1..=1 {
                if !(xd == 0 && yd == 0 && zd == 0) && frame.contains(&(x + xd, y + yd, z + zd)) {
                    count += 1;

                    if count > 3 {
                        return count;
                    }
                }
            }
        }
    }
    count
}

fn count_active_neighbours_p2(
    frame: &AHashSet<(i64, i64, i64, i64)>,
    x: i64,
    y: i64,
    z: i64,
    w: i64,
) -> usize {
    let mut count = 0;
    for xd in -1..=1 {
        for yd in -1..=1 {
            for zd in -1..=1 {
                for wd in -1..=1 {
                    if !(xd == 0 && yd == 0 && zd == 0 && wd == 0)
                        && frame.contains(&(x + xd, y + yd, z + zd, w + wd))
                    {
                        count += 1;

                        if count > 3 {
                            return count;
                        }
                    }
                }
            }
        }
    }
    count
}

fn turn_part1(current: &AHashSet<(i64, i64, i64)>) -> AHashSet<(i64, i64, i64)> {
    let mut next_frame: AHashSet<(i64, i64, i64)> = AHashSet::default();

    for (x, y, z) in current {
        for xd in -1..=1 {
            for yd in -1..=1 {
                for zd in -1..=1 {
                    next_frame.insert((x + xd, y + yd, z + zd));
                }
            }
        }
    }

    next_frame.retain(|&(x, y, z)| {
        let active_neighbours = count_active_neighbours_p1(current, x, y, z);
        let currently_active = current.contains(&(x, y, z));

        (active_neighbours == 3) || (currently_active && active_neighbours == 2)
    });

    next_frame
}

fn turn_part2(current: &AHashSet<(i64, i64, i64, i64)>) -> AHashSet<(i64, i64, i64, i64)> {
    let mut next_frame: AHashSet<(i64, i64, i64, i64)> = AHashSet::default();

    for (x, y, z, w) in current {
        for xd in -1..=1 {
            for yd in -1..=1 {
                for zd in -1..=1 {
                    for wd in -1..=1 {
                        next_frame.insert((x + xd, y + yd, z + zd, w + wd));
                    }
                }
            }
        }
    }

    next_frame.retain(|&(x, y, z, w)| {
        let active_neighbours = count_active_neighbours_p2(current, x, y, z, w);
        let currently_active = current.contains(&(x, y, z, w));

        (active_neighbours == 3) || (currently_active && active_neighbours == 2)
    });

    next_frame
}

fn calculate_p1(data: &AHashSet<(i64, i64)>) -> usize {
    let mut current_frame: AHashSet<(i64, i64, i64)> = AHashSet::default();

    for (x, y) in data {
        current_frame.insert((*x, *y, 0));
    }

    for _ in 0..6 {
        current_frame = turn_part1(&current_frame);
    }

    current_frame.len()
}

fn calculate_p2(data: &AHashSet<(i64, i64)>) -> usize {
    let mut current_frame: AHashSet<(i64, i64, i64, i64)> = AHashSet::default();

    for (x, y) in data {
        current_frame.insert((*x, *y, 0, 0));
    }

    for _ in 0..6 {
        current_frame = turn_part2(&current_frame);
    }

    current_frame.len()
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2020_17");

    const REAL_DATA: &str = include_str!("../../inputs/real/2020_17");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA)), 112);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&parse(EXAMPLE_DATA)), 848);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(REAL_DATA)), 242);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(REAL_DATA)), 2292);
    }
}
