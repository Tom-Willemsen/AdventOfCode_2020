use ahash::AHashSet;
use clap::Parser;
use itertools::Itertools;
use ndarray::Array2;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> Vec<(i8, i8)> {
    raw_inp
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(parse_line)
        .collect()
}

fn parse_line(raw_inp: &str) -> (i8, i8) {
    let mut pos = (0, 0);
    let mut index = 0;

    while index < raw_inp.len() {
        let s = &raw_inp[index..];
        if s.starts_with('e') {
            pos = (pos.0 + 1, pos.1);
            index += 1;
        } else if s.starts_with('w') {
            pos = (pos.0 - 1, pos.1);
            index += 1;
        } else if s.starts_with("ne") {
            pos = (pos.0, pos.1 + 1);
            index += 2;
        } else if s.starts_with("nw") {
            pos = (pos.0 - 1, pos.1 + 1);
            index += 2;
        } else if s.starts_with("se") {
            pos = (pos.0 + 1, pos.1 - 1);
            index += 2;
        } else if s.starts_with("sw") {
            pos = (pos.0, pos.1 - 1);
            index += 2;
        } else {
            panic!("unparseable: {:?}", s);
        }
    }

    pos
}

fn calculate_starting_map(data: &[(i8, i8)]) -> AHashSet<(i8, i8)> {
    let mut map: AHashSet<(i8, i8)> = AHashSet::default();

    for pos in data.iter() {
        if !map.insert(*pos) {
            map.remove(pos);
        }
    }

    map
}

fn calculate_p1(data: &AHashSet<(i8, i8)>) -> usize {
    data.len()
}

fn neighbours_of(loc: &(usize, usize)) -> [(usize, usize); 6] {
    [
        (loc.0 - 1, loc.1),
        (loc.0 - 1, loc.1 + 1),
        (loc.0, loc.1 - 1),
        (loc.0, loc.1 + 1),
        (loc.0 + 1, loc.1 - 1),
        (loc.0 + 1, loc.1),
    ]
}

fn new_p2_frame(data: Array2<u8>) -> Array2<u8> {
    let mut pts = data
        .indexed_iter()
        .filter(|&(_, &v)| v == 1)
        .map(|(idx, _)| (idx.0, idx.1))
        .collect::<Vec<_>>();

    let (min_x, max_x) = pts.iter().map(|pt| pt.0).minmax().into_option().unwrap();
    let (min_y, max_y) = pts.iter().map(|pt| pt.1).minmax().into_option().unwrap();

    pts.iter_mut()
        .for_each(|val| *val = (val.0 - min_x + 1, val.1 - min_y + 1));

    let mut neighbours = Array2::zeros((max_x - min_x + 3, max_y - min_y + 3));

    pts.iter().flat_map(neighbours_of).for_each(|pt| {
        neighbours[pt] += 1;
    });

    pts.into_iter().for_each(|pt| {
        neighbours[pt] += 100;
    });

    neighbours.mapv_inplace(|n| if n == 2 || n == 101 || n == 102 { 1 } else { 0 });

    neighbours
}

fn calculate_p2(data: &AHashSet<(i8, i8)>) -> usize {
    let (min_x, max_x) = data.iter().map(|pt| pt.0).minmax().into_option().unwrap();
    let (min_y, max_y) = data.iter().map(|pt| pt.1).minmax().into_option().unwrap();

    let mut frame = Array2::zeros((
        usize::try_from(max_x - min_x + 1).unwrap(),
        usize::try_from(max_y - min_y + 1).unwrap(),
    ));

    data.iter()
        .map(|pt| {
            (
                usize::try_from(pt.0 - min_x).unwrap(),
                usize::try_from(pt.1 - min_y).unwrap(),
            )
        })
        .for_each(|pt| frame[pt] = 1);

    for _ in 0..100 {
        frame = new_p2_frame(frame);
    }

    frame.into_iter().filter(|&x| x == 1).count()
}

fn main() {
    let args = Cli::parse();
    let raw_inp = fs::read_to_string(args.input).expect("can't open input file");
    let data = parse(&raw_inp);
    let map = calculate_starting_map(&data);
    let p1 = calculate_p1(&map);
    let p2 = calculate_p2(&map);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2020_24");
    const REAL_DATA: &str = include_str!("../../inputs/real/2020_24");

    #[test]
    fn test_p1_example() {
        let data = parse(EXAMPLE_DATA);
        let map = calculate_starting_map(&data);
        assert_eq!(calculate_p1(&map), 10);
    }

    #[test]
    fn test_p1_real() {
        let data = parse(REAL_DATA);
        let map = calculate_starting_map(&data);
        assert_eq!(calculate_p1(&map), 427);
    }

    #[test]
    fn test_p2_example() {
        let data = parse(EXAMPLE_DATA);
        let map = calculate_starting_map(&data);
        assert_eq!(calculate_p2(&map), 2208);
    }

    #[test]
    fn test_p2_real() {
        let data = parse(REAL_DATA);
        let map = calculate_starting_map(&data);
        assert_eq!(calculate_p2(&map), 3837);
    }
}
