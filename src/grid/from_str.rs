use std::str::FromStr;

use nom::branch::alt;
use nom::character::complete::{char, digit1, line_ending, not_line_ending};
use nom::combinator::{map, value};
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::delimited;
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
                    Some(State::Numbered(number)) => givens.push((coord, number)),
                    Some(State::White) => mark_as_white.push(coord),
                    Some(State::Black) => mark_as_black.push(coord),
                    None => {}
                }
            }
        }

        let mut grid = Grid::new(num_rows, num_cols, givens);

        for coord in mark_as_white {
            grid.mark_cell(coord, State::White).unwrap();
        }
        for coord in mark_as_black {
            grid.mark_cell(coord, State::Black).unwrap();
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
                    None => result.push('.'),
                    Some(State::White) => result.push('W'),
                    Some(State::Black) => result.push('B'),
                    Some(State::Numbered(n)) => result.push_str(n.to_string().as_str()),
                };
            }
            result.push('\n');
        }

        result.pop(); // Remove the trailing newline
        result
    }
}

fn parse_grid(input: &str) -> IResult<&str, Vec<Vec<Option<State>>>> {
    let (input, _) = parse_header(input)?;
    separated_list1(line_ending, parse_row)(input)
}

/// Parse the header comments (i.e. lines starting with `#`).
fn parse_header(input: &str) -> IResult<&str, ()> {
    let (input, _) = many0(delimited(char('#'), not_line_ending, line_ending))(input)?;
    Ok((input, ()))
}

fn parse_row(input: &str) -> IResult<&str, Vec<Option<State>>> {
    many1(parse_cell)(input)
}

fn parse_cell(input: &str) -> IResult<&str, Option<State>> {
    alt((
        value(None, char('.')),
        value(Some(State::White), char('W')),
        value(Some(State::Black), char('B')),
        map(digit1, |s: &str| Some(State::Numbered(s.parse().unwrap()))),
    ))(input)
}
