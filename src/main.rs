use std::str::FromStr;

use nurikabe::{
    strategy::{
        avoid_pools::AvoidPools, complete_islands::CompleteIslands, confinement::Confinement,
        dual_liberties::DualLiberties, hypotheticals::Hypotheticals,
        single_liberties::SingleLiberties, unreachable_cells::UnreachableCells,
    },
    DisplayLogger, Grid, Solver,
};

fn main() {
    let solver = Solver::new(vec![
        Box::new(CompleteIslands),
        Box::new(SingleLiberties),
        Box::new(DualLiberties),
        Box::new(AvoidPools),
        Box::new(UnreachableCells),
        Box::new(Confinement),
        Box::new(Hypotheticals::new(vec![
            Box::new(CompleteIslands),
            Box::new(SingleLiberties),
            Box::new(DualLiberties),
            Box::new(AvoidPools),
            Box::new(UnreachableCells),
            // Box::new(Confinement),
        ])),
    ]);

    // https://www.puzzle-nurikabe.com/
    // 5x5 Nurikabe Hard Puzzle ID: 9,690,008
    //let mut grid = Grid::from_str("2..1.\n.....\n...3.\n....5\n.....\n").unwrap();

    // https://en.wikipedia.org/wiki/Nurikabe_(puzzle)
    // let mut grid = Grid::from_str(concat!(
    //     "2........2\n",
    //     "......2...\n",
    //     ".2..7.....\n",
    //     "..........\n",
    //     "......3.3.\n",
    //     "..2....3..\n",
    //     "2..4......\n",
    //     "..........\n",
    //     ".1....2.4.\n",
    // ))
    // .unwrap();

    // Conceptis Puzzles
    // 10x14, Classic Nurikabe, Very Hard, ID: 29090611683
    let mut grid = Grid::from_str(concat!(
        "7....3....\n",
        ".......3..\n",
        "...3..2...\n",
        "..........\n",
        ".........3\n",
        "..2....5..\n",
        "......2...\n",
        ".4.....5..\n",
        "..........\n",
        "......7...\n",
        "........8.\n",
        ".........8\n",
        "..........\n",
        ".3........\n",
    ))
    .unwrap();

    println!("{}", grid);
    let solver_result = solver.solve_with_logger(&mut grid, DisplayLogger::new());
    println!("{:?}", solver_result);
}
