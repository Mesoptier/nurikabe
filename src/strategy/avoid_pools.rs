use crate::{Coord, SolverError, State};

use super::{MarkSet, Strategy, StrategyResult};

pub struct AvoidPools;

impl Strategy for AvoidPools {
    fn name(&self) -> &str {
        "AvoidPools"
    }

    fn apply(&self, grid: &mut crate::Grid) -> StrategyResult {
        let mut mark_set = MarkSet::new();

        for col in 1..grid.num_cols {
            for row in 1..grid.num_rows {
                let mut cells = [
                    Coord::new(row - 1, col - 1),
                    Coord::new(row - 1, col),
                    Coord::new(row, col - 1),
                    Coord::new(row, col),
                ]
                .map(|c| (c, grid.cell(c).state));

                assert!(Some(State::Black) > None);
                cells.sort_unstable_by_key(|(_, state)| *state);

                match cells {
                    // With three black cells in a 2x2 square, a remaining unknown cell must be
                    // marked white.
                    [(coord, None), (_, Some(State::Black)), (_, Some(State::Black)), (_, Some(State::Black))] =>
                    {
                        mark_set.insert(coord, State::White);
                    }
                    // With two black cells and two unknown cells in a 2x2 square. If marking one of
                    // the unknown cells black would make the other one unreachable, then it must be
                    // marked white.
                    [(coord_1, None), (coord_2, None), (_, Some(State::Black)), (_, Some(State::Black))] => {
                        if grid.is_cell_unreachable(coord_1, [coord_2]) {
                            mark_set.insert(coord_2, State::White);
                        } else if grid.is_cell_unreachable(coord_2, [coord_1]) {
                            mark_set.insert(coord_1, State::White);
                        }
                    }
                    [(_, Some(State::Black)), (_, Some(State::Black)), (_, Some(State::Black)), (_, Some(State::Black))] =>
                    {
                        // Found a 2x2 pool of black cells.
                        return Err(SolverError::Contradiction);
                    }
                    _ => {}
                }
            }
        }

        mark_set.apply(grid)
    }
}

#[cfg(test)]
mod tests {
    use crate::strategy::test_strategy;

    use super::AvoidPools;

    // Three black cells in a 2x2 square, with the remaining cell being unknown.
    test_strategy!(test_three_black_1, AvoidPools, "BB\nB.", Some("BB\nBW"));
    test_strategy!(test_three_black_2, AvoidPools, "BB\n.B", Some("BB\nWB"));
    test_strategy!(test_three_black_3, AvoidPools, "B.\nBB", Some("BW\nBB"));
    test_strategy!(test_three_black_4, AvoidPools, ".B\nBB", Some("WB\nBB"));

    // Two black cells and two unknown cells in a 2x2 square. One of the unknown cells must not be
    // marked black, because it would make the other one unreachable (and therefore also black).
    test_strategy!(test_two_black_1, AvoidPools, "BBW\n..W", Some("BBW\n.WW"));
}
