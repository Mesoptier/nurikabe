use crate::Grid;

pub mod avoid_pools;
pub mod complete_islands;
pub mod confinement;
pub mod dual_liberties;
pub mod single_liberties;
pub mod unreachable_cells;

pub trait Strategy {
    fn name(&self) -> &str;
    fn apply(&self, grid: &mut Grid) -> bool;
}

#[cfg(test)]
macro_rules! test_strategy {
    ($name:ident, $strategy:expr, $input:expr, $expected_output:expr) => {
        #[test]
        fn $name() {
            use crate::Grid;
            use std::str::FromStr;
            use $crate::Strategy;

            let mut grid = Grid::from_str($input).unwrap();
            let output = if $strategy.apply(&mut grid) {
                Some(grid.to_input_string())
            } else {
                None
            };
            assert_eq!(output, $expected_output.map(str::to_string));
        }
    };
}

#[cfg(test)]
use test_strategy;
