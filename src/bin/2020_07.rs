use clap::Parser;
use sscanf::sscanf;
use std::fs;
use fnv::{FnvHashSet, FnvHashMap};

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn parse(raw_inp: &str) -> Vec<(&str, &str)> {
    raw_inp
        .trim()
        .split("\n")
        .map(|item| sscanf!(item, "{str} bags contain {str}."))
        .map(|item| item.expect("invalid input"))
        .collect()
}

fn calculate(data: &Vec<(&str, &str)>) -> (usize, usize) {
    let mut p1_hs: FnvHashSet<&str> = FnvHashSet::default();
    p1_hs.insert("shiny gold");
    
    let mut any_changed = true;
    
    while any_changed {
        any_changed = false;
        for (bag, subbags) in data {
            if !p1_hs.contains(bag) {
                if p1_hs.iter().any(|item| subbags.contains(item)) {
                    p1_hs.insert(bag);
                    any_changed = true;
                }
            }
        }
    }
    
    let mut p2_map: FnvHashMap<&str, usize> = FnvHashMap::default();
    
    for (bag, subbags) in data {
        if subbags == &"no other bags" {
            p2_map.insert(bag, 1);
        }
    }
    
    any_changed = true;
    while any_changed {
        any_changed = false;
        for (bag, subbags) in data {
            let sb: Vec<&str> = subbags.split(", ").map(|i| i[0..i.len()-4].trim()).collect();

            if !p2_map.contains_key(bag) && sb.iter().all(|x| p2_map.contains_key(&x[2..])) {
                let mut c = 1;
                for i in sb.iter() {
                    let n: usize = i[0..1].parse().expect("parse failed");
                    c += n * p2_map.get(&i[2..]).expect("just checked this");
                }
                p2_map.insert(bag, c);
                any_changed = true;
            }
        }
    }
    
    (p1_hs.len() - 1, *p2_map.get("shiny gold").expect("p2 calc failed") - 1)
}

fn main() {
    let args = Cli::parse();
    let raw_inp = fs::read_to_string(args.input).expect("can't open input file");
    let data = parse(&raw_inp);
    let (p1, p2) = calculate(&data);
    println!("{}\n{}", p1, p2);
}
