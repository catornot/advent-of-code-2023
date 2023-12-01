pub trait Day {
    fn example_input(&self) -> &'static str;
    fn example_solution(&self) -> (&'static str, &'static str);
    fn part_1(&mut self, input: String) -> String;
    fn part_2(&mut self, input: String) -> String;
}

const YEAR: usize = 2023;

mod day1;

use crate::day1::Day1;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let (Some(day), Some(part)) = (
        args.get(1).map(|i| i.parse::<usize>().ok()).flatten(),
        args.get(2).map(|i| i.parse::<usize>().ok()).flatten(),
    ) else {
        return eprintln!("app < day: usize > < part: usize >");
    };

    println!("solution for day {day} part {part}");

    let input = reqwest::blocking::get(format!("https://adventofcode.com/{YEAR}/day/{day}/input"))
        .expect("couldn't get advent of code input for this day :(")
        .text()
        .expect("couldn't get the request as string");

    let mut days: Vec<Box<dyn Day>> = vec![Box::new(DummyDay), Box::new(Day1)];
    let day = days.get_mut(day).expect("day not implemented");

    assert_eq!(
        (
            day.part_1(day.example_input().to_string()).as_str(),
            day.part_2(day.example_input().to_string()).as_str(),
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

    fn example_input(&self) -> &'static str {
        "0"
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("0", "0")
    }
}
