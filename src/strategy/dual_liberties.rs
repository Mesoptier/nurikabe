use std::collections::HashSet;

use crate::{Coord, Grid, State};

use super::{Strategy, StrategyResult};

pub struct DualLiberties;

impl Strategy for DualLiberties {
    fn name(&self) -> &str {
        "DualLiberties"
    }

    fn apply(&self, grid: &mut Grid) -> StrategyResult {
        let mut mark_as_black = HashSet::<Coord>::new();

        for region in grid.regions() {
            if let State::Numbered(number) = region.state {
                if region.coords.len() + 1 == number && region.unknowns.len() == 2 {
                    let adj1 = grid.valid_unknown_neighbors(region.unknowns[0]);
                    let adj2 = grid
                        .valid_unknown_neighbors(region.unknowns[1])
                        .collect::<Vec<_>>();

                    for coord in adj1 {
                        if adj2.contains(&coord) {
                            mark_as_black.insert(coord);
                            break;
                        }
                    }
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
