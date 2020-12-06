use std::fs::File;
use std::io::prelude::*;
use itertools::Itertools;
use regex::Regex;

struct Validator {
    pub is_valid: fn(&str) -> bool,
}

impl Validator {
    fn new(is_valid: fn(&str) -> bool) -> Self {
        Validator { is_valid }
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("src/input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut i = 0;
    for line in contents.split("\n\n") {
        let valid_fields = [
            ("byr", Validator::new(|c: &str| c.parse::<usize>().unwrap() >= 1920 && c.parse::<usize>().unwrap() <= 2002).is_valid),
            ("iyr", Validator::new(|c: &str| c.parse::<usize>().unwrap() >= 2010 && c.parse::<usize>().unwrap() <= 2020).is_valid),
            ("eyr", Validator::new(|c: &str| c.parse::<usize>().unwrap() >= 2020 && c.parse::<usize>().unwrap() <= 2030).is_valid),
            ("hgt", Validator::new(|c: &str| {
                let mut valid = false;
                if c.contains("in") {
                    let end = c.find("in").unwrap();
                    let height = c[0..end].parse::<usize>().unwrap();
                    valid = height >= 59 && height <= 76;
                } else if c.contains("cm") {
                    let end = c.find("cm").unwrap();
                    let height = c[0..end].parse::<usize>().unwrap();
                    valid = height >= 150 && height <= 193;
                }
                valid
            }).is_valid),
            ("hcl", Validator::new(|c: &str| Regex::new("^#[0-9A-Fa-f]{6}$").unwrap().is_match(c)).is_valid),
            ("ecl", Validator::new(|c: &str| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().any(|i| c.to_owned() == i.to_owned())).is_valid),
            ("pid", Validator::new(|c: &str| c.len() == 9).is_valid),
            //("cid", Validator::new(|_c: &str| true).is_valid)
        ];

        let fields : Vec<(&str, &str)> = line
            .split(|c: char| (c == ' ') || (c == '\n'))
            .map(|c: &str | c.splitn(2, ":").collect_tuple().unwrap()).collect();

        if valid_fields.iter().all(|t| fields.iter().any(|i| i.0 == t.0 && t.1(i.1))) {
            i += 1;
        }
    }

    println!("{:?}", i);

    Ok(())
}