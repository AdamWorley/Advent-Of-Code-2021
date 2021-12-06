use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("Reading {}", filename);

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let raw: Vec<i8> = reader
        .lines()
        .map(|line| line.unwrap())
        .flat_map(|l| {
            l.split(',')
                .map(|v| v.parse::<i8>().unwrap())
                .collect::<Vec<i8>>()
        })
        .collect();

    let fish: HashSet<i8> = raw.clone().into_iter().collect::<HashSet<_>>();

    let mut fish_count: HashMap<i8, usize> = HashMap::new();

    for f in &fish {
        fish_count.insert(*f, raw.iter().filter(|x| x == &f).count());
    }

    (0..256).for_each(|_d| {
        let spawning_fish: usize = match fish_count.get(&0) {
            Some(n) => *n,
            None => 0,
        };

        for g in 1..9 {
            fish_count.insert(
                g - 1,
                match fish_count.get(&g) {
                    Some(n) => *n,
                    None => 0,
                },
            );
        }

        fish_count.insert(8, spawning_fish);
        fish_count.insert(
            6,
            spawning_fish
                + match fish_count.get(&6) {
                    Some(n) => *n,
                    None => 0,
                },
        );
    });

    let result: usize = fish_count.into_iter().map(|(_x, y)| y).sum();

    println!("{} fish", result);
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
