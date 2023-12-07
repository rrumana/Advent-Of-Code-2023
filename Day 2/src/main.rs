use std::fs;

fn power_of_game(game: &str) -> i32 {
    // remove game syntax
    let section = match game.split_once(':') {
        Some((_before, after)) => { after }
        None => { return 0 }
    };
    // split up each hand
    let hands = section.split(';');
    // define the minimum number of each color
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;
    for hand in hands {
        // split each hand by color
        let colors = hand.split(',');
        for color in colors {
            // split color and number then determine if valid
            let trimmed = color.trim();
            match trimmed.split_once(' ') {
                Some((str_number, color)) => { 
                    let number: i32 = str_number.parse().unwrap();
                    match color {
                        "red" => {
                            if number <= min_red { continue; }
                            else { min_red = number; }
                        }
                        "green" => {
                            if number <= min_green { continue; }
                            else { min_green = number; }
                        }
                        "blue" => {
                            if number <= min_blue { continue; }
                            else { min_blue = number; }
                        }
                        &_ => { continue; }
                    }
                }
                None => { continue; }
            }
        }
    }
    let power = min_red * min_green * min_blue;
    power
}

fn main() {
    let filepath = "real_input.txt";
    let contents = fs::read_to_string(filepath).expect("Something went wrong reading the file");
    let lines = contents.split("\n");

    let mut sum: i32 = 0;
    for line in lines {
        sum += power_of_game(line);
    }

    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power() {
        assert_eq!(power_of_game(&"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), 48);
        assert_eq!(power_of_game(&"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"), 12);
        assert_eq!(power_of_game(&"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"), 1560);
        assert_eq!(power_of_game(&"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"), 630);
        assert_eq!(power_of_game(&"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), 36);
    }
}