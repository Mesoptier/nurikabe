use std::collections::HashSet;

use crate::{Coord, Grid, State};

use super::{Strategy, StrategyResult};

pub struct CompleteIslands;

impl Strategy for CompleteIslands {
    fn name(&self) -> &str {
        "CompleteIslands"
    }

    fn apply(&self, grid: &mut Grid) -> StrategyResult {
        let mut mark_as_black = HashSet::<Coord>::new();

        for region in grid.regions() {
            if let State::Numbered(number) = region.state {
                if number == region.len() {
                    mark_as_black.extend(region.unknowns.iter());
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

#[cfg(test)]
mod tests {
    use crate::strategy::test_strategy;

    use super::CompleteIslands;

    test_strategy!(
        complete_1x1_center,
        CompleteIslands,
        "...\n.1.\n...",
        Some(".B.\nB1B\n.B.")
    );
    test_strategy!(
        complete_1x1_edge,
        CompleteIslands,
        "...\n1..\n...",
        Some("B..\n1B.\nB..")
    );
    test_strategy!(
        complete_1x1_corner,
        CompleteIslands,
        "1..\n...\n...",
        Some("1B.\nB..\n...")
    );

    test_strategy!(
        already_complete_1x1_center,
        CompleteIslands,
        ".B.\nB1B\n.B.",
        None
    );
}
