use std::ops::Not;

use crate::Day;

#[derive(Debug, Clone)]
enum CellType {
    Number(usize, usize), // number, witdh
    NumberRef(usize),
    Symbol(char),
    Nothing,
}

#[derive(Debug)]
struct EngineBoard {
    board: Vec<Vec<CellType>>,
}

#[derive(Debug)]
struct NumberBuilder {
    number: String,
    witdh: usize,
    index: usize,
}

impl NumberBuilder {
    fn clear(&mut self) {
        self.number.clear();
        (self.witdh, self.index) = (0, 0);
    }
}

pub struct Day3;

impl Day for Day3 {
    fn example_input(&self) -> (&'static str, &'static str) {
        (
            r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#,
            r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#,
        )
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("4361", "467835")
    }

    fn part_1(&mut self, input: String) -> String {
        let board = parse_input(&input);

        board
            .board
            .iter()
            .enumerate()
            .map(|(strip_index, strip)| {
                strip
                    .iter()
                    .enumerate()
                    .map(move |(index, cell)| ([strip_index, index], cell))
            })
            .flatten()
            .filter_map(|(pos, cell)| Some((pos, cell_as_number_strict(cell)?)))
            .map(|(pos, cell)| {
                is_near_symbol(cell, pos, &board.board)
                    .then(|| cell.0)
                    .unwrap_or_else(|| 0_usize)
            })
            .sum::<usize>()
            .to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        let board = parse_input(&input);
        let mut checked_symbols: Vec<[usize; 2]> = Vec::with_capacity(500);

        board
            .board
            .iter()
            .enumerate()
            .map(|(strip_index, strip)| {
                strip
                    .iter()
                    .enumerate()
                    .map(move |(index, cell)| ([strip_index, index], cell))
            })
            .flatten()
            .filter_map(|(pos, cell)| Some((pos, cell_as_number_strict(cell)?)))
            .filter_map(|(pos, cell)| {
                let info = is_near_symbol_info(cell, pos, &board.board);
                let symbol = info.0.then(|| info.1)?;
                let symbol_pos = symbol.0.eq(&'*').then(|| symbol.1)?;

                checked_symbols
                    .iter()
                    .find(|internal_pos| **internal_pos == symbol_pos)
                    .map(|_| None)
                    .unwrap_or_else(|| Some(()))?;

                checked_symbols.push(symbol_pos);

                let all_pos = (0..cell.1)
                    .map(|i| [pos[0], pos[1] + i])
                    .collect::<Vec<[usize; 2]>>();

                Some(
                    find_close_number(&all_pos, symbol_pos, &board.board)
                        .map(|info| info.1 * cell.0)
                        .unwrap_or_else(|| 0),
                )
            })
            .sum::<usize>()
            .to_string()
    }
}

fn parse_input(input: &str) -> EngineBoard {
    let mut last_number = NumberBuilder {
        number: String::with_capacity(3),
        witdh: 0_usize,
        index: 0_usize,
    };

    let mut board = EngineBoard {
        board: Vec::with_capacity(200),
    };
    let mut current_index = -1_isize;

    board.board.push(Vec::with_capacity(200));
    for (index, c) in input
        .split('\n')
        .map(|line| line.trim().chars().into_iter().enumerate())
        .flatten()
    {
        if index == 0 {
            try_insert_number(
                &mut board.board[usize::try_from(current_index).ok().unwrap_or_else(|| 0)],
                &mut last_number,
            );
            current_index += 1;
            board.board.push(Vec::with_capacity(200));
        }

        let item = match c {
            c if c == '.' => {
                try_insert_number(&mut board.board[current_index as usize], &mut last_number);
                CellType::Nothing
            }
            c if c.is_ascii_digit() => {
                last_number.number.push(c);
                last_number.witdh += 1;
                last_number.index.eq(&0).then(|| last_number.index = index);
                CellType::NumberRef(last_number.index)
            }
            c => {
                try_insert_number(&mut board.board[current_index as usize], &mut last_number);
                CellType::Symbol(c)
            }
        };

        board.board[current_index as usize].push(item);
    }

    board
}

fn try_insert_number(current_strip: &mut [CellType], last_number: &mut NumberBuilder) {
    last_number.number.is_empty().not().then(|| {
        current_strip[last_number.index] = CellType::Number(
            last_number
                .number
                .parse::<usize>()
                .expect("numbers should parsable"),
            last_number.witdh,
        );

        last_number.clear();
    });
}

fn cell_as_number_strict(cell: &CellType) -> Option<(usize, usize)> {
    match cell {
        CellType::Number(number, width) => Some((*number, *width)),
        _ => None,
    }
}

fn cell_as_number(cell: &CellType, strip: &[CellType]) -> Option<(usize, usize)> {
    match cell {
        CellType::Number(number, _) => Some((*number, 0)),
        CellType::NumberRef(index) => Some(((cell_as_number(&strip[*index], strip))?.0, *index)),
        _ => None,
    }
}

fn cell_as_symbol(cell: &CellType) -> Option<char> {
    match cell {
        CellType::Symbol(symbol) => Some(*symbol),
        _ => None,
    }
}

fn is_near_symbol_info(
    number: (usize, usize),
    pos: [usize; 2],
    board: &[Vec<CellType>],
) -> (bool, (char, [usize; 2])) {
    (-1_isize..=1)
        .into_iter()
        .map(|y| (-1..=(number.1 as isize)).map(move |x| [y, x]))
        .flatten()
        .filter_map(|[y, x]| {
            Some([
                usize::try_from((pos[0] as isize) + y).ok()?,
                usize::try_from((pos[1] as isize) + x).ok()?,
            ])
        })
        .filter_map(|[strip_index, index]| {
            Some((board.get(strip_index)?.get(index)?, [strip_index, index]))
        })
        .find(|(maybe_symbol, _)| matches!(maybe_symbol, CellType::Symbol(_)))
        .map(|(symbol, pos)| (true, (cell_as_symbol(symbol).unwrap(), pos)))
        .unwrap_or_else(|| (false, (' ', [0, 0])))
}

fn is_near_symbol(number: (usize, usize), pos: [usize; 2], board: &[Vec<CellType>]) -> bool {
    is_near_symbol_info(number, pos, board).0
}

fn find_close_number(
    original_number_pos: &[[usize; 2]],
    pos: [usize; 2],
    board: &[Vec<CellType>],
) -> Option<([usize; 2], usize)> {
    (-1_isize..=1)
        .into_iter()
        .map(|y| (-1..=1).map(move |x| [y, x]))
        .flatten()
        .filter_map(|[y, x]| {
            Some([
                usize::try_from((pos[0] as isize) + y).ok()?,
                usize::try_from((pos[1] as isize) + x).ok()?,
            ])
        })
        .filter_map(|[strip_index, index]| {
            Some((board.get(strip_index)?.get(index)?, [strip_index, index]))
        })
        .filter_map(|(maybe_number, [strip_index, mut index])| {
            Some((
                cell_as_number(maybe_number, &board[strip_index]).map(|(value, new_index)| {
                    new_index.eq(&0).not().then_some(|| index = new_index);
                    value
                })?,
                [strip_index, index],
            ))
        })
        .find(|(_, other_pos)| {
            (original_number_pos, other_pos);
            original_number_pos
                .iter()
                .find(|original| **original == *other_pos)
                .is_none()
        })
        .map(|(value, other_pos)| (other_pos, value))
}
