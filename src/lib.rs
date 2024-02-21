pub use grid::*;
use strategy::Strategy;

use crate::analysis::{is_region_complete, is_region_overfilled};

pub mod analysis;
mod grid;
pub mod strategy;

#[derive(Debug)]
pub enum SolverError {
    Contradiction,
    NoStrategyApplies,
}

pub struct Solver {
    strategies: Vec<Box<dyn Strategy>>,
}

impl Solver {
    pub fn new(strategies: Vec<Box<dyn Strategy>>) -> Self {
        Self { strategies }
    }

    pub fn solve(&self, grid: &mut Grid) -> Result<(), SolverError> {
        while !grid.is_complete() {
            let mut result = false;

            #[cfg(feature = "display")]
            let prev_states = grid.cells().map(|cell| cell.state).collect::<Vec<_>>();

            for strategy in &self.strategies {
                result = strategy.apply(grid)?;
                if result {
                    #[cfg(feature = "display")]
                    eprintln!("applying strategy {}", strategy.name());
                    break;
                }
            }

            if !result {
                #[cfg(feature = "display")]
                eprintln!("no strategy applies");
                return Err(SolverError::NoStrategyApplies);
            }

            #[cfg(feature = "display")]
            println!("{}", grid.diff(&prev_states));
        }

        self.detect_contradictions(grid)
    }

    pub fn detect_contradictions(&self, grid: &Grid) -> Result<(), SolverError> {
        for region in grid.regions() {
            if region.is_closed() && !is_region_complete(grid, region) {
                return Err(SolverError::Contradiction);
            }
            if is_region_overfilled(grid, region) {
                return Err(SolverError::Contradiction);
            }
        }

        Ok(())
    }
}
