pub use grid::*;
use strategy::Strategy;

pub mod analysis;
mod grid;
pub mod strategy;
#[cfg(test)]
mod test_util;

pub struct Solver {
    strategies: Vec<Box<dyn Strategy>>,
}

impl Solver {
    pub fn new(strategies: Vec<Box<dyn Strategy>>) -> Self {
        Self { strategies }
    }

    pub fn solve(&mut self, grid: &mut Grid) {
        while !grid.is_complete() {
            let mut result = false;

            #[cfg(feature = "display")]
            let prev_states = grid.cells().map(|cell| cell.state).collect::<Vec<_>>();

            for strategy in &self.strategies {
                result = strategy.apply(grid);
                if result {
                    #[cfg(feature = "display")]
                    eprintln!("applying strategy {}", strategy.name());
                    break;
                }
            }

            if !result {
                #[cfg(feature = "display")]
                eprintln!("no strategy applies");
                break;
            }

            #[cfg(feature = "display")]
            println!("{}", grid.diff(&prev_states));
        }
    }
}
