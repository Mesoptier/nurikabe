use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, value},
    multi::{many1, separated_list1},
    IResult,
};

use crate::{Grid, State};

impl FromStr for Grid {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, ()> {
        let (_, grid) = parse_grid(input).map_err(|_| ())?;

        let width = grid[0].len();
        let height = grid.len();
        let mut givens = vec![];

        let mut mark_as_white = vec![];
        let mut mark_as_black = vec![];

        for (y, row) in grid.iter().enumerate() {
            if row.len() != width {
                return Err(());
            }

            for (x, &state) in row.iter().enumerate() {
                match state {
                    State::Numbered(number) => givens.push(((x, y), number)),
                    State::White => mark_as_white.push((x, y)),
                    State::Black => mark_as_black.push((x, y)),
                    State::Unknown => {}
                }
            }
        }

        let mut grid = Grid::new(width, height, givens);

        for coord in mark_as_white {
            grid.mark_cell(coord, State::White);
        }
        for coord in mark_as_black {
            grid.mark_cell(coord, State::Black);
        }

        Ok(grid)
    }
}

fn parse_grid(input: &str) -> IResult<&str, Vec<Vec<State>>> {
    separated_list1(tag("/"), parse_row)(input)
}

fn parse_row(input: &str) -> IResult<&str, Vec<State>> {
    many1(parse_cell)(input)
}

fn parse_cell(input: &str) -> IResult<&str, State> {
    alt((
        // TODO: Come up with better chars for these states? Especially since
        //  a dot is often used to indicate white cells.
        value(State::Unknown, tag(".")),
        value(State::White, tag(" ")),
        value(State::Black, tag("#")),
        map(digit1, |s: &str| State::Numbered(s.parse().unwrap())),
    ))(input)
}
