use std::{collections::HashSet, ops::Not};

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
        )
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("374", "82000210")
    }

    fn part_1(&mut self, input: String) -> String {
        calculate_expansion(&input, 2)
    }

    fn part_2(&mut self, input: String) -> String {
        calculate_expansion(&input, 1000000)
    }
}

fn calculate_expansion(input: &str, expansion_size: u64) -> String {
    let mut galatic_map = input
        .lines()
        .map(|line| line.chars().map(|c| (1, c == '#')).collect())
        .collect::<Vec<Vec<(u64, bool)>>>();

    galatic_map.len();
    for x in (0..(galatic_map[0].len()))
        .into_iter()
        .filter_map(|x| {
            (0..galatic_map.len())
                .into_iter()
                .map(move |y| [x, y])
                .fold(false, |found, [x, y]| found || galatic_map[y][x].1)
                .not()
                .then(move || x)
        })
        .collect::<Vec<usize>>()
    {
        for y in 0..galatic_map.len() {
            galatic_map[y][x] = (expansion_size, false);
        }
    }

    galatic_map.len();
    for y in (0..(galatic_map.len()))
        .into_iter()
        .filter_map(|y| {
            (0..galatic_map[0].len())
                .into_iter()
                .map(move |x| [x, y])
                .fold(false, |found, [x, y]| found || galatic_map[y][x].1)
                .not()
                .then(move || y)
        })
        .collect::<Vec<usize>>()
    {
        for x in 0..galatic_map[y].len() {
            galatic_map[y][x] = (expansion_size, false);
        }
    }

    let mut visited_pairs: HashSet<([usize; 2], [usize; 2])> = HashSet::new();
    let len = galatic_map.len();
    (0..(galatic_map[0].len()))
        .into_iter()
        .map(|x| (0..len).into_iter().map(move |y| [x, y]))
        .flatten()
        .map(|[x, y]| {
            if !galatic_map[y][x].1 {
                return 0;
            }

            let galaxy_positions = galatic_map
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .filter(|(_, is_galaxy)| is_galaxy.1)
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
                .map(|[rx, ry]| {
                    (x.min(rx)..x.max(rx))
                        .map(|x| [x, y.min(ry)])
                        .map(|[x, y]| galatic_map[y][x].0)
                        .sum::<u64>()
                        + (y.min(ry)..y.max(ry))
                            .map(|y| [x.min(rx), y])
                            .map(|[x, y]| galatic_map[y][x].0)
                            .sum::<u64>()
                })
                .sum()
        })
        .sum::<u64>()
        .to_string()
}
