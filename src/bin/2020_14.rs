use ahash::AHashMap;
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

struct Instruction {
    mask: Option<[u8; 36]>,
    mem: Option<(u64, u64)>,
}

fn parse(raw_inp: &str) -> Vec<Instruction> {
    raw_inp
        .trim()
        .split('\n')
        .filter_map(|line| line.trim().split_once(" = "))
        .map(|(cmd, data)| {
            if cmd == "mask" {
                Instruction {
                    mask: Some(
                        data.bytes()
                            .collect::<Vec<u8>>()
                            .try_into()
                            .expect("into array failed"),
                    ),
                    mem: None,
                }
            } else {
                let loc: u64 = cmd
                    .trim_start_matches("mem[")
                    .trim_end_matches(']')
                    .parse()
                    .expect("invalid memory location");

                let mem_data: u64 = data.parse().expect("invalid mem data");

                Instruction {
                    mask: None,
                    mem: Some((loc, mem_data)),
                }
            }
        })
        .collect()
}

fn calculate_p1(data: &[Instruction]) -> u64 {
    let mut mem: AHashMap<u64, u64> = AHashMap::with_capacity(data.len());
    let mut current_mask: [u8; 36] = [0; 36];

    for inst in data {
        if let Some(mask) = inst.mask {
            current_mask = mask;
        } else if let Some((mem_location, mem_data)) = inst.mem {
            let mut masked_data = mem_data;

            current_mask
                .iter()
                .enumerate()
                .for_each(|(idx, m)| match m {
                    b'1' => {
                        masked_data |= 1 << (36 - idx - 1);
                    }
                    b'0' => {
                        masked_data &= !(1 << (36 - idx - 1));
                    }
                    _ => {}
                });

            mem.insert(mem_location, masked_data);
        }
    }

    mem.values().sum()
}

fn calculate_p2(data: &[Instruction]) -> u64 {
    let mut mem: AHashMap<u64, u64> = AHashMap::with_capacity(65536);
    let mut current_mask = [0u8; 36];

    for inst in data {
        if let Some(mask) = inst.mask {
            current_mask = mask;
        } else if let Some((mem_location, mem_data)) = inst.mem {
            let mut masked_addr = mem_location;

            current_mask
                .iter()
                .enumerate()
                .filter(|&(_, &m)| m == b'1')
                .for_each(|(idx, _)| {
                    masked_addr |= 1 << (36 - idx - 1);
                });

            let mut addresses: Vec<u64> = vec![masked_addr];

            current_mask
                .iter()
                .enumerate()
                .filter(|&(_, &m)| m == b'X')
                .for_each(|(idx, _)| {
                    addresses = addresses
                        .iter()
                        .flat_map(|a| [a | 1 << (36 - idx - 1), a & (!(1 << (36 - idx - 1)))])
                        .collect();
                });

            addresses.iter().for_each(|&a| {
                mem.insert(a, mem_data);
            });
        }
    }
    mem.values().sum()
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

    const TEST_DATA_P1: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    const TEST_DATA_P2: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    const REAL_DATA: &str = include_str!("../../inputs/real/2020_14");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(TEST_DATA_P1)), 165);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&parse(TEST_DATA_P2)), 208);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(REAL_DATA)), 13556564111697);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(REAL_DATA)), 4173715962894);
    }
}
