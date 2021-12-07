use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    let numbers = get_numbers(&args[1]);
    let boards: Vec<BingoBoard> = get_boards(&args[2]);

    for n in 5..numbers.len() {
        println!("round {}", n);
        let to_match: Vec<usize> = numbers.clone().into_iter().take(n).collect();

        let bingo = if let Some(bb) = boards.iter().find(|b| b.is_bingo(to_match.clone())) {
            println!("BINGO!");
            bb
        } else {
            print!("no winners :(");
            continue;
        };

        let last = to_match.last().unwrap();
        let unmatched = bingo.unmatched(to_match.clone());
        println!(
            "{}: {} x {} = {}",
            n,
            last,
            unmatched.iter().sum::<usize>(),
            last * unmatched.iter().sum::<usize>()
        );

        return;
    }
}

fn get_numbers(file_name: &str) -> Vec<usize> {
    println!("Reading {}", file_name);

    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let numbers = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    return numbers[0]
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
}

fn get_boards(file_name: &str) -> Vec<BingoBoard> {
    println!("Reading {}", file_name);

    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let tmp = reader
        .lines()
        .map(|line| line.unwrap())
        .filter(|l| !l.is_empty())
        .collect::<Vec<String>>();

    (0..tmp.len() / 5)
        .map(|n| {
            tmp.clone()
                .into_iter()
                .skip(n * 5)
                .take(5)
                .collect::<Vec<String>>()
        })
        .map(BingoBoard::new)
        .collect::<Vec<BingoBoard>>()
}

struct BingoBoard {
    pub board: Vec<usize>,
    pub board_size: usize,
}

impl BingoBoard {
    fn get_rows(&self) -> Vec<Vec<usize>> {
        (0..self.board_size)
            .map(|i| {
                self.board
                    .clone()
                    .into_iter()
                    .skip(i * self.board_size)
                    .take(5)
                    .collect::<Vec<usize>>()
            })
            .collect()
    }

    fn get_cols(&self) -> Vec<Vec<usize>> {
        (0..self.board_size)
            .map(|i| self.get_rows().iter().map(|x| x[i]).collect::<Vec<usize>>())
            .collect()
    }

    fn is_bingo(&self, to_match: Vec<usize>) -> bool {
        self.get_rows()
            .into_iter()
            .any(|item| item.iter().all(|r| to_match.contains(r)))
            || self
                .get_cols()
                .into_iter()
                .any(|item| item.iter().all(|r| to_match.contains(r)))
    }

    fn unmatched(&self, to_match: Vec<usize>) -> Vec<usize> {
        self.board
            .clone()
            .into_iter()
            .filter(|v| !to_match.contains(v))
            .collect()
    }

    fn new(values: Vec<String>) -> BingoBoard {
        let parsed_values = values
            .iter()
            .flat_map(|s| s.split_whitespace())
            .map(|v| v.parse::<usize>().unwrap_or(0))
            .collect::<Vec<usize>>();

        BingoBoard {
            board_size: 5,
            board: parsed_values,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_rows_test() {
        let input = "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25".split(',');
        let board = BingoBoard::new(input.map(|f| f.to_string()).collect::<Vec<String>>());
        let result = board.get_rows();

        assert_eq!(result.len(), 5);
    }

    #[test]
    fn get_cols_test() {
        let input = "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25".split(',');
        let board = BingoBoard::new(input.map(|f| f.to_string()).collect::<Vec<String>>());
        let result = board.get_cols();

        assert_eq!(result.len(), 5);
    }

    #[test]
    fn is_bingo_test() {
        let input = "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25".split(',');
        let board = BingoBoard::new(input.map(|f| f.to_string()).collect::<Vec<String>>());
        let result = board.is_bingo(vec![1, 2, 3]);

        assert_eq!(result, false);
    }

    #[test]
    fn is_bingo_test_should_be_true() {
        let input = "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25".split(',');
        let board = BingoBoard::new(input.map(|f| f.to_string()).collect::<Vec<String>>());
        let result = board.is_bingo(vec![1, 2, 3, 4, 5]);

        assert_eq!(result, true);
    }

    #[test]
    fn is_bingo_test_should_be_false() {
        let input = "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25".split(',');
        let board = BingoBoard::new(input.map(|f| f.to_string()).collect::<Vec<String>>());
        let result = board.is_bingo(vec![1, 6, 11, 16, 21]);

        assert_eq!(result, true);
    }

    #[test]
    fn unmatched() {
        let input = "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25".split(',');
        let board = BingoBoard::new(input.map(|f| f.to_string()).collect::<Vec<String>>());
        let result = board.unmatched(vec![1]);

        assert_eq!(result.len(), 24);
    }

    #[test]
    fn unmatched_no_matches() {
        let input = "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25".split(',');
        let board = BingoBoard::new(input.map(|f| f.to_string()).collect::<Vec<String>>());
        let result = board.unmatched(vec![99]);

        assert_eq!(result.len(), 25);
    }
}
