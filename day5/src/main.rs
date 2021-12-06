use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use vecmath::Vector2;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("Reading {}", filename);

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let vents: Vec<(Vector2<f64>, Vector2<f64>)> = reader
        .lines()
        .map(|line| parse_instruction(&line.unwrap()))
        .collect();

    let foo: Vec<(Vector2<f64>, Vector2<f64>)> = vents
        .iter()
        .zip(vents.iter().skip(1))
        .filter(|(current, next)| {
            horizontal_overlap(current, next) || vertical_overlap(current, next)
        })
        .collect();

    println!("Reading {}", foo.len());
}

fn horizontal_overlap(p1: (Vector2<f64>, Vector2<f64>), p2: (Vector2<f64>, Vector2<f64>)) -> bool {
    return p1.0[0] <= p2.0[0] && p1.1[0] >= p2.1[0];
}

fn vertical_overlap(p1: (Vector2<f64>, Vector2<f64>), p2: (Vector2<f64>, Vector2<f64>)) -> bool {
    return p1.0[1] <= p2.0[1] && p1.1[1] >= p2.1[1];
}

fn parse_instruction(s: &str) -> (Vector2<f64>, Vector2<f64>) {
    let points = s.split("->").collect::<Vec<&str>>();

    let pos1 = points[0]
        .trim()
        .split(',')
        .map(|x| x.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    let pos2 = points[1]
        .trim()
        .split(',')
        .map(|x| x.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    return ([pos1[0], pos1[1]], [pos2[0], pos2[1]]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal_overlap_test_does_overlap() {
        let p1: (Vector2<f64>, Vector2<f64>) = ([0.0, 0.0], [5.0, 0.0]);
        let p2: (Vector2<f64>, Vector2<f64>) = ([0.0, 0.0], [5.0, 0.0]);

        let result = horizontal_overlap(p1, p2);

        assert_eq!(result, true);
    }

    #[test]
    fn horizontal_overlap_test_no_overlap() {
        let p1: (Vector2<f64>, Vector2<f64>) = ([6.0, 0.0], [10.0, 0.0]);
        let p2: (Vector2<f64>, Vector2<f64>) = ([0.0, 0.0], [5.0, 0.0]);

        let result = horizontal_overlap(p1, p2);

        assert_eq!(result, false);
    }

    #[test]
    fn horizontal_overlap_test_point_overlap() {
        let p1: (Vector2<f64>, Vector2<f64>) = ([6.0, 0.0], [10.0, 0.0]);
        let p2: (Vector2<f64>, Vector2<f64>) = ([7.0, 0.0], [7.0, 0.0]);

        let result = horizontal_overlap(p1, p2);

        assert_eq!(result, true);
    }

    #[test]
    fn vertical_overlap_test_no_overlap() {
        let p1: (Vector2<f64>, Vector2<f64>) = ([0.0, 1.0], [5.0, 1.0]);
        let p2: (Vector2<f64>, Vector2<f64>) = ([0.0, 0.0], [5.0, 0.0]);

        let result = vertical_overlap(p1, p2);

        assert_eq!(result, false);
    }

    #[test]
    fn vertical_overlap_test_does_overlap() {
        let p1: (Vector2<f64>, Vector2<f64>) = ([0.0, 1.0], [5.0, 1.0]);
        let p2: (Vector2<f64>, Vector2<f64>) = ([0.0, 1.0], [5.0, 1.0]);

        let result = vertical_overlap(p1, p2);

        assert_eq!(result, true);
    }
}
