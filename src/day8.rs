use std::{collections::HashMap, str::Split};

use crate::Day;

pub struct Day8;

impl Day for Day8 {
    fn example_input(&self) -> (&'static str, &'static str) {
        (
            r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#,
            r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#,
        )
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("2", "6")
    }

    fn part_1(&mut self, input: String) -> String {
        let mut lines = input.split('\n');
        let (mut tha_path, path_parsed) = parse_input(&mut lines);

        let mut steps = 0;
        let mut current_location = "AAA";
        loop {
            if current_location == "ZZZ" {
                break;
            }

            let choice = path_parsed
                .get(current_location)
                .expect("should be a valid key");

            current_location = match tha_path.next().expect("cycle should always yield") {
                'L' => choice.0,
                'R' => choice.1,
                _ => unreachable!("or else the parsing was wrong"),
            };

            steps += 1;
        }

        steps.to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        let mut lines = input.split('\n');
        let (mut tha_path, path_parsed) = parse_input(&mut lines);

        let mut steps = 0;
        let mut current_locations = dbg!(path_parsed
            .keys()
            .filter(|key| key.ends_with('A'))
            .map(|start| *start)
            .collect::<Vec<&str>>());
        loop {
            let mut ends_found = 0u64;
            let next_turn = tha_path.next().expect("cycle should always yield");

            for current_location in current_locations.iter_mut() {
                let choice = path_parsed
                    .get(current_location)
                    .expect("should be a valid key");

                *current_location = match next_turn {
                    'L' => choice.0,
                    'R' => choice.1,
                    _ => unreachable!("or else the parsing was wrong"),
                };

                if current_location.ends_with('Z') {
                    ends_found += 1;
                }
            }
            // *dbg!(&mut steps) += 1;
            steps += 1;

            if ends_found == current_locations.len() as u64 {
                println!("aya");
                break;
            }
        }

        steps.to_string()
    }
}

fn parse_input<'a>(
    lines: &'a mut Split<char>,
) -> (
    impl Iterator<Item = char> + 'a,
    HashMap<&'a str, (&'a str, &'a str)>,
) {
    let tha_path = lines
        .next()
        .expect("should have first lines")
        .chars()
        .cycle();
    _ = lines.next();

    let path_parsed: HashMap<&str, (&str, &str)> = lines
        .filter_map(|line| line.split_once(' '))
        .rfold(HashMap::new(), |mut map, (key, path_split)| {
            map.insert(
                key.trim(),
                path_split
                    .trim()
                    .split_once(",")
                    .map(|(path1, path2)| {
                        (
                            path1.trim().split_once('(').expect("( should be there").1,
                            path2.trim().split_once(')').expect(") should be there").0,
                        )
                    })
                    .unwrap(),
            );
            map
        });

    (tha_path, path_parsed)
}
