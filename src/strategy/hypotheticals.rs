use crate::grid::State;
use crate::strategy::{Strategy, StrategyResult};
use crate::{Grid, Solver, SolverError};

pub struct Hypotheticals {
    solver: Solver,
}

impl Hypotheticals {
    pub fn new(strategies: Vec<Box<dyn Strategy>>) -> Self {
        Self {
            solver: Solver::new(strategies),
        }
    }
}

impl Strategy for Hypotheticals {
    fn name(&self) -> &str {
        "Hypotheticals"
    }

    fn apply(&self, grid: &mut Grid) -> StrategyResult {
        let result = grid
            .iter()
            .filter(|(_, cell)| cell.state.is_none())
            .flat_map(|(coord, _)| {
                [State::Black, State::White]
                    .iter()
                    .map(move |state| (coord, *state))
            })
            .find_map(|(coord, state)| {
                let mut hypothetical_grid = grid.clone();
                let result = hypothetical_grid
                    .mark_cell(coord, state)
                    .and_then(|_| self.solver.solve(&mut hypothetical_grid));
                match result {
                    Ok(_) => {
                        // Solution found
                        Some((coord, state))
                    }
                    Err(SolverError::Contradiction) => {
                        // Contradiction found
                        Some((coord, state.opposite()))
                    }
                    Err(SolverError::NoStrategyApplies) => None,
                }
            });

        match result {
            Some((coord, state)) => {
                grid.mark_cell(coord, state).unwrap();
                Ok(true)
            }
            None => Ok(false),
        }
    }
}
