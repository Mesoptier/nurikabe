use std::collections::HashSet;

use crate::analysis::is_cell_unreachable;
use crate::{Coord, State};

use super::Strategy;

pub struct AvoidPools;

impl Strategy for AvoidPools {
    fn name(&self) -> &str {
        "AvoidPools"
    }

    fn apply(&self, grid: &mut crate::Grid) -> bool {
        let mut mark_as_white = HashSet::<Coord>::new();

        for col in 1..grid.num_cols {
            for row in 1..grid.num_rows {
                let mut cells = [
                    Coord::new(row - 1, col - 1),
                    Coord::new(row - 1, col),
                    Coord::new(row, col - 1),
                    Coord::new(row, col),
                ]
                .map(|c| (c, grid.cell(c).state));

                assert!(State::Black < State::Unknown);
                cells.sort_unstable_by_key(|(_, state)| *state);

                match cells {
                    // With three black cells in a 2x2 square, a remaining unknown cell must be
                    // marked white.
                    [(_, State::Black), (_, State::Black), (_, State::Black), (coord, State::Unknown)] =>
                    {
                        mark_as_white.insert(coord);
                    }
                    // With two black cells and two unknown cells in a 2x2 square. If marking one of
                    // the unknown cells black would make the other one unreachable, then it must be
                    // marked white.
                    [(_, State::Black), (_, State::Black), (coord_1, State::Unknown), (coord_2, State::Unknown)] => {
                        if is_cell_unreachable(grid, coord_1, [coord_2]) {
                            mark_as_white.insert(coord_2);
                        } else if is_cell_unreachable(grid, coord_2, [coord_1]) {
                            mark_as_white.insert(coord_1);
                        }
                    }
                    _ => {}
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
