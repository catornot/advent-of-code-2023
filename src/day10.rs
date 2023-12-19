#![allow(unused)]

use itertools::{Itertools, Position};

use crate::Day;

type Pos = [usize; 2];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum SpecialPoint {
    Start,
    Ground,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Enclosed {
    Inside,
    Loop,
    Outside,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    next: Pos,
    from: Pos,
    dir_marker: char,
    special: Option<SpecialPoint>,
    distance: Option<usize>,
    enclosed_state: Enclosed,
}

pub struct Day10;

impl Day for Day10 {
    fn example_input(&self) -> (&'static str, &'static str) {
        (
            r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#,
            //             r#"FF7FSF7F7F7F7F7F---7
            // L|LJ||||||||||||F--J
            // FL-7LJLJ||||||LJL-77
            // F--JF--7||LJLJ7F7FJ-
            // L---JF-JLJ.||-FJLJJ7
            // |F|F-JF---7F7-L7L|7|
            // |FFJF7L7F-JF7|JL---7
            // 7-L-JL7||F7|L7F-7F7|
            // L.L7LFJ|||||FJL7||LJ
            // L7JLJL-JLJLJL--JLJ.L"#,
            r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#,
        )
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("8", "8")
    }

    fn part_1(&mut self, input: String) -> String {
        parse_and_find_distances(&input)
            .iter()
            .map(|points| points.iter())
            .flatten()
            .fold(0usize, |dis, point| {
                if matches!(point.special, None)
                    && point.distance.is_some()
                    && dis < point.distance.unwrap()
                {
                    point.distance.unwrap()
                } else {
                    dis
                }
            })
            .to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        let mut grid = parse_and_find_distances(&input);
        let grid_lenght = grid.len();
        let line_lenght = input.split_once('\n').unwrap().0.len();

        // outside/yes/no pass
        // let positions = (0..line_lenght)
        //     .into_iter()
        //     .map(|x| (0..grid_lenght).into_iter().map(move |y| [x, y]))
        //     .flatten()
        //     .collect::<Vec<[usize; 2]>>();

        let mut is_inside = false;
        for (end_or_not, pos) in (0..line_lenght)
            .into_iter()
            .map(|x| {
                (0..grid_lenght)
                    .into_iter()
                    .map(move |y| [x, y])
                    .with_position()
            })
            .flatten()
        {
            if matches!(end_or_not, Position::First) {
                is_inside = false;
            }

            let current_point = &mut grid[pos[1]][pos[0]];

            if current_point.distance.is_some() {
                if !matches!(current_point.dir_marker, '|') {
                    is_inside = !is_inside;
                }
                current_point.enclosed_state = Enclosed::Loop;
            } else if !matches!(current_point.enclosed_state, Enclosed::Outside) && is_inside {
                current_point.enclosed_state = Enclosed::Inside
            } else {
                current_point.enclosed_state = Enclosed::Outside
            }
        }

        // for pos in (0..grid_lenght)
        //     .into_iter()
        //     .map(|y| (0..line_lenght).into_iter().map(move |x| [x, y]))
        //     .flatten()
        // {
        //     grid[pos[1]][pos[0]].enclosed_state = decide_state(pos, &grid);

        //     let current_point = &mut grid[pos[1]][pos[0]];

        //     if current_point.distance.is_some() {
        //         if !matches!(current_point.dir_marker, '-') {
        //             is_inside = !is_inside;
        //         }
        //         current_point.enclosed_state = Enclosed::Loop;
        //     } else if !matches!(current_point.enclosed_state, Enclosed::Outside) && is_inside {
        //         current_point.enclosed_state = Enclosed::Inside
        //     } else {
        //         current_point.enclosed_state = Enclosed::Outside
        //     }
        // }

        print_grid(&grid);

        grid.iter()
            .map(|points| points.iter())
            .flatten()
            .filter(|point| matches!(point.enclosed_state, Enclosed::Inside))
            .count()
            .to_string()
    }
}

fn decide_state(self_pos: Pos, grid: &[Vec<Point>]) -> Enclosed {
    let grid_lenght = grid.len();
    let line_lenght = grid[0].len();
    let [x, y] = self_pos;
    let self_point = &grid[y][x];
    if (self_point.distance.is_some() || matches!(self_point.special, Some(SpecialPoint::Start)))
        && !matches!(self_point.special, Some(SpecialPoint::Ground))
    {
        return Enclosed::Loop;
    }

    // let neighbors = (-1..=1_isize)
    //     .into_iter()
    //     .map(|x| (-1..=1).into_iter().map(move |y| [x, y]))
    // .map(|[ox, oy]| Some([ox + x as isize, oy + y as isize]))
    // .flatten()
    let neighbors = [
        [self_pos[0].checked_sub(1), Some(self_pos[1])],
        [self_pos[0].checked_add(1), Some(self_pos[1])],
        [Some(self_pos[0]), self_pos[1].checked_sub(1)],
        [Some(self_pos[0]), self_pos[1].checked_add(1)],
    ]
    .map(|pos| {
        Some(
            grid.get(usize::try_from(pos[1]?).ok()?)?
                .get(usize::try_from(pos[0]?).ok()?)?,
        )
    });
    // .collect::<Vec<Option<&Point>>>();

    if neighbors
        .iter()
        .map(|point| point.map(|_| ()))
        .collect::<Option<Vec<()>>>()
        .is_none()
        || neighbors
            .iter()
            .filter_map(|point| point.as_ref())
            .find(|point| matches!(point.enclosed_state, Enclosed::Outside))
            .is_some()
    {
        return Enclosed::Outside;
    }

    // if line_clear_in_range(
    //     (-1..(x as isize))
    //         .into_iter()
    //         .rev()
    //         .map(move |ix| [ix, y as isize]),
    //     grid,
    // ) || line_clear_in_range(
    //     ((x as isize + 1)..(line_lenght as isize))
    //         .into_iter()
    //         .map(move |ix| [ix, y as isize]),
    //     grid,
    // ) || line_clear_in_range(
    //     (-1..(y as isize))
    //         .into_iter()
    //         .rev()
    //         .map(move |iy| [x as isize, iy]),
    //     grid,
    // ) || line_clear_in_range(
    //     ((y as isize + 1)..(grid_lenght as isize))
    //         .into_iter()
    //         .map(move |iy| [x as isize, iy]),
    //     grid,
    // ) {
    //     return Enclosed::Outside;
    // }

    Enclosed::Inside
}

fn line_clear_in_range(range: impl Iterator<Item = [isize; 2]>, grid: &[Vec<Point>]) -> bool {
    range
        .filter_map(|pos| {
            Some(
                grid.get(usize::try_from(pos[1]).ok()?)?
                    .get(usize::try_from(pos[0]).ok()?)?,
            )
        })
        .find(|point| !matches!(point.dir_marker, '.' | '-' | '|') || point.distance.is_none())
        .is_none()
}

fn parse_and_find_distances(input: &str) -> Vec<Vec<Point>> {
    let mut grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| (c, [x, y]))
                .map(|(c, pos)| point_from_dir(c, pos))
                .collect()
        })
        .collect::<Vec<Vec<Point>>>();
    let (starting_points, start) = {
        let start_point = grid
            .iter()
            .enumerate()
            .map(|(y, points)| {
                points
                    .iter()
                    .enumerate()
                    .map(move |(x, point)| (point, [x, y]))
            })
            .flatten()
            .find(|(point, _)| matches!(point.special, Some(SpecialPoint::Start)))
            .map(|(_, pos)| pos)
            .expect("there should be a starting point");

        (
            [
                [start_point[0].saturating_sub(1), start_point[1]],
                [start_point[0].saturating_add(1), start_point[1]],
                [start_point[0], start_point[1].saturating_sub(1)],
                [start_point[0], start_point[1].saturating_add(1)],
            ],
            start_point,
        )
    };

    for (mut current_pos, mut prev) in starting_points
        .iter()
        .map(|[x, y]| ([*x, *y], start))
        .collect::<Vec<(Pos, Pos)>>()
        .into_iter()
    {
        let mut steps = 1;
        while !matches!(
            grid[current_pos[1]][current_pos[0]].special,
            Some(SpecialPoint::Start)
        ) {
            let current_point = &mut grid[current_pos[1]][current_pos[0]];

            let new_point = match try_next_pos(current_point, prev) {
                Some(pos) => pos,
                None => break,
            };

            if current_point.distance.is_none() || current_point.distance.unwrap() >= steps {
                current_point.distance = Some(steps);
            }

            prev = current_pos;
            current_pos = new_point;

            steps += 1;
        }
    }

    grid
}

fn print_grid(grid: &[Vec<Point>]) {
    grid.iter()
        .map(|points| points.iter().with_position())
        .flatten()
        .for_each(|(position, point)| match (position, point.enclosed_state) {
            (Position::Last | Position::Only, Enclosed::Inside) => println!("I"),
            (Position::Last | Position::Only, Enclosed::Outside) => println!("O"),
            (Position::Last | Position::Only, Enclosed::Loop) => println!("{}", point.dir_marker),
            (_, Enclosed::Inside) => print!("I"),
            (_, Enclosed::Outside) => print!("O"),
            (_, Enclosed::Loop) => print!("{}", point.dir_marker),
        })
}

fn try_next_pos(point: &Point, from: Pos) -> Option<Pos> {
    if point.special.is_some() {
        return None;
    }

    if point.from == from {
        return Some(point.next);
    }

    if point.next == from {
        return Some(point.from);
    }

    None
}

fn point_from_dir(dir: char, pos: Pos) -> Point {
    match dir {
        '|' => Point {
            next: [pos[0], pos[1].saturating_sub(1)],
            from: [pos[0], pos[1].saturating_add(1)],
            dir_marker: dir,
            special: None,
            distance: None,
            enclosed_state: Enclosed::Loop,
        },
        '-' => Point {
            next: [pos[0].saturating_sub(1), pos[1]],
            from: [pos[0].saturating_add(1), pos[1]],
            dir_marker: dir,
            special: None,
            distance: None,
            enclosed_state: Enclosed::Loop,
        },
        'L' => Point {
            next: [pos[0], pos[1].saturating_sub(1)],
            from: [pos[0].saturating_add(1), pos[1]],
            dir_marker: dir,
            special: None,
            distance: None,
            enclosed_state: Enclosed::Loop,
        },
        'J' => Point {
            next: [pos[0], pos[1].saturating_sub(1)],
            from: [pos[0].saturating_sub(1), pos[1]],
            dir_marker: dir,
            special: None,
            distance: None,
            enclosed_state: Enclosed::Loop,
        },
        '7' => Point {
            next: [pos[0].saturating_sub(1), pos[1]],
            from: [pos[0], pos[1].saturating_add(1)],
            dir_marker: dir,
            special: None,
            distance: None,
            enclosed_state: Enclosed::Loop,
        },
        'F' => Point {
            next: [pos[0].saturating_add(1), pos[1]],
            from: [pos[0], pos[1].saturating_add(1)],
            dir_marker: dir,
            special: None,
            distance: None,
            enclosed_state: Enclosed::Loop,
        },
        'S' => Point {
            next: [usize::MAX, usize::MAX],
            from: [usize::MAX, usize::MAX],
            dir_marker: dir,
            special: Some(SpecialPoint::Start),
            distance: None,
            enclosed_state: Enclosed::Loop,
        },
        '.' => Point {
            next: [usize::MAX, usize::MAX],
            from: [usize::MAX, usize::MAX],
            dir_marker: dir,
            special: Some(SpecialPoint::Ground),
            distance: None,
            enclosed_state: Enclosed::Loop,
        },
        _ => panic!("invalid dir found!"),
    }
}
