use std::env;
use std::string::String;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


fn find_digit(line: &str, w: &[(&str, u32)], begin: usize, left: bool) -> u32 {
    let mut idx = begin;
    loop {
        if let Some(v) = line[idx..].chars().next().unwrap().to_digit(10) {
            return v;
        }
        for (p, v) in w {
            if line[idx..].starts_with(p) {
                return *v;
            }
        }
        idx = if left {idx + 1} else {idx - 1};
    }
}

fn find_calibration(line: &str, w: &[(&str, u32)]) -> u32 {
    let left = find_digit(line, w, 0, true);
    let right = find_digit(line, w, line.len() - 1, false);
    left * 10 + right
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = Path::new(&args[1]);

    let file = match File::open(&filename) {
        Err(why) => panic!("couldn't open input file {:?}: {}", filename, why),
        Ok(file) => file,
    };

    println!("Reading file: {:?}: {:?}", filename, file);

    let words = [
        ("zero", 0u32),
        ("one", 1u32),
        ("two", 2u32),
        ("three", 3u32),
        ("four", 4u32),
        ("five", 5u32),
        ("six", 6u32),
        ("seven", 7u32),
        ("eight", 8u32),
        ("nine", 9u32),
    ];
    let mut calibration_value = 0;
    let mut adj_calibration = 0;
    let reader = BufReader::new(file);
    for l in reader.lines() {
        let line = l.expect("Error reading file");

        calibration_value += find_calibration(&line, &[]);
        adj_calibration += find_calibration(&line, &words);
        
    }

    println!("Calibration value is: {calibration_value}");
    println!("Adjusted Calibration value is: {adj_calibration}");
}
