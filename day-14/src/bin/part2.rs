use std::collections::HashMap;

use grid::*;
use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
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
    println!("part1: {}", part2(input));
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
        for _ in 0..grid.rows() {
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

fn tilt_south(grid: Grid<Symbol>) -> Grid<Symbol> {
    // Lazy way out...
    let mut new_grid: Grid<Symbol> = grid.clone();
    new_grid.flip_rows();

    let mut new_grid = tilt_north(new_grid);
    new_grid.flip_rows();

    new_grid
}

fn tilt_west(grid: Grid<Symbol>) -> Grid<Symbol> {
    let mut new_grid: Grid<Symbol> = grid![];

    grid.iter_rows().for_each(|row_iter| {
        // Create an "empty" new row
        let mut new_row: Vec<Symbol> = Vec::new();
        for _ in 0..grid.cols() {
            new_row.push(Symbol::Gap);
        }

        let mut last_available_spot: Option<usize> = None;
        row_iter.enumerate().for_each(|(col, symbol)| match symbol {
            Symbol::Round => {
                if last_available_spot.is_some() {
                    new_row[last_available_spot.unwrap()] = Symbol::Round;
                    last_available_spot = Some(last_available_spot.unwrap() + 1);
                } else {
                    new_row[col] = Symbol::Round;
                }
            }
            Symbol::Square => {
                last_available_spot = None;
                new_row[col] = Symbol::Square;
            }
            Symbol::Gap => {
                if last_available_spot.is_none() {
                    last_available_spot = Some(col);
                }
            }
        });
        // println!("{:?}", new_row);

        new_grid.push_row(new_row);
    });

    new_grid
}

fn tilt_east(grid: Grid<Symbol>) -> Grid<Symbol> {
    // Lazy way out...
    let mut new_grid: Grid<Symbol> = grid.clone();
    new_grid.flip_cols();

    let mut new_grid = tilt_west(new_grid);
    new_grid.flip_cols();

    new_grid
}

fn positions_of_rounds(grid: &Grid<Symbol>) -> Vec<(usize, usize)> {
    let mut positions: Vec<(usize, usize)> = Vec::new();

    grid.iter_rows().enumerate().for_each(|(row, row_iter)| {
        row_iter.enumerate().for_each(|(col, symbol)| {
            if symbol == &Symbol::Round {
                positions.push((row, col));
            }
        });
    });

    positions
}

fn cycle(grid: Grid<Symbol>) -> Grid<Symbol> {
    let mut grid = tilt_north(grid);
    grid = tilt_west(grid);
    grid = tilt_south(grid);
    grid = tilt_east(grid);

    grid
}

fn detect_loop(grid: Grid<Symbol>) -> (Grid<Symbol>, u64, u64) {
    let mut grid = grid.clone();
    let mut hs: HashMap<(Vec<(usize, usize)>, Vec<(usize, usize)>), u64> = HashMap::new();
    let mut count = 0;

    loop {
        let start_pos = positions_of_rounds(&grid);
        grid = cycle(grid);
        let end_pos = positions_of_rounds(&grid);

        count += 1;

        let existing_count = hs.insert((start_pos.clone(), end_pos.clone()), count);

        if let Some(old_count) = existing_count {
            return (grid, old_count, count);
        }
    }
}

fn part2(input: &str) -> u64 {
    let (input, lines) = separated_list1(newline, symbols)(input).unwrap();
    assert_eq!(input, "");

    let mut grid: Grid<Symbol> = grid![];
    for line in lines {
        grid.push_row(line);
    }

    let target_cycles = 1_000_000_000u64;

    let (mut grid, old_count, new_count) = detect_loop(grid);

    let loop_length = new_count - old_count;
    let cycle_count = new_count;
    let cycles_left = target_cycles - cycle_count;
    let cycles_needed_to_simulate = cycles_left % loop_length;

    for _ in 0..cycles_needed_to_simulate {
        grid = cycle(grid);
    }

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
    fn test_tilt_west() {
        let input = include_str!("../../test.txt");
        let (input, lines) = separated_list1(newline, symbols)(input).unwrap();
        assert_eq!(input, "");

        let mut grid: Grid<Symbol> = grid![];
        for line in lines {
            grid.push_row(line);
        }

        println!("Original:");
        print_grid(&grid);
        println!();

        grid = tilt_west(grid);
        println!("Tilted west:");
        print_grid(&grid);
    }

    #[test]
    fn test_tilting() {
        let input = include_str!("../../test.txt");
        let (input, lines) = separated_list1(newline, symbols)(input).unwrap();
        assert_eq!(input, "");

        let mut grid: Grid<Symbol> = grid![];
        for line in lines {
            grid.push_row(line);
        }

        println!("Original:");
        print_grid(&grid);

        grid = tilt_north(grid);
        println!("Tilted north:");
        print_grid(&grid);

        grid = tilt_west(grid);
        println!("Tilted west:");
        print_grid(&grid);

        grid = tilt_south(grid);
        println!("Tilted south:");
        print_grid(&grid);

        grid = tilt_east(grid);
        println!("Tilted east:");
        print_grid(&grid);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../test.txt");
        assert_eq!(part2(input), 64);
    }
}
