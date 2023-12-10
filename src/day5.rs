use itertools::Itertools;
use std::{
    hash::Hash,
    ops::{Not, Range},
    str::Split,
};

use crate::Day;

struct RangeCmp<T: Ord + Eq + PartialEq + PartialOrd + Hash> {
    r: Range<T>,
}

#[allow(unused)]
impl<T: Ord + Eq + PartialEq + PartialOrd + Hash> RangeCmp<T> {
    fn new(range: Range<T>) -> Self {
        Self { r: range }
    }
}

impl<T: Ord + Eq + PartialEq + PartialOrd + Hash> Hash for RangeCmp<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.r.start.hash(state);
        self.r.end.hash(state);
    }
}

impl<T: Ord + Eq + PartialEq + PartialOrd + Hash> Into<Range<T>> for RangeCmp<T> {
    fn into(self) -> Range<T> {
        self.r
    }
}

impl<T: Ord + Eq + PartialEq + PartialOrd + Hash> PartialEq for RangeCmp<T> {
    fn eq(&self, other: &Self) -> bool {
        self.r.start == other.r.start && self.r.end == other.r.end
    }
}

impl<T: Ord + Eq + PartialEq + PartialOrd + Hash> Eq for RangeCmp<T> {}

// impl<T: Ord + Eq + PartialEq + PartialOrd> PartialOrd for RangeCmp<T> {}
// impl<T: Ord + Eq + PartialEq + PartialOrd> Ord for RangeCmp<T> {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum MatchedOn {
    Full,
    End,
    Start,
    None,
}

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

    fn transition_range(
        &self,
        ids: Range<u64>,
    ) -> (MatchedOn, Option<(Range<u64>, Option<Range<u64>>)>) {
        match (
            self.source.contains(&ids.start),
            self.source.contains(&ids.end),
        ) {
            (true, true) => {
                let start = self.start_destination + (ids.start - self.source.start);
                let new_range = start..(start + (ids.end - ids.start));
                debug_print_ids(&self.source, &new_range, "match all");
                (MatchedOn::Full, Some((new_range, None)))
            }
            (true, false) => {
                let remainder = self.source.end - ids.start;
                let start = self.start_destination + (ids.start - self.source.start);
                let new_range = start..(start + (self.source.end - ids.start));
                debug_print_ids(&self.source, &new_range, "match start");
                (
                    MatchedOn::Start,
                    Some((new_range, Some(ids.start..(ids.start + remainder)))),
                )
            }
            (false, true) => {
                let start = self.start_destination + (ids.end - self.source.start - 1);
                let remainder = ids.end - self.source.start;
                let new_range = start..(start + (self.source.end - ids.end));
                debug_print_ids(&self.source, &new_range, "match end");
                (
                    MatchedOn::End,
                    Some((new_range, Some(ids.start..(ids.start + remainder)))),
                )
            }
            _ => (MatchedOn::None, None),
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
        ("35", "46")
    }

    fn part_1(&mut self, input: String) -> String {
        let mut lines = input.split('\n');
        let mut current_ids = parse_first_line(&mut lines);

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
        let mut lines = input.split('\n');
        let mut current_ids = parse_first_line(&mut lines)
            .into_iter()
            .chunks(2)
            .into_iter()
            .map(|mut chunk| [chunk.next().unwrap(), chunk.next().unwrap()])
            .map(|[start, lenght]| start..(start + lenght))
            .collect::<Vec<Range<u64>>>();

        let mut cycle = 0;
        debug_print_vec_ids(&current_ids);
        while let Some(transitions) = parse_transition(&mut lines) {
            current_ids = (current_ids)
                .iter()
                .map(|ids| (check_ranges(ids.clone(), &transitions), ids))
                .map(|(mutliple_ids, ids)| {
                    mutliple_ids
                        .len()
                        .eq(&0)
                        .then(|| vec![ids.clone()])
                        .unwrap_or_else(|| mutliple_ids)
                })
                .flatten()
                .collect::<Vec<Range<u64>>>();
            cycle += 1;
            println!("%{cycle}\n");
        }
        debug_print_vec_ids(&current_ids);

        current_ids.sort_by(|it, other| it.start.cmp(&other.start));
        current_ids
            .first()
            .expect("current ids should have ids")
            .start
            .to_string()
    }
}

fn parse_first_line(lines: &mut Split<char>) -> Vec<u64> {
    let current_ids = lines
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
    current_ids
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

fn check_ranges(ids: Range<u64>, transitions: &[MapTransition]) -> Vec<Range<u64>> {
    let mut remainders = 0;
    let mut remainder: Option<Range<u64>> = None;
    let mut ranges = transitions
        .iter()
        .filter_map(|rule| {
            let (matched, ids) = rule.transition_range(ids.clone());
            ids.map(|ids| {
                ids.1.map(|ids| {
                    remainders += 1;
                    remainder = Some(ids)
                });
                (matched, ids.0)
            })
            // .map(|ids| [ids.0, ids.1.unwrap_or_else(|| u64::MAX..u64::MAX)])
            //     .map(|ids| {
            //     [
            //         vec![ids.0],
            //         ids.1
            //             .map(|ids| check_ranges(ids, transitions))
            //             .unwrap_or_else(|| vec![u64::MAX..u64::MAX]), // maybe it should return an iterator
            //     ]
            // })
        })
        // .flatten()
        .filter(|(_, ids)| ids.is_empty().not())
        .collect::<Vec<(MatchedOn, Range<u64>)>>();
    // .flatten()
    // .filter(|id| id.start != u64::MAX)
    // .map(|ids| RangeCmp::new(ids))
    // .collect::<HashSet<RangeCmp<u64>>>()
    // .into_iter()
    // .map(|range| range.into())
    // .collect() // super bad but there is no other way :(

    remainders
        .eq(&1)
        .then(|| Some(ranges.push((MatchedOn::None, remainder?))));
    ranges
        .iter()
        .find(|(matched, _)| *matched == MatchedOn::Full)
        .map(|(_, full)| vec![full.clone()])
        .unwrap_or_else(|| ranges.into_iter().map(|(_, part)| part).collect())
}

// so big brain idea here in transition_range also offset the original range like in transition since that's what I forgot

fn debug_print_vec_ids(current_ids: &[Range<u64>]) {
    println!(
        "{}",
        current_ids
            .iter()
            .map(|ids| ids
                .clone()
                .into_iter()
                .map(|id| id.to_string() + ",")
                .collect::<String>()
                + "|")
            .collect::<String>()
    );
}

fn debug_print_ids(source: &Range<u64>, ids: &Range<u64>, msg: &str) {
    print!(
        "{:?}:{}{}|",
        source.clone(),
        ids.clone()
            .into_iter()
            .map(|id| id.to_string() + ",")
            .collect::<String>(),
        msg
    );
}
