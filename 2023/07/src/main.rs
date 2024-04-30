use std::cmp;
use std::collections::HashMap;
use std::io;
use util;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, PartialOrd, Ord)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn new(value: char, with_joker: bool) -> Self {
        match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => {
                if with_joker {
                    Self::Joker
                } else {
                    Self::Jack
                }
            }
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("Invalid input {value}"),
        }
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Ord, Eq)]
struct Hand {
    r#type: HandType,
    cards: [Card; 5],
    bid: usize,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.cards.eq(&other.cards)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        let c = self.r#type.cmp(&other.r#type);
        if c != cmp::Ordering::Equal {
            return Some(c);
        }

        for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            let c = self_card.cmp(other_card);
            if c != cmp::Ordering::Equal {
                return Some(c);
            }
        }

        None
    }
}

impl Hand {
    fn new(value: String, with_joker: bool) -> Self {
        let (handstr, bidstr) = value.split_once(' ').unwrap();
        let cards: [Card; 5] = handstr
            .chars()
            .map(|x| Card::new(x, with_joker))
            .collect::<Vec<_>>()
            .try_into()
            .expect("unable to build array");
        let bid = bidstr.parse::<usize>().expect("expected an integer!");
        let mut counts: HashMap<Card, usize> = HashMap::new();
        for c in cards.iter() {
            if let Some(t) = counts.get_mut(c) {
                *t = *t + 1;
            } else {
                counts.insert(*c, 1);
            }
        }
        let jokers = counts.remove(&Card::Joker).unwrap_or(0);
        let mut sorted_counts = counts.into_values().collect::<Vec<_>>();
        sorted_counts.sort_unstable_by(|a, b| b.cmp(a));

        let r#type = match sorted_counts.get(0).unwrap_or(&0) + jokers {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => match sorted_counts.get(1).unwrap_or(&0) {
                2 => HandType::FullHouse,
                _ => HandType::ThreeOfAKind,
            },
            2 => match sorted_counts.get(1).unwrap_or(&0) {
                2 => HandType::TwoPair,
                _ => HandType::OnePair,
            },
            _ => HandType::HighCard,
        };
        Self { r#type, cards, bid }
    }
}

fn calculate_winnings(
    lines: impl Iterator<Item = Result<String, io::Error>>,
    with_jokers: bool,
) -> Result<usize, io::Error> {
    let mut hands = lines
        .map(|l| l.map(|x| Hand::new(x, with_jokers)))
        .collect::<Result<Vec<_>, _>>()?;

    hands.sort_unstable();

    Ok(hands
        .into_iter()
        .enumerate()
        .map(|(r, h)| (r + 1) * h.bid)
        .sum())
}

fn part1(lines: impl Iterator<Item = Result<String, io::Error>>) -> Result<usize, io::Error> {
    calculate_winnings(lines, false)
}

fn part2(lines: impl Iterator<Item = Result<String, io::Error>>) -> Result<usize, io::Error> {
    calculate_winnings(lines, true)
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

    const TEST_DATA: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(util::read_testdata(TEST_DATA).unwrap()).unwrap(),
            6440
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(util::read_testdata(TEST_DATA).unwrap()).unwrap(),
            5905
        );
    }
}
