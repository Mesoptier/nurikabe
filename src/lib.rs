use std::{cell::RefCell, rc::Rc};

use strategy::Strategy;

pub mod analysis;
pub mod grid_from_str;
pub mod strategy;

#[cfg(test)]
mod test_util;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum State {
    Numbered(usize),
    White,
    Black,
    Unknown,
}

#[derive(Clone)]
struct Cell {
    state: State,
    region: Option<Rc<RefCell<Region>>>,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            state: State::Unknown,
            region: None,
        }
    }
}

#[derive(Debug)]
struct Region {
    state: State,
    /// Coordinates of cells in the region
    coords: Vec<Coord>,
    /// Coordinates of unknown cells neighboring the region
    unknowns: Vec<Coord>,
}

pub struct Grid {
    num_rows: usize,
    num_cols: usize,
    cells: Box<[Cell]>,
    regions: Vec<Rc<RefCell<Region>>>,
    total_black_cells: usize,
}

impl Grid {
    pub fn new(
        num_rows: usize,
        num_cols: usize,
        givens: impl IntoIterator<Item = (Coord, usize)>,
    ) -> Self {
        let mut grid = Self::new_empty(num_rows, num_cols);

        let mut total_white_cells = 0;

        for (coord, given) in givens {
            let state = State::Numbered(given);
            let region_ptr = Rc::new(RefCell::new(Region {
                state,
                coords: vec![coord],
                // Note: All neighbors are unknown at this point, otherwise the input would be invalid.
                unknowns: grid.valid_neighbors(coord).collect(),
            }));
            grid.regions.push(region_ptr.clone());
            *grid.cell_mut(coord) = Cell {
                state,
                region: Some(region_ptr.clone()),
            };

            total_white_cells += given;
        }

        grid.total_black_cells = num_cols * num_rows - total_white_cells;

        grid
    }

    fn new_empty(num_rows: usize, num_cols: usize) -> Self {
        Self {
            num_rows,
            num_cols,
            cells: vec![Default::default(); num_cols * num_rows].into_boxed_slice(),
            regions: vec![],
            total_black_cells: 0,
        }
    }

    fn coord_to_index(&self, coord: Coord) -> usize {
        coord.row * self.num_cols + coord.col
    }

    fn valid_neighbors(&self, coord: Coord) -> impl Iterator<Item = Coord> {
        let num_cols = self.num_cols as isize;
        let num_rows = self.num_rows as isize;

        [
            (coord.row as isize - 1, coord.col as isize),
            (coord.row as isize + 1, coord.col as isize),
            (coord.row as isize, coord.col as isize - 1),
            (coord.row as isize, coord.col as isize + 1),
        ]
        .into_iter()
        .filter_map(move |(row, col)| {
            if row >= 0 && row < num_rows && col >= 0 && col < num_cols {
                Some(Coord::new(row as usize, col as usize))
            } else {
                None
            }
        })
    }

    fn valid_unknown_neighbors(&self, coord: Coord) -> impl Iterator<Item = Coord> + '_ {
        self.valid_neighbors(coord)
            .filter(move |&coord| self.cell(coord).state == State::Unknown)
    }

    pub(crate) fn cell(&self, coord: Coord) -> &Cell {
        &self.cells[self.coord_to_index(coord)]
    }

    pub(crate) fn cell_mut(&mut self, coord: Coord) -> &mut Cell {
        &mut self.cells[self.coord_to_index(coord)]
    }

    pub(crate) fn cells(&self) -> impl Iterator<Item = &Cell> {
        self.cells.iter()
    }

    fn mark_cell(&mut self, coord: Coord, state: State) {
        // TODO: Return Result:Err instead of panicking when contradiction occurs
        assert_eq!(self.cell(coord).state, State::Unknown);

        {
            // Create new region containing only the given cell
            let region = Rc::new(RefCell::new(Region {
                state,
                coords: vec![coord],
                unknowns: self.valid_unknown_neighbors(coord).collect(),
            }));
            self.regions.push(region.clone());

            // Mark the given cell, and link it to the new region
            self.cell_mut(coord).state = state;
            self.cell_mut(coord).region = Some(region.clone());
        }

        // Update adjacent regions
        for adjacent_coord in self.valid_neighbors(coord) {
            if let Some(adjacent_region_ptr) = self.cell(adjacent_coord).region.clone() {
                // Remove cell from the unknowns of all adjacent regions
                // TODO: Performance can probably be improved slightly by using .swap_remove()
                adjacent_region_ptr
                    .borrow_mut()
                    .unknowns
                    .retain(|&unknown| unknown != coord);

                // Add cell to adjacent regions with equivalent state, potentially fusing some regions
                // TODO: Make sure that this equivalence check is correct
                let is_adjacent_state_equivalent = match adjacent_region_ptr.borrow().state {
                    State::Unknown => unreachable!(),
                    State::White | State::Numbered(_) => state == State::White,
                    State::Black => state == State::Black,
                };
                if is_adjacent_state_equivalent {
                    self.fuse_regions(
                        adjacent_region_ptr,
                        self.cell(coord).region.clone().unwrap(),
                    );
                }
            }
        }
    }

    fn fuse_regions(&mut self, r1: Rc<RefCell<Region>>, r2: Rc<RefCell<Region>>) {
        // TODO: Check correctness. E.g. currently we might lose a numbered region if r1 is white and r2 is numbered.

        // No need to fuse a region to itself
        if Rc::ptr_eq(&r1, &r2) {
            return;
        }

        // Add cells of r2 to r1
        r1.borrow_mut().coords.extend(r2.borrow().coords.iter());
        for &coord in &r2.borrow().coords {
            self.cell_mut(coord).region = Some(r1.clone());
        }

        // Add new unknowns from r2 to r1
        for &coord in &r2.borrow().unknowns {
            if !r1.borrow().unknowns.contains(&coord) {
                r1.borrow_mut().unknowns.push(coord);
            }
        }

        // Remove r2 from the grid
        self.regions.retain(|r| !Rc::ptr_eq(r, &r2));
        assert_eq!(Rc::strong_count(&r2), 1);
    }

    fn is_complete(&self) -> bool {
        let total_cells = self.num_cols * self.num_rows;
        let marked_cells = self
            .regions
            .iter()
            .map(|region| region.borrow().coords.len())
            .sum::<usize>();
        total_cells == marked_cells
    }
}

#[cfg(feature = "display")]
mod display {
    use crate::{Coord, Grid, State};
    use colored::Colorize;
    use std::fmt::{Display, Formatter};

    impl Display for Grid {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            GridDiff {
                grid: self,
                prev_states: &[],
            }
            .fmt(f)
        }
    }

    impl Grid {
        pub(super) fn diff<'a>(&'a self, prev_states: &'a [State]) -> GridDiff<'a> {
            assert_eq!(prev_states.len(), self.cells.len());
            GridDiff {
                grid: self,
                prev_states,
            }
        }
    }

    pub(super) struct GridDiff<'a> {
        grid: &'a Grid,
        prev_states: &'a [State],
    }

    impl<'a> Display for GridDiff<'a> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            for row in 0..self.grid.num_rows {
                for col in 0..self.grid.num_cols {
                    let state = self.grid.cell(Coord::new(row, col)).state;
                    let prev_state = self
                        .prev_states
                        .get(self.grid.coord_to_index(Coord::new(row, col)))
                        .copied();

                    let string = match (state, prev_state) {
                        (State::Numbered(number), _) => {
                            format!("{:^3}", number.to_string().black())
                        }
                        (state, Some(prev_state)) if state != prev_state => {
                            format!("{:^3}", "*".bright_red())
                        }
                        _ => format!("{:3}", ""),
                    };

                    match state {
                        State::Unknown => write!(f, "{}", string.on_white())?,
                        State::White | State::Numbered(_) => {
                            write!(f, "{}", string.on_bright_white())?
                        }
                        State::Black => write!(f, "{}", string.on_black())?,
                    };
                }

                writeln!(f, " ")?;
            }

            Ok(())
        }
    }
}

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
