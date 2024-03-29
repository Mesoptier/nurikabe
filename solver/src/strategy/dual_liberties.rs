use crate::{Grid, State};

use super::{MarkSet, Strategy, StrategyResult};

pub struct DualLiberties;

impl Strategy for DualLiberties {
    fn name(&self) -> &str {
        "DualLiberties"
    }

    fn apply(&self, grid: &mut Grid) -> StrategyResult {
        let mut mark_set = MarkSet::new();

        for region in grid.regions() {
            if let State::Numbered(number) = region.state {
                if region.len() + 1 == number && region.unknowns_len() == 2 {
                    let adj1 = grid.valid_unknown_neighbors(region.unknowns[0]);
                    let adj2 = grid
                        .valid_unknown_neighbors(region.unknowns[1])
                        .collect::<Vec<_>>();

                    for coord in adj1 {
                        if adj2.contains(&coord) {
                            mark_set.insert(coord, State::Black);
                            break;
                        }
                    }
                }
            }
        }

        mark_set.apply(grid)
    }
}

#[cfg(test)]
mod tests {
    use crate::strategy::test_strategy;

    use super::DualLiberties;

    test_strategy!(test_apply, DualLiberties, "2.\n..", Some("2.\n.B"));
    test_strategy!(test_already_marked, DualLiberties, "2.\n.B", None);
    test_strategy!(test_already_completed, DualLiberties, "1.\n.B", None);
}
