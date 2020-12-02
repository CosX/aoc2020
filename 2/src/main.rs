use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn main() -> std::io::Result<()> {
    let mut file = File::open("src/input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut i = 0;

    for line in contents.split("\n") {
        let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();

        for cap in re.captures_iter(line) {
            let filtered_pw : String = cap[4]
                .chars()
                .filter(|&x| x.to_string() == &cap[3])
                .collect();

            //println!("{}", filtered_pw);

            if filtered_pw.len() >= cap[1].parse::<usize>().unwrap() 
            && filtered_pw.len() <= cap[2].parse::<usize>().unwrap(){

                i += 1;
            }
        }
    }

    println!("{}", i);

    i = 0;

    for line in contents.split("\n") {
        let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();

        for cap in re.captures_iter(line) {
            let pw : Vec<char> = cap[4]
                .chars()
                .collect();
            
            let first = (cap[1].parse::<usize>().unwrap()) -1;
            let second = (cap[2].parse::<usize>().unwrap()) -1;

            if second >= pw.len() {
                break;
            }
            if (pw[first].to_string() == cap[3].to_string()) ^ (pw[second].to_string() == cap[3].to_string()) {
                println!("{} {} {} {}", &cap[4], &cap[3], first, second);
                i += 1;
            }
        }
    }

    println!("{}", i);
    Ok(())
}
