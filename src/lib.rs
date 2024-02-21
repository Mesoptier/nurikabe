pub use grid::*;
use strategy::Strategy;

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
            match region.state {
                State::Numbered(number) => {
                    if region.coords.len() < number && region.unknowns.is_empty() {
                        return Err(SolverError::Contradiction);
                    }
                    if region.coords.len() > number {
                        return Err(SolverError::Contradiction);
                    }
                }
                State::White => {
                    if region.unknowns.is_empty() {
                        return Err(SolverError::Contradiction);
                    }
                }
                State::Black => {
                    if region.unknowns.is_empty() && region.coords.len() != grid.total_black_cells {
                        return Err(SolverError::Contradiction);
                    }
                }
            }
        }

        Ok(())
    }
}
