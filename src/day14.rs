use crate::Day;

enum RockType {
    Round,
    Cube,
    None
}

pub struct Day14;

impl Day for Day14 {
    fn example_input(&self) -> (&'static str, &'static str) {
        (
            r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#,
            "",
        )
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("136", "")
    }

    fn part_1(&mut self, input: String) -> String {
        let beam = input.lines().map(|line| line.chars() )
    }

    fn part_2(&mut self, input: String) -> String {
        _ = input;
        "".to_string()
    }
}
