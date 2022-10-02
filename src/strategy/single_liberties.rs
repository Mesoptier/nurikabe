use super::Strategy;
use crate::{Coord, Grid, State};
use std::collections::HashSet;

pub(crate) struct SingleLiberties;

impl Strategy for SingleLiberties {
    fn apply(&self, grid: &mut Grid) {
        let mut mark_as_black = HashSet::<Coord>::new();
        let mut mark_as_white = HashSet::<Coord>::new();

        for region_ptr in &grid.regions {
            let region = region_ptr.borrow();

            let is_region_complete = match region.state {
                State::Unknown => unreachable!(),
                State::White => false,
                State::Black => region.coords.len() == grid.total_black_cells,
                State::Numbered(number) => region.coords.len() == number,
            };

            if !is_region_complete && region.unknowns.len() == 1 {
                let unknown_coord = region.unknowns[0];
                match region.state {
                    State::Unknown => unreachable!(),
                    State::White | State::Numbered(_) => mark_as_white.insert(unknown_coord),
                    State::Black => mark_as_black.insert(unknown_coord),
                };
            }
        }

        for coord in mark_as_black {
            grid.mark_cell(coord, State::Black);
        }
        for coord in mark_as_white {
            grid.mark_cell(coord, State::White);
        }
    }
}
