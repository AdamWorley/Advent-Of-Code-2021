use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("Reading {}", filename);

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let readings: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let length = readings[0].len();

    let gamma_string: String = (0..length)
        .map(|n| {
            let tmp = readings
                .iter()
                .map(|v| v.chars().nth(n).unwrap())
                .collect::<String>();

            return if number_of_ones(&tmp) > number_of_zeros(&tmp) {
                '1'
            } else {
                '0'
            };
        })
        .collect();

    let gamma = isize::from_str_radix(&gamma_string, 2).unwrap();

    let epsilon_string: String = (0..length)
        .map(|n| {
            let tmp = readings
                .iter()
                .map(|v| v.chars().nth(n).unwrap())
                .collect::<String>();

            return if number_of_ones(&tmp) < number_of_zeros(&tmp) {
                '1'
            } else {
                '0'
            };
        })
        .collect();

    let epsilon = isize::from_str_radix(&epsilon_string, 2).unwrap();

    let oxygen_string = filter_by_common(&readings, 0, '1')[0].to_string();
    let oxygen = isize::from_str_radix(&oxygen_string, 2).unwrap();

    let co2_string = filter_by_least_common(&readings, 0, '0')[0].to_string();
    let co2 = isize::from_str_radix(&co2_string, 2).unwrap();

    println!("{}", gamma * epsilon);
    println!("{}", oxygen * co2);
}

fn number_of_ones(s: &String) -> usize {
    return s.chars().filter(|x| x == &'1').count();
}

fn number_of_zeros(s: &String) -> usize {
    return s.chars().filter(|x| x == &'0').count();
}

fn common(readings: &Vec<String>, position: usize, default: char) -> char {
    let chars_at_pos = readings
        .into_iter()
        .map(|r| {
            return r.chars().nth(position).unwrap();
        })
        .collect::<String>();

    return if number_of_ones(&chars_at_pos) == number_of_zeros(&chars_at_pos) {
        return default;
    } else if number_of_ones(&chars_at_pos) > number_of_zeros(&chars_at_pos) {
        '1'
    } else {
        '0'
    };
}

fn least_common(readings: &Vec<String>, position: usize, default: char) -> char {
    let chars_at_pos = readings
        .into_iter()
        .map(|r| {
            return r.chars().nth(position).unwrap();
        })
        .collect::<String>();

    return if number_of_ones(&chars_at_pos) == number_of_zeros(&chars_at_pos) {
        return default;
    } else if number_of_ones(&chars_at_pos) < number_of_zeros(&chars_at_pos) {
        '1'
    } else {
        '0'
    };
}

fn filter_by_common(readings: &Vec<String>, position: usize, default: char) -> Vec<String> {
    if readings.len() <= 1 {
        return readings.to_vec();
    } else {
        let c = common(readings, position, default);
        let filtered: Vec<String> = readings
            .to_vec()
            .into_iter()
            .filter(|x| x.chars().nth(position).unwrap() == c)
            .collect();

        return filter_by_common(&filtered, position + 1, default);
    };
}

fn filter_by_least_common(readings: &Vec<String>, position: usize, default: char) -> Vec<String> {
    if readings.len() <= 1 {
        return readings.to_vec();
    } else {
        let c = least_common(readings, position, default);
        let filtered: Vec<String> = readings
            .to_vec()
            .into_iter()
            .filter(|x| x.chars().nth(position).unwrap() == c)
            .collect();

        return filter_by_least_common(&filtered, position + 1, default);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_test() {
        let input: Vec<String> = vec!["10".to_string(), "00".to_string(), "00".to_string()];

        let result = common(&input, 0, '1');

        assert_eq!(result, '0');
    }

    #[test]
    fn filter_by_common_test() {
        let input: Vec<String> = vec!["10".to_string(), "00".to_string(), "01".to_string()];

        let result = filter_by_common(&input, 0, '1');

        assert_eq!(result.len(), 1);
    }
}
