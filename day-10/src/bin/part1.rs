use core::panic;
use std::collections::HashMap;

use id_arena::{Arena, Id};
use nom::{
    character::complete::newline,
    multi::{many1, separated_list1},
    IResult,
};

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
    row: isize,
    col: isize,
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

    Ok((input, spot_type))
}

fn add_to_path(
    path: &mut Vec<Step>,
    grid: &HashMap<(isize, isize), Id<Spot>>,
    arena: &Arena<Spot>,
) -> bool {
    let _start = path.first().unwrap();

    let cur_step = path.last().unwrap();
    let cur_spot = &arena[cur_step.from];
    let cur_pos = (cur_spot.row, cur_spot.col);
    let cur_direction = cur_step.walk.clone();
    let offset: (isize, isize) = cur_step.walk.clone().into();

    let next_pos = (cur_pos.0 + offset.0, cur_pos.1 + offset.1);
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
        let future_pos = (next_pos.0 + dir_offset.0, next_pos.1 + dir_offset.1);
        if grid.contains_key(&future_pos) {
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
    }

    if found_next_direction.is_some() {
        println!(
            "From: {:?} - direction: {:?}",
            next_spot_id, found_next_direction
        );
        path.push(Step {
            from: *next_spot_id,
            walk: found_next_direction.unwrap(),
        });
    } else {
        for step in path {
            let st = &arena[step.from].spot_type;
            let pos = (arena[step.from].row, arena[step.from].col);
            let dir = step.walk.clone();
            println!("Pos: {:?} - Dir: {:?} - Type: {:?}", pos, dir, st);
        }
        panic!("Couldn't find a next direction");
    }

    false
}

fn walk_the_loop(
    // start: Id<Spot>,
    grid: &HashMap<(isize, isize), Id<Spot>>,
    arena: &Arena<Spot>,
) -> Vec<Step> {
    let start = grid
        .iter()
        .find(|(_, spot)| matches!(arena[**spot].spot_type, SpotType::Start))
        .unwrap()
        .1;

    let cur_pos = (arena[*start].col, arena[*start].row);
    let mut steps = vec![];

    for direction in Direction::all_directions() {
        let dir_offset: (isize, isize) = (direction.clone()).into();
        let next_pos = (cur_pos.0 + dir_offset.0, cur_pos.1 + dir_offset.1);
        if grid.contains_key(&next_pos) {
            let spot_id = grid.get(&next_pos).unwrap();
            if arena[*spot_id].spot_type != SpotType::Ground {
                steps.push(Step {
                    from: *start,
                    walk: direction,
                });
                break;
            }
        }
    }

    let mut done = false;

    while !done {
        done = add_to_path(&mut steps, grid, arena);
    }

    steps
}

fn part1(input: &str) -> u32 {
    let (input, grid) = separated_list1(newline, many1(spot_type))(input).unwrap();
    assert_eq!(input, "");

    let mut spot_arena = Arena::<Spot>::new();
    let mut grid_hash: HashMap<(isize, isize), Id<Spot>> = HashMap::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, spot) in row.iter().enumerate() {
            let spot = spot_arena.alloc(Spot {
                row: y as isize,
                col: x as isize,
                spot_type: spot.clone(),
            });
            grid_hash.insert((y as isize, x as isize), spot);
        }
    }

    let path = walk_the_loop(&grid_hash, &spot_arena);

    println!("Len: {} - {:?}", path.len(), path);

    path.len() as u32 / 2
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("part1: {}", part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test = include_str!("../../test1.txt");
        assert_eq!(part1(test), 4);
    }
}
