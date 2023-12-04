use std::{ops::Not, str::Split};

use crate::Day;

pub struct Day4;

impl Day for Day4 {
    fn example_input(&self) -> (&'static str, &'static str) {
        (
            r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#,
            r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#,
        )
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("13", "30")
    }

    fn part_1(&mut self, input: String) -> String {
        input
            .split('\n')
            .filter_map(|line| line.trim().split_once(':')?.1.trim().split_once('|'))
            .map(|(win, got_numbers)| caculate_all_winning_cards(win.split(' '), got_numbers))
            .map(|matched| {
                matched
                    .eq(&0)
                    .not()
                    .then(|| 2_usize.pow(matched as u32 - 1))
                    .unwrap_or_else(|| 0)
            })
            .sum::<usize>()
            .to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        let mut cards = vec![0; input.split('\n').count()];

        input
            .split('\n')
            .enumerate()
            .filter_map(|(i, line)| {
                Some((i, line.trim().split_once(':')?.1.trim().split_once('|')?))
            })
            .map(|(i, (win, got_numbers))| {
                (i, caculate_all_winning_cards(win.split(' '), got_numbers))
            })
            .map(|(i, amount)| {
                cards[i] += 1;
                (0..cards[i])
                    .map(|_| ((i + 1)..(i + 1 + amount)))
                    .flatten()
                    .for_each(|i| cards[i] += 1);
                cards[i]
            })
            .sum::<usize>()
            .to_string()
    }
}

fn caculate_all_winning_cards(winning: Split<char>, got_numbers: &str) -> usize {
    winning
        .into_iter()
        .map(|winning| winning.trim())
        .filter(|winning| !winning.is_empty())
        .map(|winning| {
            got_numbers
                .split(' ')
                .filter(|number| winning == *number)
                .count()
        })
        .sum::<usize>()
}
