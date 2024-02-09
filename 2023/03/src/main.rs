use std::collections::{LinkedList, VecDeque};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::iter::*;
use std::path::Path;

#[derive(PartialEq, Debug, Copy, Clone)]
enum SchematicItemType {
    PartNumber(u32),
    Symbol(char),
}

#[derive(Debug, Copy, Clone)]
struct SchematicItem {
    x: usize,
    y: usize,
    len: usize,
    typ: SchematicItemType,
}

impl SchematicItem {
    fn is_adj(&self, other: &SchematicItem) -> bool {
        let min_y = if other.y < 1 { 0 } else { other.y - 1 };
        let max_x = other.x + other.len;
        let max_y = other.y + 1;

        if self.y < min_y || self.y > max_y {
            return false;
        }

        if self.x + self.len < other.x || self.x > max_x {
            return false;
        }

        return true;
    }

    fn is_match(&self, other: &SchematicItem) -> bool {
        match (&self.typ, &other.typ) {
            (SchematicItemType::PartNumber(_), SchematicItemType::Symbol(_)) => true,
            (SchematicItemType::Symbol(_), SchematicItemType::PartNumber(_)) => true,
            _ => false,
        }
    }
}

type LineReader = dyn Iterator<Item = Result<String, Error>>;

#[derive(Debug)]
struct TokenReader {
    char_stream: VecDeque<(usize, usize, char)>,
}

impl TryFrom<&mut LineReader> for TokenReader {
    type Error = std::io::Error;

    fn try_from(lines: &mut LineReader) -> Result<Self, Error> {
        let chars: Result<VecDeque<(usize, usize, char)>, Error> = lines
            .enumerate()
            .flat_map(|(r, l)| {
                l.map(move |l| {
                    l.char_indices()
                        .collect::<Vec<_>>()
                        .into_iter()
                        .filter(|(_, ch)| *ch != '.')
                        .map(move |(c, ch)| Ok::<_, Error>((r, c, ch)))
                })
            })
            .flatten()
            .collect();
        let char_stream = chars?;

        Ok(Self { char_stream })
    }
}

impl Iterator for TokenReader {
    type Item = SchematicItem;

    fn next(&mut self) -> Option<Self::Item> {
        let Some((y, x, tok)) = self.char_stream.pop_front() else {
            return None;
        };
        let mut len = 1;

        if let Some(mut val) = tok.to_digit(10) {
            loop {
                let Some((y2, x2, ch)) = self.char_stream.front() else {
                    break;
                };

                let Some(digval) = ch.to_digit(10) else {
                    break;
                };

                if *y2 != y || *x2 != x + len {
                    break;
                }

                val = val * 10 + digval;
                len += 1;
                self.char_stream.pop_front();
            }

            Some(SchematicItem {
                y,
                x,
                len,
                typ: SchematicItemType::PartNumber(val),
            })
        } else {
            Some(SchematicItem {
                y,
                x,
                len,
                typ: SchematicItemType::Symbol(tok),
            })
        }
    }
}

fn part1(lines: &mut LineReader) -> Result<u32, Error> {
    let toks = TokenReader::try_from(lines)?;
    let mut schematic = LinkedList::<SchematicItem>::new();
    let mut trim_row = 1;
    let mut val = 0;
    for si in toks {
        // trim items we can't match with anymore
        if si.y > trim_row {
            trim_row = si.y;
            schematic = schematic
                .into_iter()
                .filter(|i| i.y + 1 >= trim_row)
                .collect();
        }

        val += match si.typ {
            SchematicItemType::PartNumber(num) => {
                if schematic.iter().any(|i| i.is_adj(&si) && i.is_match(&si)) {
                    num
                } else {
                    schematic.push_back(si);
                    0
                }
            }
            SchematicItemType::Symbol(_ch) => {
                let (matches, rest) = schematic
                    .iter()
                    .partition(|i| i.is_adj(&si) && i.is_match(&si));
                schematic = rest;
                schematic.push_back(si);
                matches.iter().fold(0, |acc, i| {
                    if let SchematicItemType::PartNumber(num) = i.typ {
                        acc + num
                    } else {
                        acc
                    }
                })
            }
        }
    }

    Ok(val)
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let filename = Path::new(&args[1]);

    let file = match File::open(&filename) {
        Err(why) => panic!("couldn't open input file {:?}: {}", filename, why),
        Ok(file) => file,
    };

    println!(
        "part1 = {}",
        part1(&mut BufReader::new(file).lines()).unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&mut TEST_DATA.lines().map(|x| Ok(String::from(x)))).unwrap(),
            4361
        );
    }
}
