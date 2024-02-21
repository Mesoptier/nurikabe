use crate::{Coord, Grid, SolverError};
use std::collections::HashSet;

pub mod avoid_pools;
pub mod complete_islands;
pub mod confinement;
pub mod dual_liberties;
pub mod hypotheticals;
pub mod single_liberties;
pub mod unreachable_cells;

pub type StrategyResult = Result<bool, SolverError>;

pub trait Strategy {
    fn name(&self) -> &str;
    fn apply(&self, grid: &mut Grid) -> StrategyResult;
}

struct MarkSet {
    mark_as_black: HashSet<Coord>,
    mark_as_white: HashSet<Coord>,
}

impl MarkSet {
    fn new() -> Self {
        Self {
            mark_as_black: HashSet::new(),
            mark_as_white: HashSet::new(),
        }
    }

    fn insert(&mut self, coord: Coord, state: State) -> bool {
        match state {
            State::White | State::Numbered(_) => &mut self.mark_as_white,
            State::Black => &mut self.mark_as_black,
        }
        .insert(coord)
    }

    fn apply(self, grid: &mut Grid) -> StrategyResult {
        let result = !self.mark_as_white.is_empty() || !self.mark_as_black.is_empty();
        for coord in self.mark_as_black {
            grid.mark_cell(coord, State::Black)?;
        }
        for coord in self.mark_as_white {
            grid.mark_cell(coord, State::White)?;
        }
        Ok(result)
    }
}

#[cfg(test)]
macro_rules! test_strategy {
    ($name:ident, $strategy:expr, $input:expr, $expected_output:expr) => {
        #[test]
        fn $name() {
            use crate::Grid;
            use std::str::FromStr;
            use $crate::Strategy;

            let mut grid = Grid::from_str($input).unwrap();
            let output = if $strategy.apply(&mut grid).unwrap() {
                Some(grid.to_input_string())
            } else {
                None
            };
            assert_eq!(output, $expected_output.map(str::to_string));
        }
    };
}

use crate::grid::State;
#[cfg(test)]
use test_strategy;
