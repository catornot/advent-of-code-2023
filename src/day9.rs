use itertools::Itertools;

use crate::Day;

pub struct Day9;

impl Day for Day9 {
    fn example_input(&self) -> (&'static str, &'static str) {
        let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
        (input, input)
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("114", "2")
    }

    fn part_1(&mut self, input: String) -> String {
        let first_layers = input
            .lines()
            .map(|line| {
                line.trim()
                    .split_whitespace()
                    .map(|num| num.parse::<i64>().expect("input should be valid"))
                    .collect::<Vec<i64>>()
            })
            .collect::<Vec<Vec<i64>>>();

        first_layers
            .into_iter()
            .map(|layer| process_layer(layer, buble_prediction_part1))
            .sum::<i64>()
            .to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        let first_layers = input
            .lines()
            .map(|line| {
                line.trim()
                    .split_whitespace()
                    .map(|num| num.parse::<i64>().expect("input should be valid"))
                    .collect::<Vec<i64>>()
            })
            .collect::<Vec<Vec<i64>>>();

        first_layers
            .into_iter()
            .map(|layer| process_layer(layer, buble_prediction_part2))
            .sum::<i64>()
            .to_string()
    }
}

fn process_layer(
    layer: Vec<i64>,
    buble_prediction: impl Fn(&Vec<Vec<i64>>, &mut usize, i64) -> i64,
) -> i64 {
    let mut layers = vec![layer];

    get_next_layers(&mut layers, &mut 1);

    buble_prediction(&layers, &mut layers.len().saturating_sub(1), 0)
}

fn get_next_layers(layers: &mut Vec<Vec<i64>>, depth: &mut usize) {
    layers.push(
        layers[*depth - 1]
            .iter()
            .cloned()
            .tuple_windows::<(i64, i64)>()
            .map(|(first, second)| second - first)
            .collect(),
    );

    if layers[*depth].iter().any(|num| *num != 0) {
        *depth += 1;
        get_next_layers(layers, depth);
    }
}

fn buble_prediction_part1(layers: &Vec<Vec<i64>>, depth: &mut usize, current_buble: i64) -> i64 {
    let current_buble = layers[*depth][layers[*depth].len() - 1] + current_buble;
    if *depth != 0 {
        *depth -= 1;
        buble_prediction_part1(layers, depth, current_buble)
    } else {
        current_buble
    }
}

fn buble_prediction_part2(layers: &Vec<Vec<i64>>, depth: &mut usize, current_buble: i64) -> i64 {
    let current_buble = layers[*depth][0] - current_buble;
    if *depth != 0 {
        *depth -= 1;
        buble_prediction_part2(layers, depth, current_buble)
    } else {
        current_buble
    }
}
