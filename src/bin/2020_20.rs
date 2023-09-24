use ahash::{AHashMap, AHashSet};
use clap::Parser;
use ndarray::{s, Array2};
use std::fs;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

#[derive(Debug, Clone)]
struct Tile {
    id: i64,
    arr: Array2<bool>,
}

impl Tile {
    fn clone_rotate(&self) -> Self {
        let mut new_arr = self.arr.slice(s![..,..;-1]);
        new_arr.swap_axes(0, 1);
        Tile {
            id: self.id,
            arr: new_arr.into_owned(),
        }
    }

    fn clone_flip_vertical(&self) -> Self {
        let new_arr = self.arr.slice(s![..;-1,..]);
        Tile {
            id: self.id,
            arr: new_arr.into_owned(),
        }
    }

    fn clone_flip_horizontal(&self) -> Self {
        let new_arr = self.arr.slice(s![..,..;-1]);
        Tile {
            id: self.id,
            arr: new_arr.into_owned(),
        }
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Tile {}

impl Hash for Tile {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let (mut header, data) = s.split_once('\n').ok_or("header split failed")?;

        header = header
            .strip_prefix("Tile ")
            .ok_or("missing header prefix")?
            .strip_suffix(':')
            .ok_or("missing header suffix")?;

        let id = header.parse().or(Err("can't parse id"))?;

        let rows = data.trim().split('\n').count();
        let columns = data.trim().split('\n').map(|x| x.len()).max().unwrap();
        let v: Vec<bool> = data
            .trim()
            .split('\n')
            .flat_map(|x| x.bytes())
            .map(|b| b == b'#')
            .collect();

        let arr = Array2::from_shape_vec([rows, columns], v).or(Err("shape error"))?;

        Ok(Tile { id, arr })
    }
}

fn parse(raw_inp: &str) -> Vec<Tile> {
    raw_inp
        .trim()
        .split("\n\n")
        .map(|tile_str| tile_str.parse().expect("tile failed to parse"))
        .collect()
}

const NEIGHBOURS: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn get_candidate_placements(
    existing_placements: &AHashMap<(i64, i64), Tile>,
) -> AHashSet<(i64, i64)> {
    let mut candidates: AHashSet<(i64, i64)> = AHashSet::default();

    for (x, y) in existing_placements.keys() {
        for (x_diff, y_diff) in NEIGHBOURS {
            let new_x = x + x_diff;
            let new_y = y + y_diff;
            if !existing_placements.contains_key(&(new_x, new_y)) {
                candidates.insert((new_x, new_y));
            }
        }
    }
    candidates
}

fn is_possible_placement(
    existing_placements: &AHashMap<(i64, i64), Tile>,
    proposed_location: (i64, i64),
    tile: &Tile,
) -> bool {
    let (prop_x, prop_y) = proposed_location;

    if let Some(tile_above) = existing_placements.get(&(prop_x, prop_y + 1)) {
        if tile_above.arr.slice(s![-1, ..]) != tile.arr.slice(s![0, ..]) {
            return false;
        }
    }

    if let Some(tile_below) = existing_placements.get(&(prop_x, prop_y - 1)) {
        if tile_below.arr.slice(s![0, ..]) != tile.arr.slice(s![-1, ..]) {
            return false;
        }
    }

    if let Some(tile_left) = existing_placements.get(&(prop_x - 1, prop_y)) {
        if tile_left.arr.slice(s![.., -1]) != tile.arr.slice(s![.., 0]) {
            return false;
        }
    }

    if let Some(tile_right) = existing_placements.get(&(prop_x + 1, prop_y)) {
        if tile_right.arr.slice(s![.., 0]) != tile.arr.slice(s![.., -1]) {
            return false;
        }
    }

    true
}

fn is_any_placement_possible(
    assigned_tiles: &AHashMap<(i64, i64), Tile>,
    proposed_location: (i64, i64),
    tile: &Tile,
) -> Option<Tile> {
    let untransformed_tile = tile.clone();
    if is_possible_placement(assigned_tiles, proposed_location, &untransformed_tile) {
        return Some(untransformed_tile);
    }

    let rotated_tile = tile.clone_rotate();
    if is_possible_placement(assigned_tiles, proposed_location, &rotated_tile) {
        return Some(rotated_tile);
    }

    let vert_flipped_tile = tile.clone_flip_vertical();
    if is_possible_placement(assigned_tiles, proposed_location, &vert_flipped_tile) {
        return Some(vert_flipped_tile);
    }

    let horiz_flipped_tile = tile.clone_flip_horizontal();
    if is_possible_placement(assigned_tiles, proposed_location, &horiz_flipped_tile) {
        return Some(horiz_flipped_tile);
    }

    let all_flipped_tile = vert_flipped_tile.clone_flip_horizontal();
    if is_possible_placement(assigned_tiles, proposed_location, &all_flipped_tile) {
        return Some(all_flipped_tile);
    }

    let rotated_hflip_tile = rotated_tile.clone_flip_horizontal();
    if is_possible_placement(assigned_tiles, proposed_location, &rotated_hflip_tile) {
        return Some(rotated_hflip_tile);
    }

    let rotated_vflip_tile = rotated_tile.clone_flip_vertical();
    if is_possible_placement(assigned_tiles, proposed_location, &rotated_vflip_tile) {
        return Some(rotated_vflip_tile);
    }

    let double_rotated_tile = rotated_tile.clone_rotate();
    if is_possible_placement(assigned_tiles, proposed_location, &double_rotated_tile) {
        return Some(double_rotated_tile);
    }

    let triple_rotated_tile = double_rotated_tile.clone_rotate();
    if is_possible_placement(assigned_tiles, proposed_location, &triple_rotated_tile) {
        return Some(triple_rotated_tile);
    }

    None
}

fn build_image(data: &[Tile]) -> AHashMap<(i64, i64), Tile> {
    let mut assigned_tiles: AHashMap<(i64, i64), Tile> = AHashMap::default();
    let mut unassigned_tiles: AHashSet<Tile> = AHashSet::default();

    assigned_tiles.insert((0, 0), data[0].clone());

    data.iter().skip(1).for_each(|tile| {
        unassigned_tiles.insert(tile.clone());
    });

    while unassigned_tiles.len() > 0 {
        let candidate_placements = get_candidate_placements(&assigned_tiles);
        for candidate_placement in candidate_placements {
            for tile in unassigned_tiles.iter() {
                if let Some(ok_tile) =
                    is_any_placement_possible(&assigned_tiles, candidate_placement, tile)
                {
                    assigned_tiles.insert(candidate_placement, ok_tile);
                    break;
                }
            }

            unassigned_tiles.retain(|elem| assigned_tiles.values().all(|e| e != elem));
        }
    }
    assigned_tiles
}

fn calculate_p1(image: &AHashMap<(i64, i64), Tile>) -> i64 {
    let max_x: i64 = image.keys().map(|&(x, _)| x).max().expect("empty keyset?");
    let min_x: i64 = image.keys().map(|&(x, _)| x).min().expect("empty keyset?");
    let max_y: i64 = image.keys().map(|&(_, y)| y).max().expect("empty keyset?");
    let min_y: i64 = image.keys().map(|&(_, y)| y).min().expect("empty keyset?");

    image.get(&(max_x, max_y)).expect("invalid image").id
        * image.get(&(min_x, max_y)).expect("invalid image").id
        * image.get(&(max_x, min_y)).expect("invalid image").id
        * image.get(&(min_x, min_y)).expect("invalid image").id
}

fn calculate_p2(image: &AHashMap<(i64, i64), Tile>) -> i64 {
    0
}

fn main() {
    let args = Cli::parse();
    let raw_inp = fs::read_to_string(args.input).expect("can't open input file");
    let data = parse(&raw_inp);
    let image = build_image(&data);
    let p1 = calculate_p1(&image);
    let p2 = calculate_p2(&image);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2020_20");

    const REAL_DATA: &str = include_str!("../../inputs/real/2020_20");

    #[test]
    fn test_p1_example() {
        assert_eq!(
            calculate_p1(&build_image(&parse(EXAMPLE_DATA))),
            20899048083289
        );
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(
            calculate_p1(&build_image(&parse(REAL_DATA))),
            17148689442341
        );
    }
}
