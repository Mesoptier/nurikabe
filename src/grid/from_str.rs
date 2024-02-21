use nom::branch::alt;
use std::str::FromStr;

use nom::character::complete::{char, digit1, line_ending};
use nom::combinator::{map, value};
use nom::multi::{many1, separated_list1};
use nom::IResult;

use crate::{Coord, Grid, State};

impl FromStr for Grid {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, ()> {
        let (_, grid) = parse_grid(input).map_err(|_| ())?;

        let num_cols = grid[0].len();
        let num_rows = grid.len();
        let mut givens = vec![];

        let mut mark_as_white = vec![];
        let mut mark_as_black = vec![];

        for (row_idx, row) in grid.iter().enumerate() {
            if row.len() != num_cols {
                return Err(());
            }

            for (col_idx, &state) in row.iter().enumerate() {
                let coord = Coord::new(row_idx, col_idx);

                match state {
                    State::Numbered(number) => givens.push((coord, number)),
                    State::White => mark_as_white.push(coord),
                    State::Black => mark_as_black.push(coord),
                    State::Unknown => {}
                }
            }
        }

        let mut grid = Grid::new(num_rows, num_cols, givens);

        for coord in mark_as_white {
            grid.mark_cell(coord, State::White);
        }
        for coord in mark_as_black {
            grid.mark_cell(coord, State::Black);
        }

        Ok(grid)
    }
}

impl Grid {
    pub fn to_input_string(&self) -> String {
        let mut result = String::new();

        for row in self.cells.chunks(self.num_cols) {
            for cell in row {
                match cell.state {
                    State::Unknown => result.push('.'),
                    State::White => result.push('W'),
                    State::Black => result.push('B'),
                    State::Numbered(n) => result.push_str(n.to_string().as_str()),
                };
            }
            result.push('\n');
        }

        result
    }
}

fn parse_grid(input: &str) -> IResult<&str, Vec<Vec<State>>> {
    separated_list1(line_ending, parse_row)(input)
}

fn parse_row(input: &str) -> IResult<&str, Vec<State>> {
    many1(parse_cell)(input)
}

fn parse_cell(input: &str) -> IResult<&str, State> {
    alt((
        value(State::Unknown, char('.')),
        value(State::White, char('W')),
        value(State::Black, char('B')),
        map(digit1, |s: &str| State::Numbered(s.parse().unwrap())),
    ))(input)
}
