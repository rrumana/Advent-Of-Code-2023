use std::fs;

fn main() {
    let filepath = "test_input.txt";
    let contents = fs::read_to_string(filepath).expect("Something went wrong reading the file");
    let lines = contents.split("\n");

    for line in lines {
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanity() {
        assert_eq!(1,1);
    }
}