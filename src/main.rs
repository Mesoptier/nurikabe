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

    //    let mut grid = Grid::new(
    //        5,
    //        5,
    //        vec![((0, 0), 2), ((3, 0), 1), ((2, 1), 4), ((4, 2), 3)],
    //    );
    //    println!("{}", grid);
    //    solver.solve(&mut grid);

    let mut grid = Grid::new(
        5,
        5,
        vec![((3, 0), 1), ((1, 2), 2), ((3, 2), 2), ((0, 3), 3)],
    );
    println!("{}", grid);
    solver.solve(&mut grid);
}
