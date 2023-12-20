use crate::Day;

enum TileType {
    ForwardMirror,
    BackwardMirror,
    VerticalPipe,
    HorizontalPipe,
    Empty,
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn next_pos(&self, pos: [usize; 2]) -> Option<[usize; 2]> {
        match self {
            Direction::Up => Some([pos[0], pos[1].checked_add(1)?]),
            Direction::Down => Some([pos[0], pos[1].checked_sub(1)?]),
            Direction::Right => Some([pos[0].checked_add(1)?, pos[1]]),
            Direction::Left => Some([pos[0].checked_sub(1)?, pos[1]]),
        }
    }
}

struct Tile {
    energized: bool,
    ty: TileType,
    marker: char,
}

struct Beam {
    dir: Direction,
    pos: [usize; 2],
}

pub struct Day16;

impl Day for Day16 {
    fn example_input(&self) -> (&'static str, &'static str) {
        (
            r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#,
            "",
        )
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("46", "")
    }

    fn part_1(&mut self, input: String) -> String {
        let contraption = input
            .lines()
            .map(|line| line.chars().map(to_tile).collect::<Vec<Tile>>())
            .collect::<Vec<Vec<Tile>>>();

        let x_len = contraption[0].len();
        let y_len = contraption.len();

        todo!()
    }

    fn part_2(&mut self, input: String) -> String {
        _ = input;
        "".to_string()
    }
}

fn simulate_beam(contraption: &mut [Vec<Tile>], beam: Beam, lens: [usize; 2]) {
    // for [x, y] in (0..lens[0])
    //     .map(|x| (0..lens[1]).map(move |y| [x, y]))
    //     .flatten()

    while let Some(next_pos) = beam.dir.next_pos(beam.pos) {}
}

fn to_tile(c: char) -> Tile {
    match c {
        '.' => Tile {
            energized: false,
            ty: TileType::Empty,
            marker: c,
        },
        '|' => Tile {
            energized: false,
            ty: TileType::VerticalPipe,
            marker: c,
        },
        '-' => Tile {
            energized: false,
            ty: TileType::HorizontalPipe,
            marker: c,
        },
        '/' => Tile {
            energized: false,
            ty: TileType::FowardMirror,
            marker: c,
        },
        '\\' => Tile {
            energized: false,
            ty: TileType::BackwardMirror,
            marker: c,
        },
    }
}
