use std::fs::read_to_string;
use std::collections::HashSet;

#[derive(Debug)]
struct PartNumber {
    value: i64,
    points: HashSet<(i64, i64)>,
} impl PartNumber {
    fn new(row: i64, col: i64, ch: char) -> Self {
        let points = HashSet::from([
            (row - 1, col - 1),
            (row, col - 1),
            (row + 1, col - 1), // left hand side
            (row - 1, col),
            (row + 1, col), // above and below
            (row - 1, col + 1),
            (row, col + 1),
            (row + 1, col + 1), // right hand side
        ]);
        Self {
            value: (ch as u8 - b'0') as i64,
            points,
        }
    }

    fn add_digit(&mut self, row: i64, col: i64, ch: char) {
        self.value = self.value * 10 + (ch as u8 - b'0') as i64;
        self.points
            .extend([(row - 1, col + 1), (row, col + 1), (row + 1, col + 1)]);
    }

    fn extract_value(&self) -> i64 {
        self.value
    }

    fn next_to_symbol(&self, syms: &HashSet<(i64, i64)>) -> bool {
        self.points.intersection(&syms).next().is_some()
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
    let mut symbols: HashSet<(i64, i64)> = HashSet::new();
    let mut gears: HashSet<(i64, i64)> = HashSet::new();

    let mut last: Option<PartNumber> = None;
    for (row, line) in lines.iter().enumerate() {
        let mut extended = String::from(line);
        extended.push_str("...");
        for (col, ch) in extended.chars().enumerate() {
            if ch.is_ascii_digit() {
                if let Some(ref mut num) = last {
                    num.add_digit(row as i64, col as i64, ch);
                } else {
                    last = Some(PartNumber::new(row as i64, col as i64, ch));
                }
            } else {
                if let Some(num) = last.take() {
                    nums.push(num);
                }
                if ch != '.' && ch != '\r' {
                    symbols.insert((row as i64, col as i64));
                    if ch == '*' {
                        gears.insert((row as i64, col as i64));
                    }
                }
            }
        }
    }

    //let _total_one = nums
    //    .iter()
    //    .filter(|num| num.next_to_symbol(&symbols))
    //    .map(PartNumber::extract_value) 
    //    .sum::<i64>();

    let mut total_two = 0;

    'next_gear: for gear in gears {
        let mut matches = Vec::new();
        for num in &nums {
            if num.points.contains(&gear) {
                if matches.len() == 2 {
                    continue 'next_gear;
                }
                matches.push(num.value);
            }
        }
        if matches.len() == 2 {
            total_two += matches[0] * matches[1];
        }
    }

    println!("{}", total_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanity() {
        assert_eq!(1,1);
    }
}