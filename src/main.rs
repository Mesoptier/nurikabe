use nurikabe::{
    strategy::{complete_islands::CompleteIslands, single_liberties::SingleLiberties},
    Grid, Solver,
};

fn main() {
    let mut solver = Solver::new(vec![Box::new(CompleteIslands), Box::new(SingleLiberties)]);

    let mut grid = Grid::new(
        5,
        5,
        vec![((0, 0), 2), ((3, 0), 1), ((2, 1), 4), ((4, 2), 3)],
    );
    println!("{}", grid);

    solver.solve(&mut grid);
}
