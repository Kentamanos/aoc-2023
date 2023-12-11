use itertools::Itertools;

use regex::Regex;

#[derive(Debug, Eq, PartialEq, Hash)]
struct GalaxyLocation {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct EmptySpace {
    rows: usize,
    cols: usize,
}

#[derive(Debug)]
enum Space {
    EmptySpace(EmptySpace),
    Galaxy,
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("part1: {}", part2(1_000_000usize, input));
}

fn galaxy_row(input: &str) -> Vec<usize> {
    let mut results = Vec::new();
    let re = Regex::new(r"\#").unwrap();

    re.find_iter(input).for_each(|m| results.push(m.start()));

    results
}

fn part2(factor: usize, input: &str) -> u64 {
    let num_cols = input.lines().next().unwrap().len();

    // Collect galaxy in rows
    let mut galaxy_rows = Vec::<_>::new();
    for line in input.lines() {
        galaxy_rows.push(galaxy_row(line));
    }

    // println!("Initial Rows - {:?}", galaxy_rows);

    // Convert to EmptySpace format
    let mut spaced_galaxy_rows = Vec::<_>::new();
    galaxy_rows.iter().for_each(|row| {
        let mut spaced_galaxy_row = Vec::<_>::new();
        if row.is_empty() {
            spaced_galaxy_row.push(Space::EmptySpace(EmptySpace {
                rows: 1,
                cols: num_cols,
            }));
        } else {
            let mut current_column = 0;
            for galaxy in row.iter() {
                if galaxy > &current_column {
                    let empty_space = EmptySpace {
                        rows: 1,
                        cols: galaxy - current_column,
                    };
                    spaced_galaxy_row.push(Space::EmptySpace(empty_space));
                }
                spaced_galaxy_row.push(Space::Galaxy);
                current_column = galaxy + 1;
            }
            if current_column < num_cols {
                let empty_space = EmptySpace {
                    rows: 1,
                    cols: num_cols - current_column,
                };
                spaced_galaxy_row.push(Space::EmptySpace(empty_space));
            }
        }
        spaced_galaxy_rows.push(spaced_galaxy_row);
    });

    // spaced_galaxy_rows.iter().for_each(|x| println!("{:?}", x));

    // Detect empty rows and expand them to factor size
    let mut expanded_galaxy_rows: Vec<Vec<Space>> = Vec::<Vec<_>>::new();
    spaced_galaxy_rows.into_iter().for_each(|row| {
        let mut expanded_galaxy_row: Vec<Space> = Vec::<_>::new();
        if row.len() == 1 {
            assert!(matches!(row[0], Space::EmptySpace(_)));
            let empty_space = EmptySpace {
                rows: factor,
                cols: num_cols,
            };
            expanded_galaxy_row.push(Space::EmptySpace(empty_space));
        } else {
            for space in row.iter() {
                match space {
                    Space::EmptySpace(empty_space) => {
                        // TODO: Cloning here if we want to actually do that later
                        expanded_galaxy_row.push(Space::EmptySpace(EmptySpace {
                            rows: empty_space.rows,
                            cols: empty_space.cols,
                        }));
                    }
                    Space::Galaxy => {
                        expanded_galaxy_row.push(Space::Galaxy);
                    }
                }
            }
        }
        expanded_galaxy_rows.push(expanded_galaxy_row);
    });

    // expanded_galaxy_rows
    //     .iter()
    //     .for_each(|x| println!("{:?}", x));

    // Collect into columns
    let mut spaced_galaxy_columns: Vec<Vec<Space>> = Vec::<Vec<_>>::new();
    for col in 0..num_cols {
        let items_at_column: Vec<_> = expanded_galaxy_rows
            .iter()
            .map(|row| {
                let mut row_index = 0usize;
                row.iter()
                    .find_map(|row_item| match row_item {
                        Space::EmptySpace(es) => {
                            if col >= row_index && col < (row_index + es.cols) {
                                Some(Space::EmptySpace(EmptySpace {
                                    rows: es.rows,
                                    cols: 1,
                                }))
                            } else {
                                row_index += es.cols;
                                None
                            }
                        }
                        Space::Galaxy => {
                            if row_index == col {
                                Some(Space::Galaxy)
                            } else {
                                row_index += 1;
                                None
                            }
                        }
                    })
                    .unwrap()
            })
            .collect();

        spaced_galaxy_columns.push(items_at_column);
    }

    // spaced_galaxy_columns.iter().for_each(|x| {
    //     println!("{:?}", x);
    //     println!();
    // });

    // Compress spaced galaxy columns (combine empty spaces)
    let mut compressed_galaxy_columns: Vec<Vec<Space>> = Vec::<Vec<_>>::new();
    spaced_galaxy_columns.iter().for_each(|column| {
        let mut compressed_galaxy_column: Vec<Space> = Vec::<_>::new();
        let mut empty_space_total = 0usize;
        for space in column {
            match space {
                Space::EmptySpace(es) => {
                    assert_eq!(es.cols, 1);
                    empty_space_total += es.rows;
                }
                Space::Galaxy => {
                    // Handle any pending space first
                    if empty_space_total > 0 {
                        compressed_galaxy_column.push(Space::EmptySpace(EmptySpace {
                            rows: empty_space_total,
                            cols: 1,
                        }));
                        empty_space_total = 0;
                    }
                    compressed_galaxy_column.push(Space::Galaxy);
                }
            }
        }
        // Handle any remaining space
        if empty_space_total > 0 {
            compressed_galaxy_column.push(Space::EmptySpace(EmptySpace {
                rows: empty_space_total,
                cols: 1,
            }));
        }

        compressed_galaxy_columns.push(compressed_galaxy_column);
    });

    // compressed_galaxy_columns.iter().for_each(|x| {
    //     println!("{:?}", x);
    //     println!();
    // });

    // Expand out empty columns
    let mut expanded_galaxy_columns: Vec<Vec<Space>> = Vec::<Vec<_>>::new();
    compressed_galaxy_columns.into_iter().for_each(|col| {
        let mut expanded_galaxy_column: Vec<Space> = Vec::<_>::new();
        if col.len() == 1 {
            assert!(matches!(col[0], Space::EmptySpace(_)));
            if let Space::EmptySpace(empty_space) = &col[0] {
                let new_es = EmptySpace {
                    rows: empty_space.rows,
                    cols: factor,
                };

                expanded_galaxy_column.push(Space::EmptySpace(new_es));
            }
        } else {
            for space in col.iter() {
                match space {
                    Space::EmptySpace(es) => {
                        expanded_galaxy_column.push(Space::EmptySpace(EmptySpace {
                            rows: es.rows,
                            cols: es.cols,
                        }));
                    }
                    Space::Galaxy => expanded_galaxy_column.push(Space::Galaxy),
                }
            }
        }
        expanded_galaxy_columns.push(expanded_galaxy_column);
    });

    // expanded_galaxy_columns.iter().for_each(|x| {
    //     println!("{:?}", x);
    //     println!();
    // });

    // TODO: Instead of a mut/loop to add, we should see if we can use map_find to collect them all
    // Take final structure and just pull out galaxy's with locations
    let mut galaxy_locations = Vec::<GalaxyLocation>::new();
    let mut current_col = 0usize;
    expanded_galaxy_columns.iter().for_each(|col| {
        if col.len() == 1 {
            assert!(matches!(col[0], Space::EmptySpace(_)));
            if let Space::EmptySpace(empty_space) = &col[0] {
                current_col += empty_space.cols;
            }
        } else {
            let mut current_row = 0usize;
            for space in col.iter() {
                match space {
                    Space::EmptySpace(es) => {
                        assert!(es.cols == 1);
                        current_row += es.rows;
                    }
                    Space::Galaxy => {
                        galaxy_locations.push(GalaxyLocation {
                            row: current_row,
                            col: current_col,
                        });
                        current_row += 1;
                    }
                }
            }
            current_col += 1;
        }
    });

    println!("Galaxy Locations - {:?}", galaxy_locations);

    let total = galaxy_locations
        .iter()
        .combinations(2)
        .map(|x| {
            let row_diff = x[0].row as i64 - x[1].row as i64;
            let col_diff = x[0].col as i64 - x[1].col as i64;
            (row_diff.abs() + col_diff.abs()) as u64
        })
        .sum::<u64>();

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let test = include_str!("../../test.txt");
        assert_eq!(part2(10usize, test), 1030);
        assert_eq!(part2(100usize, test), 8410);
    }
}
