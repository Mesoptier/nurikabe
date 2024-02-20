pub use grid::*;
use strategy::Strategy;

pub mod analysis;
mod grid;
pub mod strategy;
#[cfg(test)]
mod test_util;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Cell {
    state: State,
    region: Option<RegionID>,
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
    regions: Vec<Option<Region>>,
    available_region_ids: Vec<RegionID>,
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
            let region_id = grid.insert_region(Region {
                state,
                coords: vec![coord],
                // Note: All neighbors are unknown at this point, otherwise the input would be invalid.
                unknowns: grid.valid_neighbors(coord).collect(),
            });
            *grid.cell_mut(coord) = Cell {
                state,
                region: Some(region_id),
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
            available_region_ids: vec![],
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

    pub(crate) fn region(&self, region_id: RegionID) -> Option<&Region> {
        self.regions[region_id.to_raw()].as_ref()
    }

    pub(crate) fn region_mut(&mut self, region_id: RegionID) -> Option<&mut Region> {
        self.regions[region_id.to_raw()].as_mut()
    }

    pub(crate) fn regions(&self) -> impl Iterator<Item = &Region> {
        self.regions.iter().filter_map(Option::as_ref)
    }

    fn insert_region(&mut self, region: Region) -> RegionID {
        if let Some(region_id) = self.available_region_ids.pop() {
            self.regions[region_id.to_raw()] = Some(region);
            region_id
        } else {
            let region_id = unsafe {
                // SAFETY: We are about to push a new region, so the length of the regions vector
                // is a valid region ID.
                RegionID::from_raw(self.regions.len())
            };
            self.regions.push(Some(region));
            region_id
        }
    }

    fn mark_cell(&mut self, coord: Coord, state: State) {
        // TODO: Return Result:Err instead of panicking when contradiction occurs
        assert_eq!(self.cell(coord).state, State::Unknown);

        {
            // Create new region containing only the given cell
            let region_id = self.insert_region(Region {
                state,
                coords: vec![coord],
                unknowns: self.valid_unknown_neighbors(coord).collect(),
            });

            // Mark the given cell, and link it to the new region
            self.cell_mut(coord).state = state;
            self.cell_mut(coord).region = Some(region_id);
        }

        // Update adjacent regions
        for adjacent_coord in self.valid_neighbors(coord) {
            if let Some(adjacent_region_id) = self.cell(adjacent_coord).region {
                let adjacent_region = self.region_mut(adjacent_region_id).unwrap();

                // Remove cell from the unknowns of all adjacent regions
                // TODO: Performance can probably be improved slightly by using .swap_remove()
                adjacent_region.unknowns.retain(|&unknown| unknown != coord);

                // Add cell to adjacent regions with equivalent state, potentially fusing some regions
                // TODO: Make sure that this equivalence check is correct
                let is_adjacent_state_equivalent = match adjacent_region.state {
                    State::Unknown => unreachable!(),
                    State::White | State::Numbered(_) => state == State::White,
                    State::Black => state == State::Black,
                };
                if is_adjacent_state_equivalent {
                    self.fuse_regions(adjacent_region_id, self.cell(coord).region.unwrap());
                }
            }
        }
    }

    fn fuse_regions(&mut self, region_id_1: RegionID, region_id_2: RegionID) {
        // TODO: Check correctness. E.g. currently we might lose a numbered region if r1 is white and r2 is numbered.

        // No need to fuse a region to itself
        if region_id_1 == region_id_2 {
            return;
        }

        let region_2 = self.regions[region_id_2.to_raw()].take().unwrap();
        let region_1 = self.regions[region_id_1.to_raw()].as_mut().unwrap();

        // Add new unknowns from region_2 to region_1
        for coord in region_2.unknowns {
            if !region_1.unknowns.contains(&coord) {
                region_1.unknowns.push(coord);
            }
        }

        // Add cells from region_2 to region_1
        region_1.coords.extend(&region_2.coords);
        for coord in region_2.coords {
            self.cells[self.coord_to_index(coord)].region = Some(region_id_1);
        }
    }

    fn is_complete(&self) -> bool {
        let total_cells = self.num_cols * self.num_rows;
        let marked_cells = self
            .regions()
            .map(|region| region.coords.len())
            .sum::<usize>();
        total_cells == marked_cells
    }
}

#[cfg(feature = "display")]
mod display {
    use std::fmt::{Display, Formatter};

    use colored::Colorize;

    use crate::{Coord, Grid, State};

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
