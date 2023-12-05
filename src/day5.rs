use std::{
    ops::{Not, Range},
    str::Split,
};

use crate::Day;

#[derive(Debug, Clone)]
struct MapTransition {
    source: Range<u64>,
    start_destination: u64,
}

impl MapTransition {
    fn transition(&self, id: u64) -> Option<u64> {
        if self.source.contains(&id) {
            Some(self.start_destination + id - self.source.start)
        } else {
            None
        }
    }
}

pub struct Day5;

impl Day for Day5 {
    fn example_input(&self) -> (&'static str, &'static str) {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
        (input, input)
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("35", "")
    }

    fn part_1(&mut self, input: String) -> String {
        let mut lines = input.split('\n');
        let mut current_ids = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .split(' ')
            .filter_map(|num| num.parse::<u64>().ok())
            .collect::<Vec<u64>>();
        _ = lines.next();

        while let Some(transitions) = parse_transition(&mut lines) {
            current_ids = current_ids
                .iter()
                .map(|id| {
                    transitions
                        .iter()
                        .map(|rule| rule.transition(*id))
                        .find(Option::is_some)
                        .flatten()
                        .unwrap_or_else(|| *id)
                })
                .collect::<Vec<u64>>()
        }

        current_ids.sort();
        current_ids
            .first()
            .expect("current ids should have ids")
            .to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        _ = input;
        "".into()
    }
}

fn parse_transition(lines: &mut Split<char>) -> Option<Vec<MapTransition>> {
    let mut transitions = Vec::new();

    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            break;
        }
        if line.chars().nth(0).unwrap().is_ascii_alphabetic() {
            continue;
        }

        let mut rules = line.trim().split(' ');
        let destination = rules.next().unwrap().parse().unwrap();
        let source = rules.next().unwrap().parse().unwrap();
        let range: u64 = rules.next().unwrap().parse().unwrap();

        transitions.push(MapTransition {
            source: source..(source + range),
            start_destination: destination,
        })
    }

    transitions.len().eq(&0).not().then(|| transitions)
}
