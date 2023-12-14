use grid::*;
use itertools::Itertools;
use nom::{
    character::complete::{newline, one_of},
    multi::{many1, many_m_n, separated_list1},
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Symbol {
    Round,
    Square,
    Gap,
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("part1: {}", part1(input));
}

fn symbols(input: &str) -> IResult<&str, Vec<Symbol>> {
    let (input, symbol_chars) = many1(one_of("#O."))(input)?;
    let symbols: Vec<_> = symbol_chars
        .iter()
        .map(|c| match c {
            '#' => Symbol::Square,
            '.' => Symbol::Gap,
            'O' => Symbol::Round,
            _ => panic!("unexpected symbol"),
        })
        .collect();

    Ok((input, symbols))
}

fn calculate_load(grid: &Grid<Symbol>) -> u64 {
    grid.iter_cols()
        .map(|col_iter| {
            col_iter
                .rev()
                .enumerate()
                .map(|(i, symbol)| {
                    // println!("i: {}, symbol: {:?}", i, symbol);
                    match symbol {
                        Symbol::Round => i as u64 + 1,
                        _ => 0,
                    }
                })
                .sum::<u64>()
        })
        .sum()
}

fn tilt_north(grid: Grid<Symbol>) -> Grid<Symbol> {
    let mut new_grid: Grid<Symbol> = grid![];

    grid.iter_cols().for_each(|col_iter| {
        // Create an "empty" new column
        let mut new_col: Vec<Symbol> = Vec::new();
        for i in 0..grid.rows() {
            new_col.push(Symbol::Gap);
        }

        let mut last_available_spot: Option<usize> = None;
        col_iter.enumerate().for_each(|(row, symbol)| match symbol {
            Symbol::Round => {
                if last_available_spot.is_some() {
                    new_col[last_available_spot.unwrap()] = Symbol::Round;
                    last_available_spot = Some(last_available_spot.unwrap() + 1);
                } else {
                    new_col[row] = Symbol::Round;
                }
            }
            Symbol::Square => {
                last_available_spot = None;
                new_col[row] = Symbol::Square;
            }
            Symbol::Gap => {
                if last_available_spot.is_none() {
                    last_available_spot = Some(row);
                }
            }
        });

        new_grid.push_col(new_col);
    });

    new_grid
}

fn part1(input: &str) -> u64 {
    let (input, lines) = separated_list1(newline, symbols)(input).unwrap();
    assert_eq!(input, "");

    let mut grid: Grid<Symbol> = grid![];
    for line in lines {
        grid.push_row(line);
    }

    let grid = tilt_north(grid);

    calculate_load(&grid)
}

fn print_grid(grid: &Grid<Symbol>) {
    grid.iter_rows().for_each(|row_iter| {
        row_iter.for_each(|symbol| match symbol {
            Symbol::Round => print!("O"),
            Symbol::Square => print!("#"),
            Symbol::Gap => print!("."),
        });
        println!();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_calc() {
        let test = include_str!("../../test.txt");

        let (test, lines) = separated_list1(newline, symbols)(test).unwrap();
        assert_eq!(test, "");

        let mut grid: Grid<Symbol> = grid![];
        for line in lines {
            grid.push_row(line);
        }

        let load = calculate_load(&grid);

        assert_eq!(load, 136);
    }

    #[test]
    fn test_tilt() {
        let test = include_str!("../../test.txt");

        let (test, lines) = separated_list1(newline, symbols)(test).unwrap();
        assert_eq!(test, "");

        let mut grid: Grid<Symbol> = grid![];
        for line in lines {
            grid.push_row(line);
        }

        let new_grid = tilt_north(grid);

        print_grid(&new_grid);

        println!("load: {}", calculate_load(&new_grid));
    }
}
