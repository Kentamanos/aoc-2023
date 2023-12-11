use std::vec;

use itertools::Itertools;

use regex::Regex;

#[derive(Debug, Eq, PartialEq, Hash)]
struct RowCol {
    row: usize,
    col: usize,
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("part1: {}", part1(input));
}

fn galaxy_row(input: &str) -> Vec<usize> {
    let mut results = Vec::new();
    let re = Regex::new(r"\#").unwrap();

    re.find_iter(input).for_each(|m| results.push(m.start()));

    results
}

fn part1(input: &str) -> u32 {
    let num_cols = input.lines().next().unwrap().len();

    // Collect galaxy in rows
    let mut galaxy_rows = Vec::<_>::new();
    for line in input.lines() {
        galaxy_rows.push(galaxy_row(line));
    }

    println!("Initial Rows - {:?}", galaxy_rows);

    // Detect empty rows and add rows as necessary
    let mut expanded_galaxy_rows = Vec::<_>::new();
    galaxy_rows.iter().for_each(|row| {
        expanded_galaxy_rows.push(row.clone());
        if row.is_empty() {
            expanded_galaxy_rows.push(vec![]);
        }
    });

    println!("Expanded Rows - {:?}", expanded_galaxy_rows);

    // Collect galxies in columns
    let mut galaxy_cols = Vec::<_>::new();
    for col in 0..num_cols {
        let mut galaxy_col = Vec::<_>::new();
        expanded_galaxy_rows
            .iter()
            .enumerate()
            .for_each(|(row_number, row_data)| {
                for col_val in row_data.iter() {
                    if *col_val == col {
                        galaxy_col.push(row_number);
                    }
                }
            });
        galaxy_cols.push(galaxy_col);
    }

    println!("Initial Columns - {:?}", galaxy_cols);

    // Detect empty cols and add cols as necessary
    let mut expanded_galaxy_cols = Vec::<_>::new();
    galaxy_cols.iter().for_each(|col| {
        expanded_galaxy_cols.push(col.clone());
        if col.is_empty() {
            expanded_galaxy_cols.push(vec![]);
        }
    });

    println!("Expanded Columns - {:?}", expanded_galaxy_cols);

    // Collect galaxies into a HashSet
    // let mut galaxy_hash: HashSet<RowCol> = HashSet::new();
    let mut galax_vec = Vec::<_>::new();
    for (col_num, col_data) in expanded_galaxy_cols.iter().enumerate() {
        for row_num in col_data.iter() {
            galax_vec.push(RowCol {
                row: *row_num,
                col: col_num,
            });
        }
    }

    let total = galax_vec
        .iter()
        .combinations(2)
        .map(|x| {
            let row_diff = x[0].row as i32 - x[1].row as i32;
            let col_diff = x[0].col as i32 - x[1].col as i32;
            (row_diff.abs() + col_diff.abs()) as u32
        })
        .sum::<u32>();

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test = include_str!("../../test.txt");
        assert_eq!(part1(test), 374);
    }
}
