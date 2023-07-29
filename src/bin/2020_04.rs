use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

fn parse_passport_parts(item: &str) -> Passport {
    let mut passport = Passport {
        byr: None,
        iyr: None,
        eyr: None,
        hgt: None,
        hcl: None,
        ecl: None,
        pid: None,
    };

    for part in item.split_whitespace() {
        if let Some(s) = part.strip_prefix("byr:") {
            passport.byr = Some(s.to_string());
        } else if let Some(s) = part.strip_prefix("iyr:") {
            passport.iyr = Some(s.to_string());
        } else if let Some(s) = part.strip_prefix("eyr:") {
            passport.eyr = Some(s.to_string());
        } else if let Some(s) = part.strip_prefix("hgt:") {
            passport.hgt = Some(s.to_string());
        } else if let Some(s) = part.strip_prefix("hcl:") {
            passport.hcl = Some(s.to_string());
        } else if let Some(s) = part.strip_prefix("ecl:") {
            passport.ecl = Some(s.to_string());
        } else if let Some(s) = part.strip_prefix("pid:") {
            passport.pid = Some(s.to_string());
        }
    }

    passport
}

fn parse(raw_inp: &str) -> Vec<Passport> {
    raw_inp
        .trim()
        .split("\n\n")
        .map(parse_passport_parts)
        .collect()
}

fn valid_p1(item: &Passport) -> bool {
    item.byr.is_some()
        && item.iyr.is_some()
        && item.eyr.is_some()
        && item.hgt.is_some()
        && item.hcl.is_some()
        && item.ecl.is_some()
        && item.pid.is_some()
}

fn is_valid_int_within_bounds(item: Option<&str>, lower_bound: i64, upper_bound: i64) -> bool {
    item.and_then(|s| s.parse::<i64>().ok())
        .map(|y| y >= lower_bound && y <= upper_bound)
        .unwrap_or(false)
}

fn byr_valid_p2(item: &Passport) -> bool {
    is_valid_int_within_bounds(item.byr.as_deref(), 1920, 2002)
}

fn iyr_valid_p2(item: &Passport) -> bool {
    is_valid_int_within_bounds(item.iyr.as_deref(), 2010, 2020)
}

fn eyr_valid_p2(item: &Passport) -> bool {
    is_valid_int_within_bounds(item.eyr.as_deref(), 2020, 2030)
}

fn hgt_valid_p2(item: &Passport) -> bool {
    match &item.hgt {
        None => false,
        Some(s) => {
            if let Some(s) = s.strip_suffix("cm") {
                is_valid_int_within_bounds(Some(s), 150, 193)
            } else if let Some(s) = s.strip_suffix("in") {
                is_valid_int_within_bounds(Some(s), 59, 76)
            } else {
                false
            }
        }
    }
}

fn hcl_valid_p2(item: &Passport) -> bool {
    match &item.hcl {
        None => false,
        Some(s) => {
            s.len() == 7 && s.starts_with('#') && s[1..7].chars().all(|c| c.is_ascii_hexdigit())
        }
    }
}

fn ecl_valid_p2(item: &Passport) -> bool {
    match &item.ecl {
        None => false,
        Some(s) => {
            s == "amb"
                || s == "blu"
                || s == "brn"
                || s == "gry"
                || s == "grn"
                || s == "hzl"
                || s == "oth"
        }
    }
}

fn pid_valid_p2(item: &Passport) -> bool {
    item.pid
        .as_ref()
        .map(|s| s.len() == 9 && s.chars().all(char::is_numeric))
        .unwrap_or(false)
}

fn valid_p2(item: &Passport) -> bool {
    byr_valid_p2(item)
        && eyr_valid_p2(item)
        && iyr_valid_p2(item)
        && hgt_valid_p2(item)
        && hcl_valid_p2(item)
        && ecl_valid_p2(item)
        && pid_valid_p2(item)
}

fn main() {
    let args = Cli::parse();

    let raw_inp = fs::read_to_string(args.input).expect("can't open input file");

    let data: Vec<Passport> = parse(&raw_inp);

    let p1 = data.iter().filter(|item| valid_p1(item)).count();
    let p2 = data.iter().filter(|item| valid_p2(item)).count();

    println!("{}\n{}", p1, p2);
}

#[test]
fn test_p1() {
    assert_eq!(
        valid_p1(&parse_passport_parts(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm"
        )),
        true
    );
    assert_eq!(
        valid_p1(&parse_passport_parts(
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929"
        )),
        false
    );
    assert_eq!(
        valid_p1(&parse_passport_parts(
            "hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm"
        )),
        true
    );
    assert_eq!(
        valid_p1(&parse_passport_parts(
            "hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in"
        )),
        false
    );
}

#[test]
fn test_p2_invalid() {
    assert_eq!(
        valid_p2(&parse_passport_parts(
            "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"
        )),
        false
    );
    assert_eq!(
        valid_p2(&parse_passport_parts(
            "iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946"
        )),
        false
    );
    assert_eq!(
        valid_p2(&parse_passport_parts(
            "hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"
        )),
        false
    );
    assert_eq!(
        valid_p2(&parse_passport_parts(
            "hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007"
        )),
        false
    );
}

#[test]
fn test_p2_valid() {
    assert_eq!(
        valid_p2(&parse_passport_parts(
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f"
        )),
        true
    );
    assert_eq!(
        valid_p2(&parse_passport_parts(
            "eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"
        )),
        true
    );
    assert_eq!(
        valid_p2(&parse_passport_parts(
            "hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022"
        )),
        true
    );
    assert_eq!(
        valid_p2(&parse_passport_parts(
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
        )),
        true
    );
}
