use crate::Day;

pub struct Day11;

impl Day for Day11 {
    fn example_input(&self) -> (&'static str, &'static str) {
        (
            r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#,
            "",
        )
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("374", "")
    }

    fn part_1(&mut self, input: String) -> String {
        let mut galatic_map = input
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect::<Vec<Vec<bool>>>();

        // todo implement doubling yk

        let len = galatic_map.len();
        (0..(galatic_map[0].len()))
            .into_iter()
            .map(|x| (0..len).into_iter().map(move |y| [x, y]))
            .flatten()
            .map(|[x, y]| {
                if !galatic_map[y][x] {
                    return 0;
                }

                galatic_map.iter()

                todo!()
            })
            .sum::<u64>();
        todo!()
    }

    fn part_2(&mut self, input: String) -> String {
        _ = input;
        "".to_string()
    }
}
