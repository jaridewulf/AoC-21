use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn star_one(content: Vec<String>) -> i32 {
    let mut col_pos = 0;
    let mut gamma_rate: Vec<u32> = Vec::new();
    let mut epsilon_rate: Vec<u32> = Vec::new();

    while col_pos.clone() < 13 {
        let mut column_numbers: Vec<String> = Vec::new();

        for el_pos in 0..content.len() {
            let to_check: Vec<_> = content[el_pos].split("").collect();
            column_numbers.push(to_check[col_pos].to_owned());
        }

        if column_numbers.iter().filter(|&n| *n == "1").count() > column_numbers.len() / 2 {
            gamma_rate.push(1);
        }
        if column_numbers.iter().filter(|&n| *n == "0").count() > column_numbers.len() / 2 {
            gamma_rate.push(0);
        }
        col_pos += 1;
    }

    for (i, _val) in gamma_rate.iter().enumerate() {
        if gamma_rate[i] == 0 {
            epsilon_rate.push(1);
        }
        if gamma_rate[i] == 1 {
            epsilon_rate.push(0);
        }
    }

    let gamma_rate_string: String = gamma_rate
        .into_iter()
        .map(|d| std::char::from_digit(d, 10).unwrap())
        .collect();

    let epsilon_rate_string: String = epsilon_rate
        .into_iter()
        .map(|d| std::char::from_digit(d, 10).unwrap())
        .collect();

    return i32::from_str_radix(&gamma_rate_string, 2).unwrap()
        * i32::from_str_radix(&epsilon_rate_string, 2).unwrap();
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
