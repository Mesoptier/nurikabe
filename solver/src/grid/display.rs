use std::fmt::{Display, Formatter};

use colored::Colorize;

use crate::{Coord, Grid, State};

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        GridDiff {
            grid: self,
            prev_states: &[],
        }
        .fmt(f)
    }
}

impl Grid {
    pub(crate) fn diff<'a>(&'a self, prev_states: &'a [Option<State>]) -> GridDiff<'a> {
        assert_eq!(prev_states.len(), self.cells.len());
        GridDiff {
            grid: self,
            prev_states,
        }
    }
}

pub(crate) struct GridDiff<'a> {
    grid: &'a Grid,
    prev_states: &'a [Option<State>],
}

impl<'a> Display for GridDiff<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.grid.num_rows {
            for col in 0..self.grid.num_cols {
                let state = self.grid.cell(Coord::new(row, col)).state;
                let prev_state = self
                    .prev_states
                    .get(self.grid.coord_to_index(Coord::new(row, col)))
                    .copied();

                let string = match (state, prev_state) {
                    (Some(State::Numbered(number)), _) => {
                        format!("{:^3}", number.to_string().black())
                    }
                    (state, Some(prev_state)) if state != prev_state => {
                        format!("{:^3}", "*".bright_red())
                    }
                    _ => format!("{:3}", ""),
                };

                match state {
                    None => write!(f, "{}", string.on_white())?,
                    Some(State::White | State::Numbered(_)) => {
                        write!(f, "{}", string.on_bright_white())?
                    }
                    Some(State::Black) => write!(f, "{}", string.on_black())?,
                };
            }

            writeln!(f, " ")?;
        }

        Ok(())
    }
}
