use std::collections::HashSet;
use std::io;

use util;

fn part1(lines: impl Iterator<Item = Result<String, io::Error>>) -> Result<u32, io::Error> {
    let mut acc = 0;
    for line in lines {
        let Some((_, data)) = line.as_ref().unwrap().split_once(':') else {
            println!("malformed line: {}", line?);
            continue;
        };

        let Some((winners, have)) = data.split_once('|') else {
            println!("malformed data: {data}");
            continue;
        };

        let winners: HashSet<u32> = winners
            .split(' ')
            .filter(|x| *x != "")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        acc += (1u32
            << have
                .split(' ')
                .filter(|x| *x != "")
                .map(|x| x.parse::<u32>().unwrap())
                .filter(|x| winners.contains(x))
                .count())
            >> 1;
    }
    Ok(acc)
}

fn part2(lines: impl Iterator<Item = Result<String, io::Error>>) -> Result<u32, io::Error> {
    let cards = lines
        .map(|x| {
            x.unwrap()
                .split_once(':')
                .unwrap()
                .1
                .split_once('|')
                .map(|(w, h)| {
                    (
                        w.split(' ')
                            .filter(|x| *x != "")
                            .map(|x| x.parse::<u32>().unwrap())
                            .collect::<HashSet<u32>>(),
                        h.split(' ')
                            .filter(|x| *x != "")
                            .map(|x| x.parse::<u32>().unwrap())
                            .collect::<Vec<u32>>(),
                    )
                })
                .unwrap()
        })
        .collect::<Vec<_>>();

    let mut num_cards = vec![1; cards.len()];
    for (idx, (w, h)) in cards.iter().enumerate() {
        let winners = h.iter().filter(|x| w.contains(x)).count();
        for idx2 in (idx + 1)..=(idx + winners) {
            if idx2 >= num_cards.len() {
                break;
            }
            num_cards[idx2] += num_cards[idx];
        }
    }

    Ok(num_cards.iter().sum())
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    println!(
        "part1 = {}",
        part1(util::read_file(&args[1]).unwrap()).unwrap()
    );
    println!(
        "part2 = {}",
        part2(util::read_file(&args[1]).unwrap()).unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        assert_eq!(part1(util::read_testdata(TEST_DATA).unwrap()).unwrap(), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(util::read_testdata(TEST_DATA).unwrap()).unwrap(), 30);
    }
}
