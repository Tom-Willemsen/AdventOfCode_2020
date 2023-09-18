use ahash::{AHashSet, AHashMap};
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> AHashSet<(i32, i32)> {
    let mut data: AHashSet<(i32, i32)> = AHashSet::default();
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
                    data.insert((column as i32, row as i32));
                });
        });

    data
}

type Point3 = (i32, i32, i32);
type Point4 = (i32, i32, i32, i32);
type PointSet3 = AHashSet<Point3>;
type PointSet4 = AHashSet<Point4>;

fn build_p1_activity_map(frame: &PointSet3) -> AHashMap<Point3, usize> {
    let mut activity_map: AHashMap<Point3, usize> = AHashMap::with_capacity(frame.len() * 40);

    for &(x, y, z) in frame {
        for xd in -1..=1 {
            for yd in -1..=1 {
                for zd in -1..=1 {
                    if xd == 0 && yd == 0 && zd == 0 {
                        continue;
                    }
                    
                    let inc = if zd == -1 && z == 1 {
                        2
                    } else {
                        1
                    };
                    
                    *activity_map.entry((x+xd, y+yd, z+zd)).or_insert(0) += inc;
                }
            }
        }
    }
    
    activity_map
}

fn build_p2_activity_map(frame: &PointSet4) -> AHashMap<Point4, usize> {
    let mut activity_map: AHashMap<Point4, usize> = AHashMap::with_capacity(frame.len() * 40);

    for &(x, y, z, w) in frame {
        for xd in -1..=1 {
            for yd in -1..=1 {
                for zd in -1..=1 {
                    for wd in -1..=1 {
                        if xd == 0 && yd == 0 && zd == 0 && wd == 0 {
                            continue;
                        }
                        
                        let inc = if (wd == -1 && w == 1) && (zd == -1 && z == 1) {
                            4
                        } else if (wd == -1 && w == 1) || (zd == -1 && z == 1) {
                            2
                        } else {
                            1
                        };
                        
                        *activity_map.entry((x+xd, y+yd, z+zd, w+wd)).or_insert(0) += inc;
                    }
                }
            }
        }
    }
    
    activity_map
}

fn turn_part1(current: &PointSet3) -> PointSet3 {
    build_p1_activity_map(current).into_iter()
        .filter(|&((_, _, z), _)| {
            z >= 0
        })
        .filter(|&(k, v)| {
            v == 3 || (v==2 && current.contains(&k))
        })
        .map(|(k, _)| k)
        .collect()
}

fn turn_part2(current: &PointSet4) -> PointSet4 {
    build_p2_activity_map(current).into_iter()
        .filter(|&((_, _, z, w), _)| {
            z >= 0 && w >= 0
        })
        .filter(|&(k, v)| {
            v == 3 || (v==2 && current.contains(&k))
        })
        .map(|(k, _)| k)
        .collect()
}

fn calculate_p1(data: &AHashSet<(i32, i32)>) -> usize {
    let mut current_frame: PointSet3 = AHashSet::with_capacity(data.len());

    for (x, y) in data {
        current_frame.insert((*x, *y, 0));
    }

    for _ in 0..6 {
        current_frame = turn_part1(&current_frame);
    }

    current_frame
        .into_iter()
        .map(|(_, _, z)| if z == 0 { 1 } else { 2 })
        .sum()
}

fn calculate_p2(data: &AHashSet<(i32, i32)>) -> usize {
    let mut current_frame: PointSet4 = AHashSet::with_capacity(data.len());

    for (x, y) in data {
        current_frame.insert((*x, *y, 0, 0));
    }

    for _ in 0..6 {
        current_frame = turn_part2(&current_frame);
    }

    current_frame
        .into_iter()
        .map(|(_, _, z, w)| {
            if z == 0 && w == 0 {
                1
            } else if z == 0 || w == 0 {
                2
            } else {
                4
            }
        })
        .sum()
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
