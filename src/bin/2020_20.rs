use ahash::{AHashMap, AHashSet};
use clap::Parser;
use ndarray::{s, ArcArray, Array2, ArrayView2, Ix2, Zip};
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
    id: u64,
    arr: ArcArray<bool, Ix2>,
}

impl Tile {
    fn clone_rotate(&self) -> Self {
        let mut new_arr = self.arr.slice(s![..,..;-1]);
        new_arr.swap_axes(0, 1);
        Tile {
            id: self.id,
            arr: new_arr.to_owned().into_shared(),
        }
    }

    fn clone_flip_vertical(&self) -> Self {
        let new_arr = self.arr.slice(s![..;-1,..]);
        Tile {
            id: self.id,
            arr: new_arr.to_owned().into_shared(),
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

        Ok(Tile {
            id,
            arr: arr.into_shared(),
        })
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
            candidates.insert((new_x, new_y));
        }
    }
    candidates.retain(|elem| !existing_placements.contains_key(elem));
    candidates
}

fn is_possible_placement(
    existing_placements: &AHashMap<(i64, i64), Tile>,
    proposed_location: (i64, i64),
    tile: &Tile,
) -> bool {
    let (prop_x, prop_y) = proposed_location;

    if let Some(tile_above) = existing_placements.get(&(prop_x, prop_y - 1)) {
        if tile_above.arr.slice(s![-1, ..]) != tile.arr.slice(s![0, ..]) {
            return false;
        }
    }

    if let Some(tile_below) = existing_placements.get(&(prop_x, prop_y + 1)) {
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
    layout: &AHashMap<(i64, i64), Tile>,
    proposed_location: (i64, i64),
    tile: &Tile,
) -> Option<Tile> {
    if is_possible_placement(layout, proposed_location, tile) {
        return Some(tile.clone());
    }

    let rotated_tile = tile.clone_rotate();
    if is_possible_placement(layout, proposed_location, &rotated_tile) {
        return Some(rotated_tile);
    }

    let double_rotated_tile = rotated_tile.clone_rotate();
    if is_possible_placement(layout, proposed_location, &double_rotated_tile) {
        return Some(double_rotated_tile);
    }

    let triple_rotated_tile = double_rotated_tile.clone_rotate();
    if is_possible_placement(layout, proposed_location, &triple_rotated_tile) {
        return Some(triple_rotated_tile);
    }

    let flipped_tile = tile.clone_flip_vertical();
    if is_possible_placement(layout, proposed_location, &flipped_tile) {
        return Some(flipped_tile);
    }

    let flipped_rotated_tile = flipped_tile.clone_rotate();
    if is_possible_placement(layout, proposed_location, &flipped_rotated_tile) {
        return Some(flipped_rotated_tile);
    }

    let flipped_double_rotated_tile = flipped_rotated_tile.clone_rotate();
    if is_possible_placement(layout, proposed_location, &flipped_double_rotated_tile) {
        return Some(flipped_double_rotated_tile);
    }

    let flipped_triple_rotated_tile = flipped_double_rotated_tile.clone_rotate();
    if is_possible_placement(layout, proposed_location, &flipped_triple_rotated_tile) {
        return Some(flipped_triple_rotated_tile);
    }

    None
}

fn build_layout(data: &[Tile]) -> AHashMap<(i64, i64), Tile> {
    let mut layout: AHashMap<(i64, i64), Tile> = AHashMap::default();
    let mut unassigned: AHashSet<Tile> = AHashSet::default();

    layout.insert((0, 0), data[0].clone());

    data.iter().skip(1).for_each(|tile| {
        unassigned.insert(tile.clone());
    });

    while unassigned.len() > 0 {
        let candidate_placements = get_candidate_placements(&layout);
        for candidate_placement in candidate_placements {
            let mut placed = false;
            for tile in unassigned.iter() {
                if let Some(ok_tile) = is_any_placement_possible(&layout, candidate_placement, tile)
                {
                    layout.insert(candidate_placement, ok_tile);
                    placed = true;
                    break;
                }
            }

            if placed {
                unassigned.retain(|elem| layout.values().all(|e| e != elem));
                break;
            }
        }
    }
    layout
}

struct LayoutInfo {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

fn calculate_bounds(layout: &AHashMap<(i64, i64), Tile>) -> LayoutInfo {
    let max_x: i64 = layout.keys().map(|&(x, _)| x).max().expect("empty keyset?");
    let min_x: i64 = layout.keys().map(|&(x, _)| x).min().expect("empty keyset?");
    let max_y: i64 = layout.keys().map(|&(_, y)| y).max().expect("empty keyset?");
    let min_y: i64 = layout.keys().map(|&(_, y)| y).min().expect("empty keyset?");

    LayoutInfo {
        min_x,
        max_x,
        min_y,
        max_y,
    }
}

fn layout_to_image(layout: &AHashMap<(i64, i64), Tile>, bounds: &LayoutInfo) -> Array2<bool> {
    let x_size: usize = (bounds.max_x - bounds.min_x) as usize + 1;
    let y_size: usize = (bounds.max_y - bounds.min_y) as usize + 1;

    Array2::from_shape_fn([y_size * 8, x_size * 8], |(y, x)| {
        let tile_x: i64 = bounds.min_x + (x / 8) as i64;
        let tile_y: i64 = bounds.min_y + (y / 8) as i64;

        *layout
            .get(&(tile_x, tile_y))
            .expect("invalid tile")
            .arr
            .get((1 + y % 8, 1 + x % 8))
            .expect("invalid tile index")
    })
}

fn calculate_p1(layout: &AHashMap<(i64, i64), Tile>, bounds: &LayoutInfo) -> u64 {
    layout
        .get(&(bounds.max_x, bounds.max_y))
        .expect("invalid layout")
        .id
        * layout
            .get(&(bounds.min_x, bounds.max_y))
            .expect("invalid layout")
            .id
        * layout
            .get(&(bounds.max_x, bounds.min_y))
            .expect("invalid layout")
            .id
        * layout
            .get(&(bounds.min_x, bounds.min_y))
            .expect("invalid layout")
            .id
}

const SEA_MONSTER: &str = "                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";

fn get_sea_monster_array() -> Array2<bool> {
    let rows = SEA_MONSTER.split('\n').count();
    let columns = SEA_MONSTER.split('\n').map(|x| x.len()).max().unwrap();
    let v: Vec<bool> = SEA_MONSTER
        .split('\n')
        .flat_map(|x| x.bytes())
        .map(|b| b == b'#')
        .collect();

    Array2::from_shape_vec([rows, columns], v).expect("sea monster construction failed")
}

fn count_monsters(image: &Array2<bool>, sea_monster: &ArrayView2<bool>) -> usize {
    let sea_monster_ysize = sea_monster.dim().0;
    let sea_monster_xsize = sea_monster.dim().1;

    let mut num_monsters = 0;
    for y in 0..image.dim().0 - sea_monster_ysize {
        for x in 0..image.dim().1 - sea_monster_xsize {
            let slice = image.slice(s![y..y + sea_monster_ysize, x..x + sea_monster_xsize]);

            let is_monster = Zip::from(sea_monster).and(slice).all(|&m, &s| s || !m);

            if is_monster {
                num_monsters += 1;
            }
        }
    }
    num_monsters
}

fn count_true(data: &ArrayView2<bool>) -> usize {
    data.iter().filter(|&itm| *itm).count()
}

fn calculate_p2(layout: &AHashMap<(i64, i64), Tile>, bounds: &LayoutInfo) -> usize {
    let image = layout_to_image(layout, bounds);
    let mut sea_monster: Array2<bool> = get_sea_monster_array();
    let mut num_monsters = 0;

    while num_monsters == 0 {
        num_monsters = count_monsters(&image, &sea_monster.view());
        if num_monsters == 0 {
            // If no monsters found, flip monster and try again
            num_monsters = count_monsters(&image, &sea_monster.slice(s![..,..;-1]));
        }

        if num_monsters == 0 {
            // If still no monsters found, rotate monster 90deg and try again
            sea_monster = sea_monster.slice(s![..,..;-1]).into_owned();
            sea_monster.swap_axes(0, 1);
        }
    }

    count_true(&image.view()) - num_monsters * count_true(&sea_monster.view())
}

fn main() {
    let args = Cli::parse();
    let raw_inp = fs::read_to_string(args.input).expect("can't open input file");
    let data = parse(&raw_inp);
    let layout = build_layout(&data);
    let bounds = calculate_bounds(&layout);
    let p1 = calculate_p1(&layout, &bounds);
    let p2 = calculate_p2(&layout, &bounds);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2020_20");

    const REAL_DATA: &str = include_str!("../../inputs/real/2020_20");

    #[test]
    fn test_p1_example() {
        let layout = build_layout(&parse(EXAMPLE_DATA));
        let bounds = calculate_bounds(&layout);
        assert_eq!(calculate_p1(&layout, &bounds), 20899048083289);
    }

    #[test]
    fn test_p1_real() {
        let layout = build_layout(&parse(REAL_DATA));
        let bounds = calculate_bounds(&layout);
        assert_eq!(calculate_p1(&layout, &bounds), 17148689442341);
    }

    #[test]
    fn test_p2_example() {
        let layout = build_layout(&parse(EXAMPLE_DATA));
        let bounds = calculate_bounds(&layout);
        assert_eq!(calculate_p2(&layout, &bounds), 273);
    }

    #[test]
    fn test_p2_real() {
        let layout = build_layout(&parse(REAL_DATA));
        let bounds = calculate_bounds(&layout);
        assert_eq!(calculate_p2(&layout, &bounds), 2009);
    }
}
