use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("Reading {}", filename);

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut raw: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap())
        .flat_map(|l| {
            l.split(',')
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    raw.sort();

    let median = median(&raw);
    let mean = mean(&raw);

    let fuel: i64 = raw
        .into_iter()
        .map(|r| something((mean as i64 - r as i64).abs()))
        .sum();

    println!("{}", fuel);
}

fn mean(list: &[i32]) -> f64 {
    let sum: i32 = Iterator::sum(list.iter());
    f64::from(sum) / (list.len() as f64)
}

fn median(list: &[i32]) -> f64 {
    let len = list.len();
    let mid = len / 2;
    if len % 2 == 0 {
        mean(&list[(mid - 1)..(mid + 1)])
    } else {
        f64::from(list[mid])
    }
}

fn fib(num: i64) -> i64 {
    match num {
        0 => 1,
        1 => 1,
        _ => fib(num - 1) + num,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_test() {
        let input = 1;
        let result = fib(input);

        assert_eq!(result, 1);
    }

    #[test]
    fn fib_4_test() {
        let input = 4;
        let result = fib(input);

        assert_eq!(result, 10);
    }

    #[test]
    fn fib_11_test() {
        let input = 11;
        let result = fib(input);

        assert_eq!(result, 66);
    }
}
