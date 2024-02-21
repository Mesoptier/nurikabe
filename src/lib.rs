pub use grid::*;
use strategy::Strategy;

mod grid;
pub mod strategy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        self.solve_with_logger(grid, NoopLogger)
    }

    pub fn solve_with_logger(
        &self,
        grid: &mut Grid,
        mut logger: impl SolverLogger,
    ) -> Result<(), SolverError> {
        'outer: while !grid.is_complete() {
            logger.before_apply(grid);

            for strategy in &self.strategies {
                if strategy.apply(grid)? {
                    logger.strategy_applied(grid, strategy.name());
                    continue 'outer;
                }
            }

            // Detect contradictions before giving up because no strategy applied, so the
            // Hypotheticals strategy has a chance to work.
            self.detect_contradictions(grid)?;

            logger.no_strategy_applies(grid);
            return Err(SolverError::NoStrategyApplies);
        }

        self.detect_contradictions(grid)
    }

    pub fn detect_contradictions(&self, grid: &Grid) -> Result<(), SolverError> {
        for region in grid.regions() {
            if region.is_closed() && grid.is_region_incomplete(region) {
                return Err(SolverError::Contradiction);
            }
            if grid.is_region_overfilled(region) {
                return Err(SolverError::Contradiction);
            }
        }

        Ok(())
    }
}

pub trait SolverLogger {
    fn before_apply(&mut self, grid: &Grid);
    fn strategy_applied(&mut self, grid: &Grid, strategy_name: &str);
    fn no_strategy_applies(&mut self, grid: &Grid);
}

pub struct NoopLogger;
impl SolverLogger for NoopLogger {
    fn before_apply(&mut self, _grid: &Grid) {}
    fn strategy_applied(&mut self, _grid: &Grid, _strategy_name: &str) {}
    fn no_strategy_applies(&mut self, _grid: &Grid) {}
}

#[cfg(feature = "display")]
pub struct DisplayLogger {
    prev_states: Box<[Option<State>]>,
}

#[cfg(feature = "display")]
impl DisplayLogger {
    pub fn new() -> Self {
        Self {
            prev_states: Box::new([]),
        }
    }
}

#[cfg(feature = "display")]
impl SolverLogger for DisplayLogger {
    fn before_apply(&mut self, grid: &Grid) {
        self.prev_states = grid
            .cells()
            .map(|cell| cell.state)
            .collect::<Vec<_>>()
            .into_boxed_slice();
    }

    fn strategy_applied(&mut self, grid: &Grid, strategy_name: &str) {
        println!("applying strategy {}", strategy_name);
        println!("{}", grid.diff(&self.prev_states));
    }

    fn no_strategy_applies(&mut self, grid: &Grid) {
        println!("no strategy applies");
        println!("{}", grid);
    }
}
