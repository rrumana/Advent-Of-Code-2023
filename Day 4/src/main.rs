use std::fs::read_to_string;
use std::collections::HashSet;
use std::convert::TryInto;

#[derive(Debug)]
struct Card {
    card: i64,
    winning: HashSet<i64>,
    numbers: HashSet<i64>
} impl Card {
    fn new(card: i64, winning: HashSet<i64>, numbers: HashSet<i64>,) -> Self {
        Self {
            card: card,
            winning: winning,
            numbers: numbers
        }
    }

    fn get_score(&self) -> i64 {
        let count = self
            .winning
            .intersection(&self.numbers)
            .count();
        if count > 0 {
            1 << (count -1)
        } else {
            0
        }
    }
}

fn part_one(cards: &Vec<Card>) -> i64 {
    cards.iter().map(Card::get_score).sum::<i64>()
}

fn part_two() -> i64 {
    return 0;
}

fn main() {
    let filepath = "real_input.txt";
    let lines: Vec<String> = read_to_string(filepath)
        .expect("Something went wrong reading the file")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    let mut cards: Vec<Card> = Vec::new();

    for line in lines.iter() {
        let (_, nums) = line.split_once(": ").unwrap();
        let (winning, chosen) = nums.split_once(" | ").unwrap();

        let winning_nums = winning
            .split_whitespace()
            .map(|snum| snum.parse::<i64>().unwrap())
            .collect::<HashSet<_>>();

        let chosen_nums = chosen
            .split_whitespace()
            .map(|snum| snum.parse::<i64>().unwrap())
            .collect::<HashSet<_>>();

        cards.push(Card::new((cards.len()+1).try_into().unwrap(), winning_nums, chosen_nums));
    }

    let total_one: i64 = part_one(&cards);
    let total_two: i64 = part_two();

    println!("Part one answer: {}", total_one);
    println!("Part two answer: {}", total_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanity() {
        assert_eq!(1,1);
    }
}