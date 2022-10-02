use super::Strategy;
use crate::{Coord, Grid, State};
use std::{collections::HashSet, rc::Rc};

pub struct UnreachableCells;

impl UnreachableCells {
    fn is_cell_unreachable(&self, grid: &Grid, coord: Coord) -> bool {
        let index = grid.coord_to_index(coord);
        if grid.cells[index].state != State::Unknown {
            return false;
        }

        // TODO: Expand unreachability logic, currently only ensures that two different numbered regions cannot be connected

        let mut adjacent_numbered_regions = vec![];

        for adjacent_coord in grid.valid_neighbors(coord) {
            let adjacent_index = grid.coord_to_index(adjacent_coord);
            if let Some(adjacent_region_ptr) = &grid.cells[adjacent_index].region {
                if let State::Numbered(_) = adjacent_region_ptr.borrow().state {
                    if !adjacent_numbered_regions
                        .iter()
                        .any(|region_ptr| Rc::ptr_eq(region_ptr, adjacent_region_ptr))
                    {
                        adjacent_numbered_regions.push(adjacent_region_ptr.clone());
                    }
                }
            }
        }

        adjacent_numbered_regions.len() > 1
    }
}

impl Strategy for UnreachableCells {
    fn apply(&self, grid: &mut Grid) -> bool {
        let mut mark_as_black = HashSet::<Coord>::new();

        for x in 0..grid.width {
            for y in 0..grid.height {
                if self.is_cell_unreachable(grid, (x, y)) {
                    mark_as_black.insert((x, y));
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
