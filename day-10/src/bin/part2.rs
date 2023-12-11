use core::panic;
use std::collections::HashMap;

use colored::Colorize;
use id_arena::{Arena, Id};
use iter_tools::Itertools;
use nom::{multi::many1, IResult};

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn all_directions() -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
    }

    fn inverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct RowCol {
    row: isize,
    col: isize,
}

impl From<Direction> for (isize, isize) {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::North => (-1isize, 0isize),
            Direction::South => (1isize, 0isize),
            Direction::East => (0isize, 1isize),
            Direction::West => (0isize, -1isize),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum SpotType {
    Ground,
    Connector(Direction, Direction),
    Start,
}

#[derive(Debug)]
struct Spot {
    spot_type: SpotType,
    row_col: RowCol,
}

#[derive(Debug)]
struct Step {
    from: Id<Spot>,
    walk: Direction,
}

fn spot_type(input: &str) -> IResult<&str, SpotType> {
    let (input, c) = nom::character::complete::one_of("|-LJ7FS.")(input)?;

    let spot_type = match c {
        '|' => SpotType::Connector(Direction::North, Direction::South),
        '-' => SpotType::Connector(Direction::East, Direction::West),
        'L' => SpotType::Connector(Direction::North, Direction::East),
        'J' => SpotType::Connector(Direction::North, Direction::West),
        '7' => SpotType::Connector(Direction::South, Direction::West),
        'F' => SpotType::Connector(Direction::South, Direction::East),
        '.' => SpotType::Ground,
        'S' => SpotType::Start,
        _ => panic!("Unknown character: {}", c),
    };

    // Not sure why this is needed, but it blows up without it...
    // let (input, _) = many0(newline)(input)?;

    Ok((input, spot_type))
}

fn add_to_path(
    path: &mut Vec<Step>,
    grid: &HashMap<RowCol, Id<Spot>>,
    arena: &Arena<Spot>,
) -> bool {
    // let pl = path.len();
    // if pl == 9 {
    //     println!("Path...");
    //     for foo in grid.keys() {
    //         println!("Foo: {:?}", foo);
    //     }
    //     let food = grid.get(&(7isize, 4isize));
    //     println!("Food: {:?}", food);
    // }
    let cur_step = path.last().unwrap();
    let cur_spot = &arena[cur_step.from];
    let cur_pos = &cur_spot.row_col;
    let cur_direction = cur_step.walk.clone();
    let offset: (isize, isize) = cur_step.walk.clone().into();

    let next_pos = RowCol {
        row: cur_pos.row + offset.0,
        col: cur_pos.col + offset.1,
    };
    let next_spot_id = grid.get(&next_pos).unwrap();
    let next_spot = &arena[*next_spot_id];

    let mut directions_to_check = vec![];
    if let SpotType::Connector(d1, d2) = next_spot.spot_type.clone() {
        if d1 != Direction::inverse(&cur_direction) {
            directions_to_check.push(d1);
        }
        if d2 != Direction::inverse(&cur_direction) {
            directions_to_check.push(d2);
        }
    }

    let mut found_next_direction = None;
    // Check all the directions but "backwards"
    for next_direction in directions_to_check {
        let dir_offset: (isize, isize) = next_direction.clone().into();
        let future_pos = RowCol {
            row: next_pos.row + dir_offset.0,
            col: next_pos.col + dir_offset.1,
        };

        let s_id = grid.get(&future_pos);
        if s_id.is_none() {
            println!("WTF!");
        }
        let spot_id = grid.get(&future_pos).unwrap();

        match arena[*spot_id].spot_type.clone() {
            SpotType::Ground => continue,
            SpotType::Connector(_, _) => {
                found_next_direction = Some(next_direction);
                break;
            }
            SpotType::Start => {
                path.push(Step {
                    from: *next_spot_id,
                    walk: next_direction,
                });

                // We found a way back to the start, we're done...
                return true;
            }
        }
    }

    if found_next_direction.is_some() {
        path.push(Step {
            from: *next_spot_id,
            walk: found_next_direction.unwrap(),
        });
    } else {
        println!("Path len: {}", path.len());
        for step in path {
            let st = &arena[step.from].spot_type;
            let pos = &arena[step.from].row_col;
            let dir = step.walk.clone();
            println!("Pos: {:?} - Dir: {:?} - Type: {:?}", pos, dir, st);
        }
        panic!("Couldn't find a next direction");
    }

    false
}

fn walk_the_loop(
    // start: Id<Spot>,
    grid: &HashMap<RowCol, Id<Spot>>,
    arena: &Arena<Spot>,
) -> Vec<Step> {
    let start = grid
        .iter()
        .find(|(_, spot)| matches!(&arena[**spot].spot_type, SpotType::Start))
        .unwrap()
        .1;

    let cur_pos = &arena[*start].row_col;
    let mut steps = vec![];

    for direction in Direction::all_directions() {
        let dir_offset: (isize, isize) = (direction.clone()).into();
        let next_pos = RowCol {
            row: cur_pos.row + dir_offset.0,
            col: cur_pos.col + dir_offset.1,
        };
        // let next_pos = (cur_pos.0 + dir_offset.0, cur_pos.1 + dir_offset.1);
        if grid.contains_key(&next_pos) {
            let spot_id = grid.get(&next_pos).unwrap();
            let spot = &arena[*spot_id];
            match &spot.spot_type {
                SpotType::Ground => continue,
                SpotType::Connector(d1, d2) => {
                    if *d1 == Direction::inverse(&direction)
                        || *d2 == Direction::inverse(&direction)
                    {
                        steps.push(Step {
                            from: *start,
                            walk: direction,
                        });
                        break;
                    }
                }
                SpotType::Start => {
                    panic!("Start shouldn't see a start around it")
                }
            }
        }
    }

    let mut done = false;

    while !done {
        done = add_to_path(&mut steps, grid, arena);
    }

    steps
}

fn part2(input: &str) -> u32 {
    let mut grid: Vec<Vec<SpotType>> = Vec::new();

    let lines = input.lines().collect::<Vec<_>>();
    for line in lines.iter() {
        let mut row_vec: Vec<SpotType> = Vec::new();
        let (_, spots) = many1(spot_type)(line).unwrap();
        for spot in spots.iter() {
            row_vec.push(spot.clone());
        }
        println!("{:?}", row_vec);
        grid.push(row_vec);
    }

    // let (input, grid) = separated_list1(newline, many1(spot_type))(input).unwrap();
    // assert_eq!(input, "");

    let row_count = grid.len();
    let col_count = grid[0].len();

    grid.iter().for_each(|row| println!("{:?}", row.len()));

    // Make sure rows are all the same length
    let grid_iter = grid.iter();
    for (prev, next) in grid_iter.tuple_windows() {
        assert_eq!(prev.len(), next.len());
    }

    let mut spot_arena = Arena::<Spot>::new();
    let mut grid_hash: HashMap<RowCol, Id<Spot>> = HashMap::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, spot) in row.iter().enumerate() {
            println!("X: {} - Y: {}", x, y);
            let row_col = RowCol {
                row: y as isize,
                col: x as isize,
            };

            let spot = spot_arena.alloc(Spot {
                row_col,
                spot_type: spot.clone(),
            });
            grid_hash.insert(
                RowCol {
                    row: y as isize,
                    col: x as isize,
                },
                spot,
            );
        }
    }

    assert!(grid_hash.len() > 0);
    assert!(grid_hash.contains_key(&RowCol { row: 6, col: 4 }));

    let path = walk_the_loop(&grid_hash, &spot_arena);

    let mut spots_visited: HashMap<RowCol, Id<Spot>> = HashMap::new();
    for step in path.iter() {
        let spot = &spot_arena[step.from];
        let pos = RowCol {
            row: spot.row_col.row,
            col: spot.row_col.col,
        };
        spots_visited.insert(pos, step.from);
    }

    // let mut horizontal_scan: HashSet<(isize, isize)> = HashSet::new();

    let mut count = 0;

    for row in 0..row_count as isize {
        let mut inside = false;
        for col in 0..col_count as isize {
            let pos = RowCol { row, col };

            if spots_visited.contains_key(&pos) {
                let spot_id = spots_visited.get(&pos).unwrap();
                let spot = &spot_arena[*spot_id];
                match &spot.spot_type {
                    SpotType::Connector(d1, d2) => {
                        if d1 == &Direction::South || d2 == &Direction::South {
                            print!("!");
                            inside = !inside;
                        } else {
                            print!("+");
                        }
                    }
                    SpotType::Start => {
                        // Find the actual directions the start is pointing...
                        let d1 = path.first().unwrap().walk.clone();
                        let d2 = path.last().unwrap().walk.clone().inverse();
                        if d1 == Direction::South || d2 == Direction::South {
                            inside = !inside;
                        }
                        print!("S");
                    }
                    _ => {
                        panic!("This spot type shouldn't be in our path...");
                    }
                }
            } else if inside {
                // Get the type of tile we're on...
                print!("{}", "I".red());
                count += 1;
            } else {
                print!("O");
            }
        }

        println!();
    }
    println!();
    count
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_1() {
        let test = include_str!("../../test2_1.txt");
        assert_eq!(part2(test), 4);
    }

    #[test]
    fn test_part2_2() {
        let test = include_str!("../../test2_2.txt");
        assert_eq!(part2(test), 4);
    }

    #[test]
    fn test_part2_3() {
        let test = include_str!("../../test2_3.txt");
        assert_eq!(part2(test), 8);
    }

    #[test]
    fn test_part2_4() {
        let test = include_str!("../../test2_4.txt");
        assert_eq!(part2(test), 10);
    }
}
