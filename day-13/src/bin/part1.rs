use grid::*;
use itertools::Itertools;
use nom::{
    character::complete::{newline, one_of},
    multi::{many1, many_m_n, separated_list1},
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Symbol {
    Period,
    Hash,
}

enum Mirroring {
    Vertical(usize),
    Horizontal(usize),
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("part1: {}", part1(input));
}

fn symbols(input: &str) -> IResult<&str, Vec<Symbol>> {
    let (input, symbol_chars) = many1(one_of("#."))(input)?;
    let symbols: Vec<_> = symbol_chars
        .iter()
        .map(|c| match c {
            '#' => Symbol::Hash,
            '.' => Symbol::Period,
            _ => panic!("unexpected symbol"),
        })
        .collect();

    Ok((input, symbols))
}

fn pattern(input: &str) -> IResult<&str, Grid<Symbol>> {
    let (input, lines) = separated_list1(newline, symbols)(input)?;

    let mut grid: Grid<Symbol> = grid![];
    for line in lines {
        grid.push_row(line);
    }

    Ok((input, grid))
}

fn detect_horizontal_mirroring(grid: &Grid<Symbol>) -> Option<Mirroring> {
    let mut candidate_indices = Vec::new();
    for (i, (a, b)) in grid.iter_rows().tuple_windows::<(_, _)>().enumerate() {
        if a.eq(b) {
            candidate_indices.push(i);
        }
    }

    for index in candidate_indices {
        let a_iter = (0..=index).rev();
        let b_iter = index + 1..grid.rows();

        let mut combo = a_iter.zip(b_iter);
        if combo.all(|(a, b)| {
            let a_row = grid.iter_row(a);
            let b_row = grid.iter_row(b);
            a_row.eq(b_row)
        }) {
            return Some(Mirroring::Horizontal(index));
        }
    }

    None
}

fn detect_vertical_mirroring(grid: &Grid<Symbol>) -> Option<Mirroring> {
    let mut candidate_indices = Vec::new();
    for (i, (a, b)) in grid.iter_cols().tuple_windows::<(_, _)>().enumerate() {
        if a.eq(b) {
            candidate_indices.push(i);
        }
    }

    for index in candidate_indices {
        let a_iter = (0..=index).rev();
        let b_iter = index + 1..grid.cols();

        let mut combo = a_iter.zip(b_iter);
        if combo.all(|(a, b)| {
            let a_row = grid.iter_col(a);
            let b_row = grid.iter_col(b);
            a_row.eq(b_row)
        }) {
            return Some(Mirroring::Vertical(index));
        }
    }

    None
}

fn detect_mirroring(grid: &Grid<Symbol>) -> Option<Mirroring> {
    detect_vertical_mirroring(grid).or(detect_horizontal_mirroring(grid))
}

fn part1(input: &str) -> u64 {
    let (input, grids) = separated_list1(many_m_n(2, 2, newline), pattern)(input).unwrap();
    assert_eq!(input, "");

    let total: u64 = grids
        .iter()
        .map(|g| {
            let mirroring = detect_mirroring(&g);
            match mirroring {
                Some(Mirroring::Vertical(col)) => (col + 1) as u64,
                Some(Mirroring::Horizontal(row)) => (row + 1) as u64 * 100,
                // None => panic!("Found no mirroring"),
                None => 0,
            }
        })
        .sum();

    // let total: u64 = patterns.iter().map(|p| score_pattern(p)).sum();

    // total

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test = include_str!("../../test.txt");
        assert_eq!(part1(test), 405);
    }
}
