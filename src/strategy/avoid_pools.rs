use super::Strategy;
use crate::{Coord, State};
use std::collections::HashSet;

pub struct AvoidPools;

impl Strategy for AvoidPools {
    fn apply(&self, grid: &mut crate::Grid) -> bool {
        let mut mark_as_white = HashSet::<Coord>::new();

        for x in 1..grid.width {
            for y in 1..grid.height {
                let mut black_cells = vec![];
                let mut unknown_cells = vec![];

                let coords = [(x - 1, y - 1), (x - 1, y), (x, y - 1), (x, y)];
                for coord in coords {
                    let index = grid.coord_to_index(coord);
                    match grid.cells[index].state {
                        State::Black => black_cells.push(coord),
                        State::Unknown => unknown_cells.push(coord),
                        _ => {},
                    };
                }

                if black_cells.len() == 3 && unknown_cells.len() == 1 {
                    mark_as_white.insert(unknown_cells[0]);
                }
            }
        }

        let result = !mark_as_white.is_empty();
        for coord in mark_as_white {
            grid.mark_cell(coord, State::White);
        }
        result
    }
}
