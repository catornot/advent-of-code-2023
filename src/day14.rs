use std::ops::Range;

use itertools::{Itertools, Position};

use crate::Day;

enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
enum RockType {
    Round,
    Cube,
    None,
}

impl RockType {
    fn is_falling_rock(&self) -> bool {
        if let RockType::Round = self {
            return true;
        }
        false
    }
}

impl From<char> for RockType {
    fn from(value: char) -> Self {
        match value {
            'O' => RockType::Round,
            '#' => RockType::Cube,
            '.' => Self::None,
            _ => panic!("only 3 rock types should exists"),
        }
    }
}

pub struct Day14;

impl Day for Day14 {
    fn example_input(&self) -> (&'static str, &'static str) {
        let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;
        (input, input)
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("136", "64")
    }

    fn part_1(&mut self, input: String) -> String {
        let mut beam = input
            .lines()
            .map(|line| line.chars().map(RockType::from).collect())
            .collect::<Vec<Vec<RockType>>>();

        let x_len = beam[0].len();
        for (range, [x, y]) in (1..input.lines().count())
            .map(|y| (0..x_len).map(move |x| (0..y, [x, y])))
            .flatten()
        {
            if !beam[y][x].is_falling_rock() {
                continue;
            }

            let drop_y = range
                .into_iter()
                .rev()
                .find(|y| beam[*y][x] != RockType::None)
                .map(|drop_y| drop_y + 1)
                .unwrap_or_else(|| 0);

            if drop_y != y {
                checked_drop(&mut beam, [x, drop_y]);
                beam[y][x] = RockType::None;
            }
        }

        caculate_load(beam)
    }

    fn part_2(&mut self, input: String) -> String {
        let mut beam = input
            .lines()
            .map(|line| line.chars().map(RockType::from).collect())
            .collect::<Vec<Vec<RockType>>>();

        let x_len = beam[0].len();
        let y_len = beam.len();
        // let cached = caculate_load(beam.clone());
        for cycle in 1..=1000000000 {
            for (range, dir, [x, y]) in get_cycle_iter(cycle, x_len, y_len).into_iter() {
                if !beam[y][x].is_falling_rock() {
                    continue;
                }

                match dir {
                    Direction::North | Direction::South => {
                        let drop_y = match dir {
                            Direction::North => range
                                .into_iter()
                                .rev()
                                .find(|y| beam[*y][x] != RockType::None)
                                .map(|drop_y| drop_y + 1)
                                .unwrap_or_else(|| 0),
                            _ => range
                                .into_iter()
                                .find(|y| beam[*y][x] != RockType::None)
                                .map(|drop_y| drop_y - 1)
                                .unwrap_or_else(|| y_len - 1),
                        };

                        // print_grid(&beam);
                        // dbg!((drop_y, [x, y]));

                        if drop_y != y {
                            checked_drop(&mut beam, [x, drop_y]);
                            beam[y][x] = RockType::None;
                        }
                    }
                    Direction::East | Direction::West => {
                        let drop_x = match dir {
                            Direction::West => range
                                .rev()
                                .into_iter()
                                .find(|x| beam[y][*x] != RockType::None)
                                .map(|drop_x| drop_x + 1)
                                .unwrap_or_else(|| 0),

                            _ => range
                                .into_iter()
                                .find(|x| beam[y][*x] != RockType::None)
                                .map(|drop_x| drop_x - 1)
                                .unwrap_or_else(|| x_len - 1),
                        };

                        if drop_x != x {
                            checked_drop(&mut beam, [drop_x, y]);
                            beam[y][x] = RockType::None;
                        }
                    }
                }
            }

            // print_grid(&beam);
            // if cached == caculate_load(beam.clone()) {
            //     println!("found possible cycle around {cycle}");
            // }
        }

        caculate_load(beam)
    }
}

fn get_cycle_iter(
    current_cycle: usize,
    x_len: usize,
    y_len: usize,
) -> Box<dyn Iterator<Item = (Range<usize>, Direction, [usize; 2])>> {
    match current_cycle {
        cycle if cycle % 4 == 0 => Box::new(
            (0..(x_len - 1))
                .rev()
                .map(move |x| (0..y_len).map(move |y| ((x + 1)..x_len, [x, y])))
                .flatten()
                .map(|(range, pos)| (range, Direction::East, pos)),
        ),
        cycle if cycle % 3 == 0 => Box::new(
            (0..(y_len - 1))
                .rev()
                .map(move |y| (0..x_len).map(move |x| ((y + 1)..y_len, [x, y])))
                .flatten()
                .map(|(range, pos)| (range, Direction::South, pos)),
        ),
        cycle if cycle % 2 == 0 => Box::new(
            (1..x_len)
                .map(move |x| (0..y_len).map(move |y| (0..x, [x, y])))
                .flatten()
                .map(|(range, pos)| (range, Direction::West, pos)),
        ),
        _ => Box::new(
            (1..y_len)
                .map(move |y| (0..x_len).map(move |x| (0..y, [x, y])))
                .flatten()
                .map(|(range, pos)| (range, Direction::North, pos)),
        ),
    }
}

fn checked_drop(beam: &mut [Vec<RockType>], pos: [usize; 2]) {
    if beam[pos[1]][pos[0]] != RockType::None {
        panic!("false drop_y");
    }

    beam[pos[1]][pos[0]] = RockType::Round;
}

fn caculate_load(mut beam: Vec<Vec<RockType>>) -> String {
    beam.reverse();
    beam.into_iter()
        .enumerate()
        .map(|(height, rocks)| (height + 1, rocks))
        .map(|(height, rocks)| {
            rocks
                .into_iter()
                .filter_map(move |rock| rock.is_falling_rock().then(move || height))
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

#[allow(unused)]
fn print_grid(beam: &[Vec<RockType>]) {
    beam.iter()
        .map(|points| points.iter().with_position())
        .flatten()
        .for_each(|(position, rock)| match (position, rock) {
            (Position::Last | Position::Only, RockType::Round) => println!("O"),
            (Position::Last | Position::Only, RockType::Cube) => println!("#"),
            (Position::Last | Position::Only, RockType::None) => println!("."),
            (_, RockType::Round) => print!("O"),
            (_, RockType::Cube) => print!("#"),
            (_, RockType::None) => print!("."),
        })
}
