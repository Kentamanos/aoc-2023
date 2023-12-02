use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;

static LOOK_UP_TABLE: Lazy<HashMap<&str, u32>> = Lazy::new(|| {
    let values = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];

    let m: HashMap<_, _> = values
        .iter()
        .enumerate()
        .map(|(i, v)| (*v, i as u32 % 9 + 1))
        .collect();

    m
});

fn main() {
    let input = include_str!("../../input.txt");
    println!("part2: {}", part2(input));
}

fn line_to_digits(line: &str) -> Vec<u32> {
    let mut line_copy = line.clone();
    let mut digits = Vec::new();
    // TODO: Create a RegexSet from keys in LOOK_UP_TABLE?
    let re = Regex::new(r"[0-9]|zero|one|two|three|four|five|six|seven|eight|nine").unwrap();

    loop {
        let foo = re.find(line_copy);
        match foo {
            Some(m) => {
                let num_match = &line_copy[m.start()..m.end()];
                digits.push(*LOOK_UP_TABLE.get(num_match).unwrap());
                line_copy = &line_copy[m.start() + 1..];
            }
            None => break,
        }
    }
    digits
}

fn part2(input: &str) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let digits = line_to_digits(line);
        let value = digits.first().unwrap() * 10 + digits.last().unwrap();
        total += value;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let test = include_str!("../../test2.txt");
        assert_eq!(part2(test), 281);
    }

    #[test]
    fn test_line_to_digits() {
        assert_eq!(line_to_digits("onetwo3four"), vec![1, 2, 3, 4]);
        assert_eq!(line_to_digits("two4four"), vec![2, 4, 4]);
        assert_eq!(line_to_digits("7eightseveneightthree"), vec![7, 8, 7, 8, 3]);
        assert_eq!(line_to_digits("eightwothree"), vec![8, 2, 3]);
    }
}
