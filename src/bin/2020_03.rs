use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: String) -> Vec<Vec<bool>> {
    raw_inp
        .trim()
        .split('\n')
        .map(|line| line.trim())
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn count_trees(data: &Vec<Vec<bool>>, down: usize, right: usize) -> i64 {
    let x_size = data[0].len();

    let mut trees = 0;

    for steps in 0..(data.len() / down) {
        let y = steps * down;
        let x = (right * steps) % x_size;

        if data[y][x] {
            trees += 1;
        }
    }

    trees
}

fn main() {
    let args = Cli::parse();

    let raw_inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(raw_inp);

    let p1 = count_trees(&data, 1, 3);
    let p2 = p1
        * count_trees(&data, 1, 1)
        * count_trees(&data, 1, 5)
        * count_trees(&data, 1, 7)
        * count_trees(&data, 2, 1);

    println!("{}\n{}", p1, p2);
}

#[test]
fn test_example() {
    let raw_data = "..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#
        ";

    assert_eq!(count_trees(&parse(raw_data.to_string()), 1, 1), 2);
    assert_eq!(count_trees(&parse(raw_data.to_string()), 1, 3), 7);
    assert_eq!(count_trees(&parse(raw_data.to_string()), 1, 5), 3);
    assert_eq!(count_trees(&parse(raw_data.to_string()), 1, 7), 4);
    assert_eq!(count_trees(&parse(raw_data.to_string()), 2, 1), 2);
}
