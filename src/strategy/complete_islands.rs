use super::Strategy;
use crate::{Coord, Grid, State};
use std::collections::HashSet;

pub struct CompleteIslands;

impl Strategy for CompleteIslands {
    fn name(&self) -> &str { "CompleteIslands" }

    fn apply(&self, grid: &mut Grid) -> bool {
        let mut mark_as_black = HashSet::<Coord>::new();

        for region_ptr in &grid.regions {
            let region = region_ptr.borrow();
            if let State::Numbered(number) = region.state {
                if number == region.coords.len() {
                    mark_as_black.extend(region.unknowns.iter());
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

#[cfg(test)]
mod tests {
    use super::CompleteIslands;
    use crate::{strategy::Strategy, test_util::get_regions, Grid, State};

    #[test]
    fn complete_islands() {
        let mut grid = Grid::new(3, 3, vec![((1, 1), 1)]);

        assert_eq!(CompleteIslands.apply(&mut grid), true);
        assert_eq!(
            get_regions(&grid),
            vec![
                (State::Numbered(1), vec![(1, 1)]),
                (State::Black, vec![(0, 1)]),
                (State::Black, vec![(1, 0)]),
                (State::Black, vec![(1, 2)]),
                (State::Black, vec![(2, 1)]),
            ]
        );
    }
}
