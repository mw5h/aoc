use std::collections::BTreeMap;
use std::io;

use util;

#[derive(Debug)]
struct Mapper {
    map: BTreeMap<u64, (u64, u64, i64)>,
}

impl Mapper {
    fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    fn reset(&mut self) -> () {
        self.map.clear();
    }

    fn add_mapping(&mut self, from: u64, to: u64, len: u64) -> () {
        self.map
            .insert(from, (from, from + len - 1, to as i64 - from as i64));
    }

    fn lookup(&self, from: u64, to: u64) -> Vec<(u64, u64)> {
        let mut result = Vec::<(u64, u64)>::new();
        let mut from_idx = from;
        for (_, (f, t, d)) in self.map.iter() {
            if *t < from_idx {
                continue;
            }

            if to < *f {
                continue;
            }

            if from_idx < *f {
                result.push((from_idx, *f - 1));
                from_idx = *f;
            }

            let new_from = from_idx
                .checked_add_signed(*d)
                .expect("from value underflow");
            if to <= *t {
                result.push((
                    new_from,
                    to.checked_add_signed(*d)
                        .expect("to value underflow {to} + {d}"),
                ));
                from_idx = to + 1;
                break;
            }

            result.push((
                new_from,
                t.checked_add_signed(*d).expect("to value underflow"),
            ));
            from_idx = *t + 1;
        }

        if from_idx <= to {
            result.push((from_idx, to));
        }

        result
    }
}

fn process_data(
    lines: impl Iterator<Item = Result<String, io::Error>>,
    seed_parser: fn(&str) -> Vec<(u64, u64)>,
) -> Result<u64, io::Error> {
    let mut seeds = Vec::<(u64, u64)>::new();
    let mut mapper = Mapper::new();
    for line in lines {
        let l = line?;

        if l.contains("seeds: ") {
            let s = l.strip_prefix("seeds: ").unwrap();
            seeds = seed_parser(s);
            continue;
        }

        if l == "" {
            continue;
        }

        if l.contains("map:") {
            seeds = seeds
                .into_iter()
                .map(|(f, t)| mapper.lookup(f, t))
                .flatten()
                .collect();
            mapper.reset();
            continue;
        }

        let new_mapping = l
            .split(' ')
            .map(|x| x.parse::<u64>().expect("expected integer!"))
            .collect::<Vec<_>>();
        mapper.add_mapping(new_mapping[1], new_mapping[0], new_mapping[2]);
    }
    Ok(seeds
        .into_iter()
        .map(|(f, t)| mapper.lookup(f, t))
        .flatten()
        .map(|(f, _)| f)
        .min()
        .expect("no minimum value!"))
}

fn part1_seeds(seed_string: &str) -> Vec<(u64, u64)> {
    seed_string
        .split(' ')
        .map(|x| x.parse::<u64>().expect("expected integer!"))
        .map(|x| (x, x))
        .collect()
}

fn part1(lines: impl Iterator<Item = Result<String, io::Error>>) -> Result<u64, io::Error> {
    process_data(lines, part1_seeds)
}

fn part2_seeds(seed_string: &str) -> Vec<(u64, u64)> {
    seed_string
        .split(' ')
        .map(|x| x.parse::<u64>().expect("expected integer!"))
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|x| (x[0], x[0] + x[1] - 1))
        .collect()
}

fn part2(lines: impl Iterator<Item = Result<String, io::Error>>) -> Result<u64, io::Error> {
    process_data(lines, part2_seeds)
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    println!(
        "part1 = {}  part2 = {}",
        part1(util::read_file(&args[1]).unwrap()).unwrap(),
        part2(util::read_file(&args[1]).unwrap()).unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part1() {
        assert_eq!(part1(util::read_testdata(TEST_DATA).unwrap()).unwrap(), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(util::read_testdata(TEST_DATA).unwrap()).unwrap(), 46);
    }
}
