use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, one_of},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Symbol {
    Hash,
    Period,
    Question,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Row {
    symbols: Vec<Symbol>,
    values: Vec<usize>,
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("part2: {}", part2(input));
}

fn symbols(input: &str) -> IResult<&str, Vec<Symbol>> {
    let (input, symbol_chars) = many1(one_of("#.?"))(input)?;

    let symbols = symbol_chars
        .iter()
        .map(|c| match c {
            '#' => Symbol::Hash,
            '.' => Symbol::Period,
            '?' => Symbol::Question,
            _ => panic!("unexpected symbol"),
        })
        .collect();

    Ok((input, symbols))
}

fn symbols_to_string(symbols: &Vec<Symbol>) -> String {
    symbols
        .iter()
        .map(|s| match s {
            Symbol::Hash => "#",
            Symbol::Period => ".",
            Symbol::Question => "?",
        })
        .collect::<Vec<_>>()
        .join("")
}

fn indent(indent: u32) {
    for _ in 0..indent {
        print!("| ");
    }
}

fn score_combos(indent_level: u32, row: &Row, memo: &mut HashMap<Row, u64>) -> u64 {
    // indent(indent_level);
    // println!("Symbols: {} - Values: {:?}", foo, row.values);

    if let Some(score) = memo.get(row) {
        return *score;
    }

    let symbols_empty = row.symbols.is_empty();
    let values_empty = row.values.is_empty();

    match (symbols_empty, values_empty) {
        (true, true) => {
            // indent(indent_level);
            // println!("Symbols and values empty. Found a match!");
            return 1;
        }
        (true, false) => {
            // indent(indent_level);
            // println!("Symbols empty, but values not. No match!");
            return 0;
        }
        (_, _) => {
            // Keep going...
        }
    };

    match row.symbols[0] {
        Symbol::Period => {
            let new_symbols = &row.symbols[1..];
            // let new_values = &row.values[1..];
            let new_row = Row {
                symbols: new_symbols.to_vec(),
                values: row.values.clone(),
            };
            let score = score_combos(indent_level + 1, &new_row, memo);
            memo.insert(new_row, score);
            score
        }
        Symbol::Question => {
            let mut hash_version: Vec<_> = Vec::new();
            hash_version.push(Symbol::Hash);
            hash_version.extend_from_slice(&row.symbols[1..]);

            let new_row = Row {
                symbols: hash_version.to_vec(),
                values: row.values.clone(),
            };
            let hash_val = score_combos(indent_level + 1, &new_row, memo);
            memo.insert(new_row, hash_val);

            let mut period_version: Vec<_> = Vec::new();
            period_version.push(Symbol::Period);
            period_version.extend_from_slice(&row.symbols[1..]);
            let new_row = Row {
                symbols: period_version.to_vec(),
                values: row.values.clone(),
            };

            let period_val = score_combos(indent_level + 1, &new_row, memo);
            memo.insert(new_row, period_val);

            hash_val + period_val
        }
        Symbol::Hash => {
            // If we have hashes but are out of values, we're done...
            if row.values.is_empty() {
                // indent(indent_level);
                // println!("I have hashes left, but values is empty. No match!");
                // memo.insert(*row, 0);
                return 0;
            }

            let number_of_hashes = row
                .symbols
                .iter()
                .take_while(|s| **s == Symbol::Hash)
                .count();

            let number_of_possible_hashes = row
                .symbols
                .iter()
                .take_while(|s| **s != Symbol::Period)
                .count();

            if number_of_hashes == row.values[0] {
                if number_of_possible_hashes > number_of_hashes {
                    // We have to treat the next thing after our hashes as a period
                    // indent(indent_level);
                    // println!("Exact number of hashes, but ? is next, converting to period.");
                    let mut new_symbols = Vec::new();
                    new_symbols.push(Symbol::Period);
                    new_symbols.extend_from_slice(&row.symbols[number_of_hashes + 1..]);
                    let new_values = &row.values[1..];

                    let new_row = Row {
                        symbols: new_symbols.to_vec(),
                        values: new_values.to_vec(),
                    };

                    let score = score_combos(indent_level + 1, &new_row, memo);
                    memo.insert(new_row, score);
                    return score;
                } else {
                    // indent(indent_level);
                    // println!("Exact number of hashes...");
                    let new_symbols = &row.symbols[number_of_hashes..];
                    let new_values = &row.values[1..];
                    let new_row = Row {
                        symbols: new_symbols.to_vec(),
                        values: new_values.to_vec(),
                    };
                    let score = score_combos(indent_level + 1, &new_row, memo);
                    memo.insert(new_row, score);
                    return score;
                }
            } else if number_of_hashes > row.values[0] {
                // indent(indent_level);
                // println!("Too many hashes. No match!");
                // memo.insert(*row, 0);
                return 0;
            } else if number_of_possible_hashes < row.values[0] {
                // indent(indent_level);
                // println!("Not enough hashes. No match!");
                // memo.insert(*row, 0);
                return 0;
            } else {
                // indent(indent_level);
                // println!("Not exact number of hashes...");
                // Consume what we have and reduce value by that amount, but force the next one to be a #
                let mut new_symbols = Vec::new();
                new_symbols.push(Symbol::Hash);
                new_symbols.extend_from_slice(&row.symbols[number_of_hashes + 1..]);
                let mut new_values = Vec::new();
                new_values.push(row.values[0] - number_of_hashes);
                new_values.extend_from_slice(&row.values[1..]);
                let new_row = Row {
                    symbols: new_symbols.to_vec(),
                    values: new_values.to_vec(),
                };
                let score = score_combos(indent_level + 1, &new_row, memo);
                memo.insert(new_row, score);
                return score;
            }
        }
    }
}

fn row(input: &str) -> IResult<&str, Row> {
    let (input, (symbols, numbers)) =
        separated_pair(symbols, tag(" "), separated_list1(tag(","), complete::u64))(input).unwrap();

    let numbers_usize = numbers.iter().map(|n| *n as usize).collect::<Vec<_>>();

    Ok((
        input,
        Row {
            symbols,
            values: numbers_usize,
        },
    ))
}

fn part2(input: &str) -> u64 {
    let (input, rows) = separated_list1(newline, row)(input).unwrap();
    assert_eq!(input, "");

    println!("Parsed...");

    // Duplicate rows with 5 of each side...
    // TODO: Feels like there is a more elegant way...
    let mut new_rows = Vec::new();
    for row in rows {
        let mut new_symbols: Vec<Symbol> = Vec::new();
        let mut new_values = Vec::new();

        for i in 0..5 {
            row.symbols.iter().for_each(|sym| {
                new_symbols.push(sym.clone());
            });
            if i < 4 {
                new_symbols.push(Symbol::Question);
            }
        }

        for _ in 0..5 {
            new_values.extend(row.values.iter());
        }

        let new_row = Row {
            symbols: new_symbols,
            values: new_values,
        };
        new_rows.push(new_row);
    }

    let rows = new_rows;

    println!("Duplicated...");

    let mut memo = HashMap::<Row, u64>::new();

    // let total: u64 = rows.par_iter().map(|row| score_combos(0, &row)).sum();
    let total: u64 = rows
        .iter()
        .map(|row| {
            // println!("Row: {:?}", row);
            score_combos(0, &row, &mut memo)
        })
        .sum();

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let test = include_str!("../../test.txt");
        assert_eq!(part2(test), 21);
    }

    #[test]
    fn individual_test() {
        // let input = "???.### 1,1,3";
        // assert_eq!(part1(input), 1);

        // let input = ".??..??...?##. 1,1,3";
        // assert_eq!(part1(input), 4);

        // let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        // assert_eq!(part1(input), 1);

        // let input = "????.#...#... 4,1,1";
        // assert_eq!(part1(input), 1);

        // let input = "????.######..#####. 1,6,5";
        // assert_eq!(part1(input), 4);

        // let input = "?###???????? 3,2,1";
        // assert_eq!(part1(input), 10);

        let input = "??????????????? 1,1,1,2,1";
        let score = part2(input);
    }

    #[test]
    fn mini_test_2() {
        let input = ".??#.?#??#????#?? 2,4,1,1,1";
        // Not sure the value, but blowing up...
        part2(input);
    }
}
