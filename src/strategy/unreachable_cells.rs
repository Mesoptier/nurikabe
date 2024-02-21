use std::collections::HashSet;

use crate::analysis::is_cell_unreachable;
use crate::{Coord, Grid, State};

use super::{Strategy, StrategyResult};

pub struct UnreachableCells;

impl Strategy for UnreachableCells {
    fn name(&self) -> &str {
        "UnreachableCells"
    }

    fn apply(&self, grid: &mut Grid) -> StrategyResult {
        let mut mark_as_black = HashSet::<Coord>::new();

        for col in 0..grid.num_cols {
            for row in 0..grid.num_rows {
                let coord = Coord::new(row, col);
                if is_cell_unreachable(grid, coord, mark_as_black.iter().copied()) {
                    mark_as_black.insert(coord);
                }
            }
        }

        let result = !mark_as_black.is_empty();
        for coord in mark_as_black {
            grid.mark_cell(coord, State::Black)?;
        }
        Ok(result)
    }
}
