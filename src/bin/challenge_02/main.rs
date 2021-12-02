use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn star_one(content: Vec<String>) -> i32 {
    let mut depth = 0;
    let mut h_pos = 0;

    for i in 0..content.len() {
        let to_check: Vec<_> = content[i].split_whitespace().collect();
        match to_check[0] {
            "forward" => h_pos = h_pos + to_check[1].parse::<i32>().unwrap(),
            "down" => depth = depth + to_check[1].parse::<i32>().unwrap(),
            "up" => depth = depth - to_check[1].parse::<i32>().unwrap(),
            _ => panic!("Oops, I failed"),
        }
    }
    depth * h_pos
}

fn star_two(content: Vec<String>) -> i32 {
    let mut depth = 0;
    let mut h_pos = 0;
    let mut aim = 0;

    for i in 0..content.len() {
        let to_check: Vec<_> = content[i].split_whitespace().collect();
        match to_check[0] {
            "forward" => {
                h_pos = h_pos + to_check[1].parse::<i32>().unwrap();
                depth = depth + to_check[1].parse::<i32>().unwrap() * aim;
            }
            "down" => {
                aim = aim + to_check[1].parse::<i32>().unwrap();
            }
            "up" => {
                aim = aim - to_check[1].parse::<i32>().unwrap();
            }
            _ => panic!("Oops, I failed"),
        }
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

    println!(
        "the answer of the first problem is: {:?}",
        star_two(content)
    );
}
