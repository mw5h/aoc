use std::collections::BTreeMap;
use std::io;

use util;

#[derive(Debug)]
struct Mapper {
    map: BTreeMap<usize, (usize, usize)>,
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

    fn add_mapping(&mut self, from: usize, to: usize, len: usize) -> () {
        self.map.insert(from, (to, len));
    }

    fn lookup(&self, from: usize) -> usize {
        for (f, (t, l)) in self.map.iter() {
            if *f > from {
                break;
            }

            if *f <= from && *f + *l > from {
                return *t + (from - *f);
            }
        }

        from
    }
}

fn part1(lines: impl Iterator<Item = Result<String, io::Error>>) -> Result<usize, io::Error> {
    let mut seeds = Vec::<usize>::new();
    let mut mapper = Mapper::new();
    for line in lines {
        let l = line?;

        if l.contains("seeds: ") {
            let s = l.strip_prefix("seeds: ").unwrap();
            seeds = s
                .split(' ')
                .map(|x| x.parse::<usize>().expect("expected integer!"))
                .collect();
            continue;
        }

        if l == "" {
            continue;
        }

        if l.contains("map:") {
            seeds = seeds.into_iter().map(|x| mapper.lookup(x)).collect();
            mapper.reset();
            continue;
        }

        let new_mapping = l
            .split(' ')
            .map(|x| x.parse::<usize>().expect("expected integer!"))
            .collect::<Vec<_>>();
        mapper.add_mapping(new_mapping[1], new_mapping[0], new_mapping[2]);
    }
    Ok(seeds
        .into_iter()
        .map(|x| mapper.lookup(x))
        .min()
        .expect("no minimum value!"))
}

fn part2(lines: impl Iterator<Item = Result<String, io::Error>>) -> Result<usize, io::Error> {
    Ok(0)
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    println!(
        "part1 = {}",
        part1(util::read_file(&args[1]).unwrap()).unwrap()
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
        assert_eq!(part2(util::read_testdata(TEST_DATA).unwrap()).unwrap(), 30);
    }
}
