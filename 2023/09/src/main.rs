use std::io;
use util;

fn calculate_next(steps: &Vec<isize>) -> isize {
    if steps.into_iter().all(|x| *x == 0) {
        0
    } else {
        let next_steps = steps.windows(2).map(|x| x[1] - x[0]).collect();
        steps.last().unwrap() + calculate_next(&next_steps)
    }
}

fn part1(lines: impl Iterator<Item = Result<String, io::Error>>) -> Result<isize, io::Error> {
    let res: isize = lines
        .map(|x| {
            x.expect("io error")
                .split_whitespace()
                .map(|y| y.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|x| calculate_next(&x))
        .sum();

    Ok(res)
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    println!(
        "part1 = {}  part2 = {}",
        part1(util::read_file(&args[1]).unwrap()).unwrap(),
        part1(util::read_file(&args[1]).unwrap()).unwrap(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part1() {
        assert_eq!(part1(util::read_testdata(TEST_DATA).unwrap()).unwrap(), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part1(util::read_testdata(TEST_DATA).unwrap()).unwrap(), 114);
    }
}
