use ahash::{AHashMap, AHashSet};
use clap::Parser;
use std::collections::hash_map::Entry;
use std::fs;
use std::str::FromStr;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

const CONTAINS: &str = " (contains ";

impl FromStr for Food {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let allergens;
        let raw_ingredients;

        if s.contains(CONTAINS) {
            let raw_allergens;
            (raw_ingredients, raw_allergens) = s.split_once(CONTAINS).ok_or("split should work")?;
            allergens = raw_allergens
                .trim_end_matches(')')
                .split(", ")
                .map(|s| s.to_string())
                .collect();
        } else {
            raw_ingredients = s;
            allergens = vec![];
        }

        let ingredients = raw_ingredients.split(' ').map(|s| s.to_string()).collect();

        Ok(Food {
            ingredients,
            allergens,
        })
    }
}

fn parse(raw_inp: &str) -> Vec<Food> {
    raw_inp
        .trim()
        .lines()
        .map(|line| line.parse().expect("parse failed"))
        .collect()
}

fn count_safe_ingredients(safe_ingredients: &AHashSet<&String>, data: &[Food]) -> usize {
    data.iter()
        .map(|food| {
            food.ingredients
                .iter()
                .filter(|i| safe_ingredients.contains(i))
                .count()
        })
        .sum()
}

fn mapping_to_str(mapping: &AHashMap<&String, &String>) -> String {
    let mut vec: Vec<(&String, &String)> = vec![];

    mapping.iter().for_each(|(&k, &v)| vec.push((k, v)));

    vec.sort_unstable_by_key(|item| item.0);

    vec.iter()
        .map(|&item| item.1)
        .map(|s| s.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

fn is_consistent(ingredient: &String, allergen: &String, data: &[Food]) -> bool {
    for food in data.iter() {
        if food.allergens.contains(allergen) && !food.ingredients.contains(ingredient) {
            return false;
        }
    }
    true
}

fn calculate_safe_ingredients<'a>(
    all_ingredients: &AHashSet<&'a String>,
    all_allergens: &AHashSet<&'a String>,
    data: &[Food],
    allergen_candidates: &mut AHashMap<&'a String, AHashSet<&'a String>>,
) -> AHashSet<&'a String> {
    let mut safe_ingredients: AHashSet<&String> = AHashSet::default();

    for ingredient in all_ingredients.iter() {
        let mut can_be_any_allergen = false;
        for allergen in all_allergens.iter() {
            if is_consistent(ingredient, allergen, data) {
                can_be_any_allergen = true;
            } else {
                allergen_candidates
                    .get_mut(ingredient)
                    .expect("ingredient should exist")
                    .remove(allergen);
            }
        }
        if !can_be_any_allergen {
            safe_ingredients.insert(ingredient);
        }
    }

    safe_ingredients
}

fn get_all_allergens(data: &[Food]) -> AHashSet<&String> {
    data.iter().flat_map(|food| &food.allergens).collect()
}

fn get_all_ingredients(data: &[Food]) -> AHashSet<&String> {
    data.iter().flat_map(|food| &food.ingredients).collect()
}

fn get_allergen_candidates(data: &[Food]) -> AHashMap<&String, AHashSet<&String>> {
    let mut allergen_candidates: AHashMap<&String, AHashSet<&String>> = AHashMap::default();

    for food in data.iter() {
        for ingredient in food.ingredients.iter() {
            match allergen_candidates.entry(ingredient) {
                Entry::Occupied(mut entry) => {
                    food.allergens.iter().for_each(|c| {
                        entry.get_mut().insert(c);
                    });
                }
                Entry::Vacant(entry) => {
                    entry.insert(food.allergens.iter().collect());
                }
            }
        }
    }

    allergen_candidates
}

fn calculate_allergen_mapping<'a>(
    safe_ingredients: &AHashSet<&'a String>,
    all_ingredients: &AHashSet<&'a String>,
    allergen_candidates: &mut AHashMap<&'a String, AHashSet<&'a String>>,
) -> AHashMap<&'a String, &'a String> {
    let mut allergen_mapping: AHashMap<&String, &String> = AHashMap::default();

    while allergen_mapping.len() + safe_ingredients.len() != all_ingredients.len() {
        for ingredient in all_ingredients.iter() {
            let candidates = allergen_candidates
                .get(ingredient)
                .expect("ingredient should exist");

            if candidates.len() == 1 {
                allergen_mapping.insert(candidates.iter().next().unwrap(), ingredient);
            }
        }

        for ingredient in all_ingredients.iter() {
            allergen_candidates
                .get_mut(ingredient)
                .expect("ingredient should exist")
                .retain(|i| !allergen_mapping.contains_key(i));
        }
    }

    allergen_mapping
}

fn calculate(data: &[Food]) -> (usize, String) {
    let all_ingredients = get_all_ingredients(data);
    let all_allergens = get_all_allergens(data);
    let mut allergen_candidates = get_allergen_candidates(data);

    let safe_ingredients = calculate_safe_ingredients(
        &all_ingredients,
        &all_allergens,
        data,
        &mut allergen_candidates,
    );

    safe_ingredients.iter().for_each(|safe| {
        allergen_candidates.insert(safe, AHashSet::default());
    });

    let mapping = calculate_allergen_mapping(
        &safe_ingredients,
        &all_ingredients,
        &mut allergen_candidates,
    );

    let p1 = count_safe_ingredients(&safe_ingredients, data);
    let p2 = mapping_to_str(&mapping);
    (p1, p2)
}

fn main() {
    let args = Cli::parse();
    let raw_inp = fs::read_to_string(args.input).expect("can't open input file");
    let data = parse(&raw_inp);
    let (p1, p2) = calculate(&data);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2020_21");

    const REAL_DATA: &str = include_str!("../../inputs/real/2020_21");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate(&parse(EXAMPLE_DATA)).0, 5);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate(&parse(REAL_DATA)).0, 2162);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate(&parse(EXAMPLE_DATA)).1, "mxmxvkd,sqjhc,fvjkl");
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(
            calculate(&parse(REAL_DATA)).1,
            "lmzg,cxk,bsqh,bdvmx,cpbzbx,drbm,cfnt,kqprv"
        );
    }
}
