use std::env;
use std::string::String;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = Path::new(&args[1]);

    let file = match File::open(&filename) {
        Err(why) => panic!("couldn't open input file {:?}: {}", filename, why),
        Ok(file) => file,
    };

    println!("Reading file: {:?}: {:?}", filename, file);

    let mut total = 0;
    let mut powset = 0;
    let reader = BufReader::new(file);
    for l in reader.lines() {
        let line = l.expect("Error reading file");

        let (p1, p2) = parse_game(&line, 12, 13, 14);
        total += p1;
        powset += p2;
    }

    println!("Total: {total}");
    println!("Power Set: {powset}");
}

fn parse_game(game_data: &str, red: u32, green: u32, blue: u32) -> (u32, u32) {
    let (game_str, pulls_str) = game_data.split_once(':').unwrap();
    let pulls_iter = pulls_str.trim().split(';');
    let mut max = (0, 0, 0);

    for pull in pulls_iter {
        let colors = pull.trim().split(',');
        for color in colors {
            let (nstr, cstr) = color.trim().split_once(' ').unwrap();
            let num = u32::from_str_radix(nstr, 10).unwrap();
            max = match cstr.trim() {
                "red" => (num.max(max.0), max.1, max.2),
                "green" => (max.0, num.max(max.1), max.2),
                "blue" => (max.0, max.1, num.max(max.2)),
                _ => max,
            };
        }
    }
   
    let (r, g, b) = max;
    let power_set = r * g * b;
    if r <= red && g <= green && b <= blue {
        let (_, game_num_str) = game_str.split_once(' ').unwrap();
        return (u32::from_str_radix(game_num_str, 10).unwrap(), power_set);
    }
    (0, power_set)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA : &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1() {
        let mut total = 0;
        let mut powset = 0;
        for line in TEST_DATA.lines() {
            let (p1, p2) = parse_game(line, 12, 13, 14);
            total += p1;
            powset += p2;
        }
        assert_eq!(total, 8);
        assert_eq!(powset, 2286);
    }
}
