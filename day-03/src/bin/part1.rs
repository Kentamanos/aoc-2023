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
    println!("part1: {}", part1(input));
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

fn part1(input: &str) -> u32 {
    let schematic = parse_schematic(input);
    let mut total = 0;
    for number in schematic.numbers {
        for symbol in schematic.symbols.iter() {
            if number.row.abs_diff(symbol.row) <= 1
                && ((number.col_start.abs_diff(symbol.col) <= 1)
                    || (number.col_end.abs_diff(symbol.col) <= 1))
            {
                total += number.value;
                break;
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
        assert_eq!(part1(test), 4361);
    }
}
