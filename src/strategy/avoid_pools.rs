use super::Strategy;
use crate::{Coord, State};
use std::collections::HashSet;

pub struct AvoidPools;

impl Strategy for AvoidPools {
    fn name(&self) -> &str {
        "AvoidPools"
    }

    fn apply(&self, grid: &mut crate::Grid) -> bool {
        let mut mark_as_white = HashSet::<Coord>::new();

        for col in 1..grid.num_cols {
            for row in 1..grid.num_rows {
                let mut black_cells = vec![];
                let mut unknown_cells = vec![];

                let coords = [
                    Coord::new(row - 1, col - 1),
                    Coord::new(row - 1, col),
                    Coord::new(row, col - 1),
                    Coord::new(row, col),
                ];
                for coord in coords {
                    let index = grid.coord_to_index(coord);
                    match grid.cells[index].state {
                        State::Black => black_cells.push(coord),
                        State::Unknown => unknown_cells.push(coord),
                        _ => {}
                    };
                }

                if black_cells.len() == 3 && unknown_cells.len() == 1 {
                    mark_as_white.insert(unknown_cells[0]);
                }

                // TODO: in cases with 2 black cells and 2 unknown cells, we
                //  might want to try both unknown cells for unreachability.
            }
        }

        let result = !mark_as_white.is_empty();
        for coord in mark_as_white {
            grid.mark_cell(coord, State::White);
        }
        result
    }
}
