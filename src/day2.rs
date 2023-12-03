use crate::Day;

#[derive(Debug, Clone, Copy)]
enum Cube {
    Red(usize),
    Blue(usize),
    Green(usize),
}

impl Cube {
    fn extract(self) -> usize {
        match self {
            Cube::Red(a) => a,
            Cube::Blue(a) => a,
            Cube::Green(a) => a,
        }
    }
}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Cube::Red(_), Cube::Red(_)) => true,
            (Cube::Green(_), Cube::Green(_)) => true,
            (Cube::Blue(_), Cube::Blue(_)) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct Game {
    sets: Vec<Vec<Cube>>,
}

#[derive(Debug, Default)]
struct MinUse {
    green: usize,
    blue: usize,
    red: usize,
}

pub struct Day2;

impl Day for Day2 {
    fn example_input(&self) -> (&'static str, &'static str) {
        (
            r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#,
            r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#,
        )
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("8", "2286")
    }

    fn part_1(&mut self, input: String) -> String {
        const REDS: usize = 12;
        const GREENS: usize = 13;
        const BLUES: usize = 14;

        let games = parse_games(input);

        games
            .into_iter()
            .enumerate()
            .map(|(i, game)| (i + 1, game))
            .filter(|(_, game)| {
                game.sets
                    .iter()
                    .map(|set| {
                        set.iter()
                            .filter(|cube| match cube {
                                Cube::Red(amount) if *amount <= REDS => true,
                                Cube::Blue(amount) if *amount <= BLUES => true,
                                Cube::Green(amount) if *amount <= GREENS => true,
                                _ => false,
                            })
                            .count()
                    })
                    .sum::<usize>()
                    == game
                        .sets
                        .iter()
                        .map(|sets| sets.iter().count())
                        .sum::<usize>()
            })
            .map(|(index, _)| (index))
            .sum::<usize>()
            .to_string()
    }
    fn part_2(&mut self, input: String) -> String {
        let games = parse_games(input);

        let min_requirements = games
            .into_iter()
            .map(|game| {
                let mut min_requirement = MinUse::default();

                game.sets.iter().for_each(|set| {
                    set.iter().for_each(|cube| match cube {
                        Cube::Red(amount) if *amount > min_requirement.red => {
                            min_requirement.red = *amount
                        }
                        Cube::Blue(amount) if *amount > min_requirement.blue => {
                            min_requirement.blue = *amount
                        }
                        Cube::Green(amount) if *amount > min_requirement.green => {
                            min_requirement.green = *amount
                        }
                        _ => {}
                    })
                });

                min_requirement
            })
            .collect::<Vec<MinUse>>();

        min_requirements
            .into_iter()
            .map(|min| min.green * min.blue * min.red)
            .sum::<usize>()
            .to_string()
    }
}

fn parse_games(input: String) -> Vec<Game> {
    input
        .trim_end()
        .trim()
        .split('\n')
        .map(|line| parse_game(line.to_string()))
        .collect::<Vec<Game>>()
}

fn parse_game(mut game: String) -> Game {
    game = game.replace("Game ", "");
    let (_, sets) = game.split_once(':').expect("game as to have id split");

    let sets = sets
        .split(';')
        .map(|set| {
            sum_cubes(
                set.split(',')
                    .map(|cubes| {
                        {
                            parse_cubes(
                                cubes
                                    .trim()
                                    .split_once(' ')
                                    .expect("cube has to have an amount and cube split"),
                            )
                        }
                    })
                    .collect::<Vec<Cube>>(),
            )
        })
        .collect::<Vec<Vec<Cube>>>();

    Game { sets }
}

fn parse_cubes(cubes: (&str, &str)) -> Cube {
    text_to_cube(
        cubes.1,
        cubes
            .0
            .parse()
            .expect("cubes should have some amount of them"),
    )
    .expect("can't have invalid cube names")
}

fn text_to_cube(txt: &str, amount: usize) -> Option<Cube> {
    match txt {
        "red" => Some(Cube::Red(amount)),
        "blue" => Some(Cube::Blue(amount)),
        "green" => Some(Cube::Green(amount)),
        _ => None,
    }
}

fn sum_cubes(mut cubes: Vec<Cube>) -> Vec<Cube> {
    let reds = sum_alike(&cubes, Cube::Red(0));
    let greens = sum_alike(&cubes, Cube::Green(0));
    let blues = sum_alike(&cubes, Cube::Blue(0));
    cubes.clear();

    cubes.push(Cube::Red(reds));
    cubes.push(Cube::Green(greens));
    cubes.push(Cube::Blue(blues));

    cubes
}

fn sum_alike(cubes: &[Cube], cube_cmp: Cube) -> usize {
    cubes
        .iter()
        .filter(|cube| **cube == cube_cmp)
        .map(|cube| cube.extract())
        .sum()
}
