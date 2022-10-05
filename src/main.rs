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
    let mut grid = Grid::new(
        5,
        5,
        vec![((0, 0), 2), ((3, 0), 1), ((3, 2), 3), ((4, 3), 5)],
    );
    println!("{}", grid);
    solver.solve(&mut grid);
}
