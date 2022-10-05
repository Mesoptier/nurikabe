use std::str::FromStr;

use nurikabe::{
    strategy::{
        avoid_pools::AvoidPools, complete_islands::CompleteIslands,
        single_liberties::SingleLiberties, unreachable_cells::UnreachableCells,
    },
    Grid, Solver,
};

fn main() {
    let mut solver = Solver::new(vec![
        Box::new(CompleteIslands),
        Box::new(SingleLiberties),
        Box::new(AvoidPools),
        Box::new(UnreachableCells),
    ]);

    // https://www.puzzle-nurikabe.com/
    // 5x5 Nurikabe Hard Puzzle ID: 9,690,008
    //let mut grid = Grid::from_str("2..1./...../...3./....5/.....").unwrap();

    // https://en.wikipedia.org/wiki/Nurikabe_(puzzle)
    let mut grid = Grid::from_str("2........2/......2.../.2..7...../........../......3.3./..2....3../2..4....../........../.1....2.4.").unwrap();

    println!("{}", grid);
    solver.solve(&mut grid);
}
