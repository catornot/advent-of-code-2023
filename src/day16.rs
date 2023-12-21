use std::{collections::HashMap, convert::Infallible};

use crate::Day;

#[derive(Debug, Clone)]
enum TileType {
    ForwardMirror,
    BackwardMirror,
    VerticalPipe,
    HorizontalPipe,
    Empty,
}

impl TileType {
    fn passthrough(&self, dir: Direction) -> (Direction, Option<Direction>) {
        match self {
            TileType::ForwardMirror => match dir {
                Direction::Up => (Direction::Right, None),
                Direction::Down => (Direction::Left, None),
                Direction::Right => (Direction::Up, None),
                Direction::Left => (Direction::Down, None),
            },
            TileType::BackwardMirror => match dir {
                Direction::Up => (Direction::Left, None),
                Direction::Down => (Direction::Right, None),
                Direction::Right => (Direction::Down, None),
                Direction::Left => (Direction::Up, None),
            },
            TileType::VerticalPipe => match dir {
                Direction::Right | Direction::Left => (Direction::Down, Some(Direction::Up)),
                Direction::Up => (Direction::Up, None),
                Direction::Down => (Direction::Down, None),
            },
            TileType::HorizontalPipe => match dir {
                Direction::Up | Direction::Down => (Direction::Left, Some(Direction::Right)),
                Direction::Right => (Direction::Right, None),
                Direction::Left => (Direction::Left, None),
            },
            TileType::Empty => (dir, None),
        }
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn next_pos(&self, pos: [usize; 2]) -> Option<[usize; 2]> {
        match self {
            Direction::Up => Some([pos[0], pos[1].checked_sub(1)?]),
            Direction::Down => Some([pos[0], pos[1].checked_add(1)?]),
            Direction::Right => Some([pos[0].checked_add(1)?, pos[1]]),
            Direction::Left => Some([pos[0].checked_sub(1)?, pos[1]]),
        }
    }
}

#[derive(Debug, Clone)]
struct Tile {
    energized: bool,
    ty: TileType,
}

#[derive(Debug, Clone)]
struct Beam {
    dir: Direction,
    pos: [usize; 2],
    visited: HashMap<[usize; 2], usize>,
}

pub struct Day16;

impl Day for Day16 {
    fn example_input(&self) -> (&'static str, &'static str) {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        (input, input)
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("46", "51")
    }

    fn part_1(&mut self, input: String) -> String {
        let mut contraption = input
            .lines()
            .map(|line| line.chars().map(to_tile).collect::<Vec<Tile>>())
            .collect::<Vec<Vec<Tile>>>();

        let beam = Beam {
            dir: contraption[0][0].ty.passthrough(Direction::Right).0,
            pos: [0, 0],
            visited: HashMap::new(),
        };
        contraption[beam.pos[1]][beam.pos[0]].energized = true;

        simulate_beam(&mut contraption, vec![beam]);

        contraption
            .into_iter()
            .map(|line| line.into_iter().filter(|tile| tile.energized).count())
            .sum::<usize>()
            .to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        let contraption = input
            .lines()
            .map(|line| line.chars().map(to_tile).collect::<Vec<Tile>>())
            .collect::<Vec<Vec<Tile>>>();

        let len_x = contraption[0].len();
        let len_y = contraption.len();

        (0..len_x)
            .map(|x| [x, 0])
            .zip((0..len_x).map(|x| [x, len_y - 1]))
            .zip(
                (0..len_y)
                    .map(|y| [0, y])
                    .zip((0..len_y).map(|y| [len_x - 1, y])),
            )
            .map(|((pos1, pos2), (pos3, pos4))| [pos1, pos2, pos4, pos3])
            .flatten()
            .map(|start_pos| {
                [
                    Direction::Up,
                    Direction::Right,
                    Direction::Left,
                    Direction::Down,
                ]
                .into_iter()
                .map(move |dir| (start_pos, dir))
            })
            .flatten()
            .map(|(start_pos, dir)| {
                let mut contraption = contraption.clone();

                let (dir, other_dir) = contraption[start_pos[1]][start_pos[0]].ty.passthrough(dir);

                let mut beams = vec![Beam {
                    dir,
                    pos: start_pos,
                    visited: HashMap::new(),
                }];
                contraption[beams[0].pos[1]][beams[0].pos[0]].energized = true;

                if let Some(dir) = other_dir {
                    beams.push(Beam {
                        dir,
                        pos: start_pos,
                        visited: HashMap::new(),
                    })
                }

                simulate_beam(&mut contraption, beams);

                contraption
                    .iter()
                    .cloned()
                    .map(|line| line.into_iter().filter(|tile| tile.energized).count())
                    .sum::<usize>()
            })
            .fold(0, |prev, current| {
                prev.cmp(&current)
                    .is_lt()
                    .then(|| current)
                    .unwrap_or_else(|| prev)
            })
            .to_string()
    }
}

fn simulate_beam(contraption: &mut [Vec<Tile>], mut beams: Vec<Beam>) -> Option<Infallible> {
    contraption
        .get_mut(beams[0].pos[1])?
        .get_mut(beams[0].pos[0])?
        .energized = true;

    let mut new_beams = vec![];
    while beams.len() != 0 {
        beams
            .iter_mut()
            .enumerate()
            .filter_map(|(i, beam)| {
                tick_beam(contraption, beam)
                    .map(|beam| {
                        beam.map(|beam| new_beams.push(beam))
                            .map(|_| None)
                            .flatten()
                    })
                    .unwrap_or_else(move || Some(i))
            })
            .find(|_| true)
            .map(|i| beams.remove(i));

        beams.extend(new_beams.drain(..));
    }
    None
}

fn tick_beam(contraption: &mut [Vec<Tile>], beam: &mut Beam) -> Option<Option<Beam>> {
    if let Some(visited) = beam.visited.get_mut(&beam.pos) {
        *visited += 1;
        if *visited > 10 {
            return None;
        }
    } else {
        beam.visited.insert(beam.pos, 1);
    }

    if let Some(next_pos) = beam.dir.next_pos(beam.pos) {
        beam.pos = next_pos.clone();

        let tile = contraption.get_mut(next_pos[1])?.get_mut(next_pos[0])?;

        let (next_dir, split_dir) = tile.ty.passthrough(beam.dir.clone());
        beam.dir = next_dir;

        if let (Some(dir), false) = (split_dir, tile.energized) {
            tile.energized = true;
            Some(Some(Beam {
                dir,
                pos: next_pos,
                visited: HashMap::new(),
            }))
        } else {
            tile.energized = true;
            Some(None)
        }
    } else {
        None
    }
}

fn to_tile(c: char) -> Tile {
    match c {
        '.' => Tile {
            energized: false,
            ty: TileType::Empty,
        },
        '|' => Tile {
            energized: false,
            ty: TileType::VerticalPipe,
        },
        '-' => Tile {
            energized: false,
            ty: TileType::HorizontalPipe,
        },
        '/' => Tile {
            energized: false,
            ty: TileType::ForwardMirror,
        },
        '\\' => Tile {
            energized: false,
            ty: TileType::BackwardMirror,
        },
        _ => panic!("invalid char"),
    }
}
