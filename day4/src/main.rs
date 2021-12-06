use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let numbers = get_numbers(&args[1]);
    let boards = get_boards(&args[2]);

    println!("{}", numbers[0]);
    println!("{}", boards);
}

fn get_numbers(file_name: &str) -> Vec<i32> {
    println!("Reading {}", file_name);

    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let numbers_foo = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    return numbers_foo[0]
        .split(',')
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
}

fn get_boards(file_name: &str) -> usize {
    println!("Reading {}", file_name);

    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let board_foo = reader
        .lines()
        .map(|line| line.unwrap())
        .filter(|l| l.len() > 0)
        .collect::<Vec<String>>();

    let tmp = BingoBoard {
        value: HashMap::from([
            ("Mercury", 0.4),
            ("Venus", 0.7),
            ("Earth", 1.0),
            ("Mars", 1.5),
        ]),
    };

    return board_foo.len();
}

struct BingoBoard {
    pub board: [i32; 25],
    pub board_coords: [(usize, usize); 25],
    pub board_cols: [[usize; 5]; 5],
    pub board_rows: [[usize; 5]; 5],
}
