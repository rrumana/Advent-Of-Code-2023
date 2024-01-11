use std::fs::read_to_string;
use std::collections::HashMap;

#[derive(Debug)]
struct Seed {
    value: i64
} impl Seed {
    fn new(value: i64) -> Self {
        Self { value: value }
    }

    fn change_value(&mut self, source_size: &HashMap<i64, i64>, source_dest: &HashMap<i64, i64>) {
        for (key, val) in source_size.iter() {
            //println!("Seed: {}, Key: {}, Value: {}", self.value, key, val);
            if self.value < *key { 
                //println!("Skipped! Value smaller than key.");
                continue; }
            if self.value >= (key + val) { 
                //println!("Skipped! Value larger than key + range size.");
                continue; }
            self.value = self.value + (*source_dest.get(&key).unwrap()-key);
            //println!("Edited! now equals {}.", self.value);
            break;
        }
        //if map.contains_key(&self.value) { self.value = *map.get(&self.value).unwrap(); }
    }
}

fn edit_map(source_size: &mut HashMap<i64, i64>, source_dest: &mut HashMap<i64, i64>, destination: i64, source: i64, length: i64) {
    source_size.insert(source, length);
    source_dest.insert(source, destination);
}

fn part_one(seeds: &Vec<Seed>) -> i64 {
    let mut min: i64 = i64::MAX;
    for seed in seeds {
        if seed.value > min { continue };
        min = seed.value;
    }
    return min;
}

fn _part_two() -> i64 {
    return 0;
}

fn main() {
    let filepath = "real_input.txt";
    let lines: Vec<String> = read_to_string(filepath)
        .expect("Something went wrong reading the file")
        .split("\r\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    let (_, data) = lines[0].split_once(": ").unwrap();

    let mut seeds = data
        .split_whitespace()
        .map(|snum| Seed::new(snum.parse::<i64>().unwrap()))
        .collect::<Vec<Seed>>();

    //println!("Seeds: {:?}", seeds);

    let mut source_size = HashMap::new();
    let mut source_dest = HashMap::new();

    for line in lines.iter().skip(1) {
        //println!("line: {:?}", line);

        if !line.chars().next().unwrap().is_numeric() { 
            for seed in seeds.iter_mut() {
                seed.change_value(&source_size, &source_dest);
            }

            println!("Data: {:?}", seeds);

            source_size = HashMap::new();
            source_dest = HashMap::new();
        } else {
            let vals = line
                .split_whitespace()
                .map(|snum| snum.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            edit_map(&mut source_size, &mut source_dest, vals[0], vals[1], vals[2]);
        }
    }

    let total_one: i64 = part_one(&seeds);
    //let total_two: i64 = part_two();

    println!("Part one answer: {}", total_one);
    //println!("Part two answer: {}", total_two);
}