use crate::Day;

pub struct Day6;

impl Day for Day6 {
    fn example_input(&self) -> (&'static str, &'static str) {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
        (input, input)
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("288", "71503")
    }

    fn part_1(&mut self, input: String) -> String {
        let mut lines = input.split('\n');
        let times = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty());
        let distances = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty());
        let records = times.zip(distances).map(|(time, distance)| {
            (
                time.parse::<u64>().unwrap(),
                distance.parse::<u64>().unwrap(),
            )
        });

        records
            .into_iter()
            .map(|(time, distance)| {
                (1..time)
                    .map(|i| (time - i) * i)
                    .filter(|new_distance| distance < *new_distance)
                    .count() as u64
            })
            .product::<u64>()
            .to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        let mut lines = input.split('\n');
        let time = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .replace(' ', "")
            .trim()
            .parse::<u64>()
            .unwrap();
        let distance = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .replace(' ', "")
            .trim()
            .parse::<u64>()
            .unwrap();

        (1..time)
            .map(|i| (time - i) * i)
            .filter(|new_distance| distance < *new_distance)
            .count()
            .to_string()
    }
}
