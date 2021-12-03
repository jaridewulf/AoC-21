use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn star_one(content: Vec<String>) -> Vec<u32> {
    let mut col_pos = 0;
    let mut gamma_rate: Vec<u32>;
    while col_pos.clone() < 12 {
        let mut column_numbers: Vec<String>;

        for el_pos in 0..content.len() {
            let to_check: Vec<_> = content[el_pos].split_whitespace().collect();
            column_numbers.push(to_check[col_pos].to_owned());
        }

        if column_numbers.iter().filter(|&n| *n == "1").count() > column_numbers.len() / 2 {
            gamma_rate.push(1);
        } else {
            gamma_rate.push(0);
        }

        col_pos += 1;
    }
    return gamma_rate;
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
