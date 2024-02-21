use crate::{Coord, Grid, State};

use super::{MarkSet, Strategy, StrategyResult};

pub struct UnreachableCells;

impl Strategy for UnreachableCells {
    fn name(&self) -> &str {
        "UnreachableCells"
    }

    fn apply(&self, grid: &mut Grid) -> StrategyResult {
        let mut mark_set = MarkSet::new();

        for col in 0..grid.num_cols {
            for row in 0..grid.num_rows {
                let coord = Coord::new(row, col);
                if grid.is_cell_unreachable(coord, mark_set.mark_as_black.iter().copied()) {
                    mark_set.insert(coord, State::Black);
                }
            }
        }

        mark_set.apply(grid)
    }
}

#[cfg(test)]
mod tests {
    use crate::strategy::test_strategy;

    use super::UnreachableCells;

    test_strategy!(
        test_too_far,
        UnreachableCells,
        "2.\n..\n..",
        Some("2.\n.B\nBB")
    );
    test_strategy!(
        test_between_numbered,
        UnreachableCells,
        "2.2\n...",
        Some("2B2\n.B.")
    );
}
