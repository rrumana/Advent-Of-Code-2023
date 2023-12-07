use std::fs::read_to_string;
use std::collections::HashSet;

struct PartNumber {
    value: i32,
    points: HashSet<(i32, i32)>,
}

impl PartNumber {
    fn new(row: i32, col: i32, ch: char) -> Self {
        let points = HashSet::from([
            (row - 1, col - 1), (row, col - 1), (row + 1, col - 1), // left hand side
            (row - 1, col), (row + 1, col),                         // above and below
            (row - 1, col + 1), (row, col + 1), (row + 1, col + 1), // right hand side
        ]);
        Self {
            value: ch.to_digit(10).unwrap() as i32,
            points,
        }
    }

    fn extend(&mut self, row: i32, col: i32, ch: char) {
        self.value = self.value * 10 + ch.to_digit(10).unwrap() as i32;
        self.points.extend([(row - 1, col + 1), (row, col + 1), (row + 1, col + 1)]);
    }

    fn get_value(&self) -> i32 {
        self.value
    }

    fn by_symbol(&self, symbols: &HashSet<(i32, i32)>) -> bool {
        self.points.intersection(symbols).next().is_some()
    }
}

fn main() {
    let filepath = "real_input.txt";
    let lines: Vec<String> = read_to_string(filepath)
        .expect("Something went wrong reading the file")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    let mut nums: Vec<PartNumber> = Vec::new();
    let mut symbols: HashSet<(i32, i32)> = HashSet::new();
    let mut gears: HashSet<(i32, i32)> = HashSet::new();

    let mut last: Option<PartNumber> = None;
    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                if let Some(ref mut num) = last {
                    num.extend(row as i32, col as i32, ch);
                } else {
                    last = Some(PartNumber::new(row as i32, col as i32, ch));
                }
            } else {
                if let Some(num) = last.take() {
                    nums.push(num);
                }
                if ch != '.' {
                    symbols.insert((row as i32, col as i32));
                    if ch == '*' {
                        gears.insert((row as i32, col as i32));
                    }
                }
            }
        }
    }

    let total = nums
        .iter()
        .filter(|num| num.by_symbol(&symbols))
        .map(PartNumber::get_value)
        .sum::<i32>();

    println!("{}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanity() {
        assert_eq!(1,1);
    }
}