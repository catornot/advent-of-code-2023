use std::{
    collections::HashSet,
    ops::{Add, Not},
};

use itertools::{Itertools, Position};

use crate::Day;

pub struct Day11;

impl Day for Day11 {
    fn example_input(&self) -> (&'static str, &'static str) {
        (
            r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#,
            "",
        )
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("374", "")
    }

    fn part_1(&mut self, input: String) -> String {
        let mut galatic_map = input
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect::<Vec<Vec<bool>>>();

        // todo implement doubling yk
        print_galaxies(&galatic_map);

        let mut offset = 0usize;
        galatic_map.len();
        for x in (0..(galatic_map[0].len()))
            .into_iter()
            .filter_map(|x| {
                (0..galatic_map.len())
                    .into_iter()
                    .map(move |y| [x, y])
                    .fold(false, |found, [x, y]| found || galatic_map[y][x])
                    .not()
                    .then(move || x)
            })
            .collect::<Vec<usize>>()
        {
            for y in 0..galatic_map.len() {
                galatic_map[y].insert(x + offset, false);
            }

            offset += 1;
        }

        offset = 0usize;
        galatic_map.len();
        for y in (0..(galatic_map.len()))
            .into_iter()
            .filter_map(|y| {
                (0..galatic_map[0].len())
                    .into_iter()
                    .map(move |x| [x, y])
                    .fold(false, |found, [x, y]| found || galatic_map[y][x])
                    .not()
                    .then(move || y)
            })
            .collect::<Vec<usize>>()
        {
            let len = galatic_map[y + offset].len();
            galatic_map.insert(y + offset, (0..len).map(|_| false).collect());

            offset += 1;
        }

        print_galaxies(&galatic_map);

        let mut visited_pairs: HashSet<([usize; 2], [usize; 2])> = HashSet::new();
        let len = galatic_map.len();
        (0..(galatic_map[0].len()))
            .into_iter()
            .map(|x| (0..len).into_iter().map(move |y| [x, y]))
            .flatten()
            .map(|[x, y]| {
                if !galatic_map[y][x] {
                    return 0;
                }

                let galaxy_positions = galatic_map
                    .iter()
                    .enumerate()
                    .map(|(y, row)| {
                        row.iter()
                            .enumerate()
                            .filter(|(_, is_galaxy)| **is_galaxy)
                            .map(move |(x, _)| [x, y])
                    })
                    .flatten()
                    .filter(|pos| *pos != [x, y])
                    .filter(|pos| visited_pairs.contains(&(*pos, [x, y])).not())
                    .collect::<Vec<[usize; 2]>>();

                galaxy_positions.iter().for_each(|pos| {
                    visited_pairs.insert(([x, y], *pos));
                    visited_pairs.insert((*pos, [x, y]));
                });

                galaxy_positions
                    .into_iter()
                    .map(move |[rx, ry]| {
                        (rx as i64 - x as i64)
                            .abs()
                            .add((ry as i64 - y as i64).abs()) as u64
                    })
                    .sum()
            })
            .sum::<u64>()
            .to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        _ = input;
        "".to_string()
    }
}

fn print_galaxies(grid: &[Vec<bool>]) {
    grid.iter()
        .map(|points| points.iter().with_position())
        .flatten()
        .for_each(|(position, point)| match (position, point) {
            (Position::Last | Position::Only, true) => println!("#"),
            (Position::Last | Position::Only, false) => println!("."),
            (_, true) => print!("#"),
            (_, false) => print!("."),
        })
}
