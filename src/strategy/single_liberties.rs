use crate::Grid;

use super::{MarkSet, Strategy, StrategyResult};

pub struct SingleLiberties;

impl Strategy for SingleLiberties {
    fn name(&self) -> &str {
        "SingleLiberties"
    }

    fn apply(&self, grid: &mut Grid) -> StrategyResult {
        let mut mark_set = MarkSet::new();

        for region in grid.regions() {
            if grid.is_region_incomplete(region) && region.unknowns_len() == 1 {
                mark_set.insert(region.unknowns[0], region.state);
            }
        }

        mark_set.apply(grid)
    }
}

#[cfg(test)]
mod tests {
    use crate::strategy::test_strategy;

    use super::SingleLiberties;

    test_strategy!(test_numbered, SingleLiberties, "3..\nBBB", Some("3W.\nBBB"));
    test_strategy!(
        test_numbered_already_complete,
        SingleLiberties,
        "2W..\nBBBB",
        None
    );
    test_strategy!(test_white, SingleLiberties, "W..\nBBB", Some("WW.\nBBB"));
    test_strategy!(test_black, SingleLiberties, "B..\nWWW", Some("BB.\nWWW"));
    test_strategy!(
        test_black_already_complete,
        SingleLiberties,
        "4.\n.W\nBB",
        None
    );
}
