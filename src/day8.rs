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
        let (tha_path, path_parsed) = parse_input(&mut lines);
        let mut tha_path = tha_path.cycle();

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
        let (tha_path, path_parsed) = parse_input(&mut lines);
        let tha_path = tha_path.collect::<Vec<char>>();

        let mut current_locations = path_parsed
            .keys()
            .filter(|key| key.ends_with('A'))
            .map(|start| *start)
            .collect::<Vec<&str>>();
        assert_eq!(
            current_locations.len(),
            path_parsed
                .keys()
                .filter(|key| key.ends_with('Z'))
                .map(|start| *start)
                .collect::<Vec<&str>>()
                .len()
        ); // sanity check pls work

        let steps: u64 = current_locations
            .iter_mut()
            .map(|current_location| (current_location, tha_path.iter().cloned().cycle()))
            .map(|(current_location, mut tha_path)| {
                let mut steps = 0u64;
                loop {
                    if current_location.ends_with('Z') {
                        break;
                    }

                    let next_turn = tha_path.next().expect("cycle should always yield");

                    let choice = path_parsed
                        .get(current_location)
                        .expect("should be a valid key");

                    *current_location = match next_turn {
                        'L' => choice.0,
                        'R' => choice.1,
                        _ => unreachable!("or else the parsing was wrong"),
                    };

                    steps += 1;
                }

                steps
            })
            .reduce(|total_steps, steps| lcm(steps, total_steps))
            .unwrap();

        steps.to_string()
    }
}

fn parse_input<'a>(
    lines: &'a mut Split<char>,
) -> (
    impl Iterator<Item = char> + 'a + Clone,
    HashMap<&'a str, (&'a str, &'a str)>,
) {
    let tha_path = lines.next().expect("should have first lines").chars();
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

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

// slightly modified function from num-integer crate
fn gcd(mut a: u64, mut b: u64) -> u64 {
    // Use Stein's algorithm
    if a == 0 || b == 0 {
        return a | b;
    }

    // find common factors of 2
    let shift = (a | b).trailing_zeros();

    // The algorithm needs positive numbers, but the minimum value
    // can't be represented as a positive one.
    // It's also a power of two, so the gcd can be
    // calculated by bitshifting in that case

    // Assuming two's complement, the number created by the shift
    // is positive for all numbers except gcd = abs(min value)
    // The call to .abs() causes a panic in debug mode
    if a == 0 || b == 0 {
        return 1 << shift;
    }

    // divide n and m by 2 until odd
    a >>= a.trailing_zeros();
    b >>= b.trailing_zeros();

    while a != b {
        if a > b {
            a -= b;
            a >>= a.trailing_zeros();
        } else {
            b -= a;
            b >>= b.trailing_zeros();
        }
    }
    a << shift
}

// fn gcd(a: u64, b: u64) -> u64 {
//     if a == 0 {
//         return b;
//     }
//     if b == 0 {
//         return a;
//     }

//     if !a & 1 == 1 {
//         if b & 1 == 1 {
//             return gcd(a >> 1, b);
//         } else {
//             return gcd(a >> 1, b >> 1) << 1;
//         }
//     }
//     if !b & 1 == 1 {
//         return gcd((a - b) >> 1, b);
//     }

//     gcd((b - a) >> 1, a)
// }
