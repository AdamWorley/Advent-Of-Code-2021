use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("Reading {}", filename);

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let instructions: Vec<Instruction> = reader
        .lines()
        .map(|line| parse_instruction(&line.unwrap()))
        .collect();

    let horizontal_distance = instructions
        .iter()
        .filter(|i| matches!(i.direction, Direction::Forward))
        .fold(0, |acc, x| acc + x.distance);

    let vertical_distance = instructions
        .iter()
        .filter(|i| matches!(i.direction, Direction::Up | Direction::Down))
        .fold(0, |acc, x| {
            if matches!(x.direction, Direction::Down) {
                acc + x.distance
            } else {
                acc - x.distance
            }
        });

    println!("{} distance", horizontal_distance * vertical_distance);
}

fn parse_instruction(s: &str) -> Instruction {
    let tmp = s.split(' ').collect::<Vec<&str>>();

    let direction = tmp[0];
    let distance = tmp[1].parse::<i32>().unwrap();

    return Instruction {
        direction: parse_direction(&direction).unwrap(),
        distance: distance,
    };
}

fn parse_direction(s: &str) -> Result<Direction, &'static str> {
    match s {
        "forward" => Ok(Direction::Forward),
        "down" => Ok(Direction::Down),
        "up" => Ok(Direction::Up),
        _ => Err("Unknown direction"),
    }
}

struct Instruction {
    direction: Direction,
    distance: i32,
}

enum Direction {
    Forward,
    Down,
    Up,
}
