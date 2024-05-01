use std::collections::HashMap;
use std::io;
use util;

fn parse_data(
    lines: &mut impl Iterator<Item = Result<String, io::Error>>,
) -> Result<(String, HashMap<String, (String, String)>), io::Error> {
    let path = match lines.next() {
        None => return Err(io::Error::from(io::ErrorKind::UnexpectedEof)),
        Some(Err(err)) => return Err(err),
        Some(Ok(l)) => l,
    };
    assert_eq!(lines.next().expect("Truncated input")?, "");

    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    for l in lines {
        let l2 = l?;
        let kv = l2.split_once(" = ").expect("malformed line");
        let kv = (
            String::from(kv.0),
            kv.1[1..(kv.1.len() - 1)]
                .split_once(", ")
                .expect("malformed line"),
        );
        let kv = (kv.0, (String::from(kv.1 .0), String::from(kv.1 .1)));
        nodes.insert(kv.0, kv.1);
    }

    Ok((path, nodes))
}

fn take_step<'a>(
    map: &'a HashMap<String, (String, String)>,
    loc: &str,
    step: Option<char>,
) -> &'a str {
    let next_tuple = map.get(loc).expect("location not found!");
    match step {
        Some('L') => &next_tuple.0,
        Some('R') => &next_tuple.1,
        _ => panic!("malformed path!"),
    }
}

fn cycle_count(
    map: &HashMap<String, (String, String)>,
    path: &str,
    start: &str,
    end: &str,
) -> (Option<usize>, usize) {
    // Total number of steps taken
    let mut counter = 0;
    // A cyclic enumerated character iterator describing the path
    let mut path_iter = path.chars().enumerate().cycle();
    // The visited nodes as indicated by node name and location on the cyclic path. Visiting a node
    // on the first step of the path is not the same as visiting it on the nth because the
    // following steps will differ. We map from this to the absolute number of steps taken when we
    // first encountered the start of the cycle.
    let mut visited: HashMap<(&str, usize), usize> = HashMap::new();
    // The node being examined.
    let mut cur = start;
    // The counter value when we first encounted an 'end node'.
    let mut end_counter = None;
    // Walk the map until we find a cycle and get the length of that cycle. As a side effect, we
    // calculate the number of steps until we see the first 'end node'.
    let cycle_length = loop {
        let step = path_iter.next().unwrap();
        let v = (cur, step.0);
        if let Some(seen) = visited.get(&v) {
            break counter - seen;
        } else {
            visited.insert(v, counter);
        }
        if cur.ends_with(end) && end_counter.is_none() {
            end_counter = Some(counter);
        }
        cur = take_step(map, cur, Some(step.1));
        counter += 1;
    };
    (end_counter, cycle_length)
}

fn part1(lines: impl Iterator<Item = Result<String, io::Error>>) -> Result<usize, io::Error> {
    let (path, nodes) = parse_data(&mut lines.into_iter())?;
    let (end_counter, _) = cycle_count(&nodes, &path, "AAA", "ZZZ");
    Ok(end_counter.expect("no end found!"))
}

fn part2(lines: impl Iterator<Item = Result<String, io::Error>>) -> Result<usize, io::Error> {
    let (path, nodes) = parse_data(&mut lines.into_iter())?;
    // It feels wrong that we ignore the number of steps until the first end node. Really it should
    // be that number plus some number of cycles. The math for that is a bit complicated and it
    // turns out that end_counter == cycle_length for our data, so we don't have to address this.
    // Or the case where there are multiple terminal nodes in a path, etc.
    let result = nodes
        .keys()
        .filter(|x| x.ends_with("A"))
        .map(|x| cycle_count(&nodes, &path, x, "Z"))
        .map(|(_, c)| c)
        .fold(1, num::integer::lcm);

    Ok(result)
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

    const TEST_DATA: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const TEST_DATA2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const TEST_DATA3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_part1() {
        assert_eq!(part1(util::read_testdata(TEST_DATA).unwrap()).unwrap(), 2);
    }

    #[test]
    fn test_part1a() {
        assert_eq!(part1(util::read_testdata(TEST_DATA2).unwrap()).unwrap(), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(util::read_testdata(TEST_DATA3).unwrap()).unwrap(), 6);
    }
}
