use core::num::dec2flt::parse;

use itertools::Itertools;

use crate::Day;

pub struct Day9;

impl Day for Day9 {
    fn example_input(&self) -> (&'static str, &'static str) {
        (
            r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#,
            "",
        )
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("114", "")
    }

    fn part_1(&mut self, input: String) -> String {
        let first_layers = input
            .lines()
            .map(|line| {
                line.trim()
                    .split_whitespace()
                    .map(|num| num.parse::<u64>().expect("input should be valid"))
                    .collect::<Vec<u64>>()
            })
            .collect::<Vec<Vec<u64>>>();

        first_layers.into_iter().map(process_layer).sum::<u64>().to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        _ = input;
        "".to_string()
    }
}

fn process_layer(layer: Vec<u64>) -> u64 {
    let layers = vec![layer,Vec::new()];

    get_next_layers(&mut layers, &mut 1); 
        
    todo!()
}

fn get_next_layers(layers: Vec<Vec<u64>>, depth: usize) {
    layers[depth - 1].iter().tuple_windows::<(u64,u64)>().map(|first, second| first - second ).col
    
    todo!()
}
