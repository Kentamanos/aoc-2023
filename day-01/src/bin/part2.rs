use regex::Regex;

fn main() {
    let input = include_str!("../../input2.txt");
    println!("part2: {}", part2(input));
}

fn line_to_digits(line: &str) -> Vec<u32> {
    let mut line_copy = line.clone();
    let mut digits = Vec::new();
    let re = Regex::new(r"[0-9]|zero|one|two|three|four|five|six|seven|eight|nine").unwrap();

    loop {
        let foo = re.find(line_copy);
        match foo {
            Some(m) => {
                let num_match = &line_copy[m.start()..m.end()];
                match num_match {
                    "0" => digits.push(0),
                    "1" => digits.push(1),
                    "2" => digits.push(2),
                    "3" => digits.push(3),
                    "4" => digits.push(4),
                    "5" => digits.push(5),
                    "6" => digits.push(6),
                    "7" => digits.push(7),
                    "8" => digits.push(8),
                    "9" => digits.push(9),
                    "zero" => digits.push(0),
                    "one" => digits.push(1),
                    "two" => digits.push(2),
                    "three" => digits.push(3),
                    "four" => digits.push(4),
                    "five" => digits.push(5),
                    "six" => digits.push(6),
                    "seven" => digits.push(7),
                    "eight" => digits.push(8),
                    "nine" => digits.push(9),
                    _ => {
                        panic!("unexpected match: {}", num_match)
                    }
                }
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
