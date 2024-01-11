use std::fs::read_to_string;

fn part_one() -> i64 {
    return 0;
}

fn part_two() -> i64 {
    return 0;
}

fn main() {
    let filepath = "test_input.txt";
    let _lines: Vec<String> = read_to_string(filepath)
        .expect("Something went wrong reading the file")
        .split("\r\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    let total_one: i64 = part_one();
    let total_two: i64 = part_two();

    println!("Part one answer: {}", total_one);
    println!("Part two answer: {}", total_two);
}