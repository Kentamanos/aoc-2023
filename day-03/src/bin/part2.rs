use regex::Regex;

struct Schematic {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

struct Number {
    value: u32,
    row: usize,
    col_start: usize,
    col_end: usize,
}

struct Symbol {
    value: char,
    row: usize,
    col: usize,
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("part1: {}", part2(input));
}

fn parse_schematic(input: &str) -> Schematic {
    let numbers_re = Regex::new(r"\d+").unwrap();
    let symbols_re = Regex::new(r"[^\d\.]").unwrap();

    let mut numbers = Vec::new();
    let mut symbols = Vec::new();

    for (row, line) in input.lines().enumerate() {
        let numbers_it = numbers_re.find_iter(line);
        for m in numbers_it {
            let number = Number {
                value: m.as_str().parse().unwrap(),
                row,
                col_start: m.start(),
                col_end: m.end() - 1,
            };
            numbers.push(number);
        }

        let symbols_it = symbols_re.find_iter(line);
        for m in symbols_it {
            let symbol = Symbol {
                value: m.as_str().chars().next().unwrap(),
                row,
                col: m.start(),
            };
            symbols.push(symbol);
        }
    }

    Schematic { numbers, symbols }
}

fn part2(input: &str) -> u32 {
    let schematic = parse_schematic(input);
    let mut total = 0;
    for symbol in &schematic.symbols {
        let mut adjacent_numbers = Vec::new();
        if symbol.value == '*' {
            for number in &schematic.numbers {
                if symbol.row.abs_diff(number.row) <= 1
                    && (symbol.col.abs_diff(number.col_start) <= 1
                        || symbol.col.abs_diff(number.col_end) <= 1)
                {
                    adjacent_numbers.push(number.value);
                }
            }
            if adjacent_numbers.len() == 2 {
                println!("{} {}", adjacent_numbers[0], adjacent_numbers[1]);
                total += adjacent_numbers[0] * adjacent_numbers[1];
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_schematic() {
        let test = include_str!("../../test1.txt");
        let schematic = parse_schematic(test);
        assert_eq!(schematic.numbers.len(), 10);
        assert_eq!(schematic.symbols.len(), 6);
    }

    #[test]
    fn test_part1() {
        let test = include_str!("../../test1.txt");
        assert_eq!(part2(test), 467835);
    }
}
