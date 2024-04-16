use std::fs;
use std::io;
use std::io::BufRead;

pub fn read_file(
    filename: &str,
) -> Result<impl Iterator<Item = Result<String, io::Error>>, io::Error> {
    let file = fs::File::open(&filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_testdata(
    data: &str,
) -> Result<impl Iterator<Item = Result<String, io::Error>> + '_, io::Error> {
    Ok(data.lines().map(|l| Ok(String::from(l))))
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
    fn it_works() {
        let file = read_file("src/test.data").unwrap();
        for (left, right) in read_testdata(TEST_DATA).unwrap().zip(file) {
            assert_eq!(left.unwrap(), right.unwrap());
        }
    }
}
