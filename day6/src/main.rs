use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("Reading {}", filename);

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut fish: Vec<i8> = reader
        .lines()
        .map(|line| line.unwrap())
        .flat_map(|l| {
            l.split(',')
                .map(|v| v.parse::<i8>().unwrap())
                .collect::<Vec<i8>>()
        })
        .collect();

    println!("{} fish", fish.len());

    (0..80).for_each(|_d| fish = fish.iter().flat_map(|f| new_day(f)).collect::<Vec<i8>>());

    println!("{} fish", fish.len());
}

fn new_day(f: &i8) -> Vec<i8> {
    if f == &0 {
        return vec![6, 8];
    } else {
        return vec![f - 1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_day_no_new_fish() {
        let input = &6;
        let result = new_day(input);

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn new_day_fish_age_decreases() {
        let input = &6;
        let result = new_day(input);

        assert_eq!(result[0], 5);
    }

    #[test]
    fn new_day_test_new_fish() {
        let input = &0;
        let result = new_day(input);

        assert_eq!(result.len(), 2);
    }
}
