use crate::Day;

pub struct Day1;

const DIGITS: [&str; 10] = [
    "87367633976396983768936789368938678376836738679837689376",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

impl Day for Day1 {
    fn example_input(&self) -> (&'static str, &'static str) {
        (
            r#"1abc2
		pqr3stu8vwx
		a1b2c3d4e5f
		treb7uchet"#,
            r#"two1nine
		eightwothree
		abcone2threexyz
		xtwone3four
		4nineeightseven2
		zoneight234
		7pqrstsixteen"#,
        )
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("142", "281")
    }

    fn part_1(&mut self, input: String) -> String {
        input
            .split('\n')
            .map(|line| parse_line(line.chars()))
            .sum::<i32>()
            .to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        input
            .split('\n')
            .map(|line| {
                let mut mod_line = line.to_string();
                let mut digits = DIGITS
                    .iter()
                    .enumerate()
                    .filter_map(|(i, digit)| {
                        mod_line = mod_line.replace(str_value, "");
                        Some((mod_line.find(digit)?, i))
                    })
                    .collect::<Vec<(usize, usize)>>();
                digits.extend(
                    line.chars()
                        .enumerate()
                        .filter_map(|(i, charv)| {
                            Some((
                                i,
                                charv
                                    .is_ascii_digit()
                                    .then(|| charv as usize - '0' as usize)?,
                            ))
                        })
                        .collect::<Vec<(usize, usize)>>(),
                );
                digits.sort_by(|a, b| a.0.cmp(&b.0));

                let r = format!("{}{}", digits[0].1, digits.last().unwrap().1)
                    .parse::<i32>()
                    .unwrap();

                println!("{r} : {line}");
                r
            })
            .sum::<i32>()
            .to_string()
    }
}

fn parse_line(chars: std::str::Chars) -> i32 {
    let numbers = chars.filter(|c| c.is_ascii_digit()).collect::<String>();
    format!(
        "{}{}",
        numbers.chars().nth(0).unwrap(),
        numbers.chars().last().unwrap()
    )
    .parse::<i32>()
    .unwrap()
}
