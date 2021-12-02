use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn star_one(content: Vec<u16>) -> u16 {
    let mut tracker = 0;

    for i in 0..content.len() - 1 {
        if content[i] < content[i + 1] {
            tracker += 1;
        }
    }
    tracker
}

fn star_two(content: Vec<u16>) -> u16 {
    let mut tracker = 0;

    for i in 0..content.len() - 3 {
        if content[i] + content[i + 1] + content[i + 2]
            < content[i + 1] + content[i + 2] + content[i + 3]
        {
            tracker += 1;
        }
    }
    tracker
}

fn main() {
    println!("{:?}", env::current_dir().unwrap());
    let input = File::open("input").expect("file doesn't exist");
    let buf_reader = BufReader::new(input);

    let content: Vec<u16> = buf_reader
        .lines()
        .map(|line| line.unwrap().parse::<u16>().unwrap())
        .collect();

    println!(
        "the answer of the first problem is: {:?}",
        star_one(content.clone())
    );
    println!(
        "the answer of the second problem is: {:?}",
        star_two(content)
    );
}
