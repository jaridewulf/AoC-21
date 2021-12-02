use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn star_one(content: Vec<String>) -> i32 {
    let mut depth = 0;
    let mut h_pos = 0;

    for i in 0..content.len() {
        let mut toCheck: Vec<_> = content[i].split_whitespace().collect();
        match toCheck[0] {
            "forward" => h_pos = h_pos + toCheck[1].parse::<i32>().unwrap(),
            "down" => depth = depth + toCheck[1].parse::<i32>().unwrap(),
            "up" => depth = depth - toCheck[1].parse::<i32>().unwrap(),
            _ => panic!("Oops, I failed"),
        }
        println!("{:?} {:?}", h_pos, depth);
    }
    depth * h_pos
}

fn main() {
    println!("{:?}", env::current_dir().unwrap());
    let input = File::open("input").expect("file doesn't exist");
    let buf_reader = BufReader::new(input);

    let content: Vec<String> = buf_reader
        .lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect();

    println!(
        "the answer of the first problem is: {:?}",
        star_one(content.clone())
    );
}
