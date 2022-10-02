use super::Strategy;
use crate::{Coord, Grid, State};
use std::collections::HashSet;

pub(crate) struct CompleteIslands;

impl Strategy for CompleteIslands {
    fn apply(&self, grid: &mut Grid) -> bool {
        let mut mark_as_black = HashSet::<Coord>::new();

        for region_ptr in &grid.regions {
            let region = region_ptr.borrow();
            if let State::Numbered(number) = region.state {
                if number == region.coords.len() {
                    mark_as_black.extend(region.unknowns.iter());
                }
            }
        }

        let result = !mark_as_black.is_empty();

        for coord in mark_as_black {
            grid.mark_cell(coord, State::Black);
        }

        result
    }
}
