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
