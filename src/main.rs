use reqwest::{blocking, cookie::Jar, Url};
use std::fs;

mod day1;

use crate::day1::Day1;

pub trait Day {
    fn example_input(&self) -> (&'static str, &'static str);
    fn example_solution(&self) -> (&'static str, &'static str);
    fn part_1(&mut self, input: String) -> String;
    fn part_2(&mut self, input: String) -> String;
}

const YEAR: usize = 2023;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let (Some(day), Some(part)) = (
        args.get(1).map(|i| i.parse::<usize>().ok()).flatten(),
        args.get(2).map(|i| i.parse::<usize>().ok()).flatten(),
    ) else {
        return eprintln!("app < day: usize > < part: usize >");
    };

    println!("solution for day {day} part {part}");

    let jar = Jar::default();
    jar.add_cookie_str(
        &format!("session={}", fs::read_to_string("session.txt").unwrap()),
        &format!("https://adventofcode.com/{YEAR}/day/{day}/input")
            .parse::<Url>()
            .unwrap(),
    );

    let client = blocking::Client::builder()
        .cookie_store(true)
        .cookie_provider(std::sync::Arc::new(jar))
        .build()
        .expect("couldn't build request client");

    let input = client
        .get(format!("https://adventofcode.com/{YEAR}/day/{day}/input"))
        .send()
        .expect("couldn't get advent of code input for this day :(")
        .text()
        .expect("couldn't get the request as string");

    let input = fs::read_to_string("input.txt").unwrap(); //temp

    let mut days: Vec<Box<dyn Day>> = vec![Box::new(DummyDay), Box::new(Day1)];
    let day = days.get_mut(day).expect("day not implemented");

    assert_eq!(
        (
            day.part_1(day.example_input().0.to_string()).as_str(),
            day.part_2(day.example_input().1.to_string()).as_str(),
        ),
        day.example_solution()
    );

    let result = get_result(day.as_mut(), part, input);

    println!("result : {result}");
}

fn get_result(day: &mut dyn Day, part: usize, input: String) -> String {
    match part {
        1 | 0 => day.part_1(input),
        2 => day.part_2(input),
        _ => panic!("invalid part!"),
    }
}

struct DummyDay;

impl Day for DummyDay {
    fn part_1(&mut self, input: String) -> String {
        input.parse::<i32>().unwrap().to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        input.parse::<i32>().unwrap().to_string()
    }

    fn example_input(&self) -> (&'static str, &'static str) {
        ("0", "0")
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("0", "0")
    }
}
