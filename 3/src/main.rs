use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("src/input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let mut map: Vec<Vec<&str>> = vec![]; 

    for line in contents.lines() {
        map.push(line.split("").filter(|&x| x != "").collect())
    }

    let a = slope(1, 1, &map);
    let b = slope(1, 3, &map);
    let c = slope(1, 5, &map);
    let d = slope(1, 7, &map);
    let e = slope(2, 1, &map);
    
    println!("Solution 1: {}", b);
    println!("Solution 2: {}", a*b*c*d*e);

    Ok(())

}

fn slope(pos_y_rate: usize, pos_x_rate: usize, map: &Vec<Vec<&str>>) -> usize {
    let mut i = 0;
    let mut pos_y = pos_y_rate;
    let mut pos_x = pos_x_rate;
    loop {
        let chars = &map[pos_y];
        let width = chars.len();

        if width <= pos_x {
            pos_x = pos_x % (width);
        }
        
        if chars[pos_x] == "#" {
            i += 1;
        }

        pos_y += pos_y_rate;
        pos_x += pos_x_rate;

        if pos_y >= map.len() { break; }
    }
    i
}
