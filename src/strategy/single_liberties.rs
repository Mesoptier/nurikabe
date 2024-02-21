use std::collections::HashSet;

use crate::{Coord, Grid, State};

use super::{Strategy, StrategyResult};

pub struct SingleLiberties;

impl Strategy for SingleLiberties {
    fn name(&self) -> &str {
        "SingleLiberties"
    }

    fn apply(&self, grid: &mut Grid) -> StrategyResult {
        let mut mark_as_black = HashSet::<Coord>::new();
        let mut mark_as_white = HashSet::<Coord>::new();

        for region in grid.regions() {
            let is_region_complete = match region.state {
                State::White => false,
                State::Black => region.len() == grid.total_black_cells,
                State::Numbered(number) => region.len() == number,
            };

            if !is_region_complete && region.unknowns_len() == 1 {
                let unknown_coord = region.unknowns[0];
                match region.state {
                    State::White | State::Numbered(_) => mark_as_white.insert(unknown_coord),
                    State::Black => mark_as_black.insert(unknown_coord),
                };
            }
        }

        let result = !mark_as_black.is_empty() || !mark_as_white.is_empty();

        for coord in mark_as_black {
            grid.mark_cell(coord, State::Black)?;
        }
        for coord in mark_as_white {
            grid.mark_cell(coord, State::White)?;
        }

        Ok(result)
    }
}
