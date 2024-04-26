use std::io;
use util;

fn solve_quadratic(a: i64, b: i64, c: i64) -> (f64, f64) {
    let plusminus = ((b * b - 4 * a * c) as f64).sqrt();
    let denom = (2 * a) as f64;
    (
        (-b as f64 - plusminus) / denom,
        (-b as f64 + plusminus) / denom,
    )
}

fn possible_wins(time_ms: u64, distance_mm: u64) -> u64 {
    let neg_time = -(time_ms as i64);
    let exact_cutoffs = solve_quadratic(1, neg_time, distance_mm as i64);
    // This is an inequality, so we want the integers that are strictly inside the range
    let low_val = (exact_cutoffs.0 + 1.0).floor() as u64;
    let high_val = (exact_cutoffs.1 - 1.0).ceil() as u64;
    high_val - low_val + 1
}

fn part1(lines: impl Iterator<Item = Result<String, io::Error>>) -> Result<u64, io::Error> {
    let fields = ["Time", "Distance"];
    let mut data = lines
        .zip(fields)
        .map(|(l, f)| {
            l.expect("i/o error")
                .strip_prefix(&format!("{f}:"))
                .expect("Expected first line to be {f}")
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<u64>().expect("expected integer!"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let distances = data.pop().unwrap();
    let times = data.pop().unwrap();
    assert_eq!(data.len(), 0);

    let wins = times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(t, d)| possible_wins(t, d))
        .fold(1, |a, x| a * x);
    Ok(wins)
}

fn part2(lines: impl Iterator<Item = Result<String, io::Error>>) -> Result<u64, io::Error> {
    let fields = ["Time", "Distance"];
    let mut data = lines
        .zip(fields)
        .map(|(l, f)| {
            l.expect("i/o error")
                .strip_prefix(&format!("{f}:"))
                .expect("Expected first line to be {f}")
                .chars()
                .filter(|x| x.is_digit(10))
                .collect::<String>()
                .parse::<u64>()
                .expect("expected integer!")
        })
        .collect::<Vec<_>>();

    let distance = data.pop().unwrap();
    let time = data.pop().unwrap();
    assert_eq!(data.len(), 0);

    return Ok(possible_wins(time, distance));
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    println!(
        "part1 = {}  part2 = {}",
        part1(util::read_file(&args[1]).unwrap()).unwrap(),
        part2(util::read_file(&args[1]).unwrap()).unwrap(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1() {
        assert_eq!(part1(util::read_testdata(TEST_DATA).unwrap()).unwrap(), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(util::read_testdata(TEST_DATA).unwrap()).unwrap(),
            71503
        );
    }
}
