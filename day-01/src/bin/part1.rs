fn main() {
    let input = include_str!("../../input1.txt");
    println!("part1: {}", part1(input));
}

fn part1(input: &str) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let mut digits = Vec::new();
        for c in line.chars() {
            match c {
                '0'..='9' => {
                    digits.push(c.to_digit(10).unwrap());
                }
                _ => {}
            }
        }
        let value = digits.first().unwrap() * 10 + digits.last().unwrap();
        total += value;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test = include_str!("../../test1.txt");
        assert_eq!(part1(test), 142);
    }
}
