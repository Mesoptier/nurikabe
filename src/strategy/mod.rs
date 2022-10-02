use crate::Grid;

pub mod complete_islands;
pub mod single_liberties;

pub trait Strategy {
    fn apply(&self, grid: &mut Grid) -> bool;
}
