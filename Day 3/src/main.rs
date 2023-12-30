use std::fs::read_to_string;
use std::collections::HashSet;

#[derive(Debug)]
struct PartNumber {
    value: i64,
    points: HashSet<(i64, i64)>,
} impl PartNumber {
    fn new(row: i64, col: i64, ch: char) -> Self {
        let points = HashSet::from([
            (row - 1, col - 1), (row, col - 1), (row + 1, col - 1), // left hand side
            (row - 1, col), (row + 1, col),                         // above and below
            (row - 1, col + 1), (row, col + 1), (row + 1, col + 1), // right hand side
        ]);
        Self {
            value: ch.to_digit(10).unwrap() as i64,
            points,
        }
    }

    fn extend(&mut self, row: i64, col: i64, ch: char) {
        self.value = self.value * 10 + ch.to_digit(10).unwrap() as i64;
        self.points.extend([(row - 1, col + 1), (row, col + 1), (row + 1, col + 1)]);
    }

    fn get_value(&self) -> i64 {
        self.value
    }

    fn by_symbol(&self, syms: &HashSet<(i64, i64)>) -> bool {
        self.points.intersection(&syms).next().is_some()
    }
}

fn part_one(nums: &Vec<PartNumber>, syms: &HashSet<(i64, i64)>) -> i64 {
    let total = nums
        .iter()
        .filter(|num| num.by_symbol(&syms))
        .map(PartNumber::get_value) 
        .sum::<i64>();

    total
}

fn part_two(nums: &Vec<PartNumber>, gears: &HashSet<(i64, i64)>) -> i64 {
    let mut total = 0;

    'next_gear: for gear in gears {
        let mut matches = Vec::new();
        for num in nums {
            if num.points.contains(&gear) {
                if matches.len() == 2 {
                    continue 'next_gear;
                }
                matches.push(num.value);
            }
        }
        if matches.len() == 2 {
            total += matches[0] * matches[1];
        }
    }

    total
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
    let mut syms: HashSet<(i64, i64)> = HashSet::new();
    let mut gears: HashSet<(i64, i64)> = HashSet::new();

    let mut last: Option<PartNumber> = None;
    for (row, line) in lines.iter().enumerate() {
        let mut extended = String::from(line);
        extended.push_str("...");
        for (col, ch) in extended.chars().enumerate() {
            if ch.is_ascii_digit() {
                if let Some(ref mut num) = last {
                    num.extend(row as i64, col as i64, ch);
                } else {
                    last = Some(PartNumber::new(row as i64, col as i64, ch));
                }
            } else {
                if let Some(num) = last.take() {
                    nums.push(num);
                }
                if ch != '.' && ch != '\r' {
                    syms.insert((row as i64, col as i64));
                    if ch == '*' {
                        gears.insert((row as i64, col as i64));
                    }
                }
            }
        }
    }

    let total_one: i64 = part_one(&nums, &syms);
    let total_two: i64 = part_two(&nums, &gears);

    println!("Part one answer: {}", total_one);
    println!("Part two answer: {}", total_two);
}