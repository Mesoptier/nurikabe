use crate::SolverError;

#[cfg(feature = "display")]
pub mod display;
pub mod from_str;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

impl Coord {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum State {
    Numbered(usize),
    White,
    Black,
}

impl State {
    pub(crate) fn is_numbered(self) -> bool {
        matches!(self, Self::Numbered(_))
    }
    pub(crate) fn is_white(self) -> bool {
        matches!(self, Self::White)
    }
    pub(crate) fn is_black(self) -> bool {
        matches!(self, Self::Black)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Cell {
    // TODO: state.is_some() <=> region.is_some(), so we could use a single Option<StateOrRegion>?
    pub(crate) state: Option<State>,
    pub(crate) region: Option<RegionID>,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            state: None,
            region: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct RegionID(usize);

impl RegionID {
    pub fn to_index(self) -> usize {
        self.0
    }
}

#[derive(Debug)]
pub(crate) struct Region {
    pub(crate) state: State,
    /// Coordinates of cells in the region
    pub(crate) coords: Vec<Coord>,
    /// Coordinates of unknown cells neighboring the region
    pub(crate) unknowns: Vec<Coord>,
}

pub struct Grid {
    pub(crate) num_rows: usize,
    pub(crate) num_cols: usize,
    cells: Box<[Cell]>,
    regions: Vec<Option<Region>>,
    available_region_ids: Vec<RegionID>,
    pub(crate) total_black_cells: usize,
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
                state: Some(state),
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

    fn index_to_coord(&self, index: usize) -> Coord {
        Coord::new(index / self.num_cols, index % self.num_cols)
    }

    pub(crate) fn valid_neighbors(&self, coord: Coord) -> impl Iterator<Item = Coord> {
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

    pub(crate) fn valid_unknown_neighbors(&self, coord: Coord) -> impl Iterator<Item = Coord> + '_ {
        self.valid_neighbors(coord)
            .filter(move |&coord| self.cell(coord).state == None)
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

    pub(crate) fn iter(&self) -> impl Iterator<Item = (Coord, &Cell)> {
        self.cells()
            .enumerate()
            .map(move |(index, cell)| (self.index_to_coord(index), cell))
    }

    pub(crate) fn region(&self, region_id: RegionID) -> Option<&Region> {
        self.regions[region_id.to_index()].as_ref()
    }

    pub(crate) fn region_mut(&mut self, region_id: RegionID) -> Option<&mut Region> {
        self.regions[region_id.to_index()].as_mut()
    }

    pub(crate) fn regions(&self) -> impl Iterator<Item = &Region> {
        self.regions.iter().filter_map(Option::as_ref)
    }

    pub(crate) fn regions_iter(&self) -> impl Iterator<Item = (RegionID, &Region)> {
        self.regions
            .iter()
            .enumerate()
            .filter_map(|(index, region)| region.as_ref().map(|region| (RegionID(index), region)))
    }

    fn insert_region(&mut self, region: Region) -> RegionID {
        if let Some(region_id) = self.available_region_ids.pop() {
            self.regions[region_id.to_index()] = Some(region);
            region_id
        } else {
            let region_id = RegionID(self.regions.len());
            self.regions.push(Some(region));
            region_id
        }
    }

    fn remove_region(&mut self, region_id: RegionID) -> Option<Region> {
        self.available_region_ids.push(region_id);
        self.regions[region_id.to_index()].take()
    }

    pub(crate) fn mark_cell(&mut self, coord: Coord, state: State) -> Result<(), SolverError> {
        if self.cell(coord).state.is_some() {
            // If the cell is already marked, we can't mark it again
            return Err(SolverError::Contradiction);
        }

        {
            // Create new region containing only the given cell
            let region_id = self.insert_region(Region {
                state,
                coords: vec![coord],
                unknowns: self.valid_unknown_neighbors(coord).collect(),
            });

            // Mark the given cell, and link it to the new region
            self.cell_mut(coord).state = Some(state);
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
                    State::White | State::Numbered(_) => state == State::White,
                    State::Black => state == State::Black,
                };
                if is_adjacent_state_equivalent {
                    self.fuse_regions(adjacent_region_id, self.cell(coord).region.unwrap());
                }
            }
        }

        Ok(())
    }

    fn fuse_regions(&mut self, region_id_1: RegionID, region_id_2: RegionID) {
        // TODO: Check correctness. E.g. currently we might lose a numbered region if r1 is white and r2 is numbered.

        // No need to fuse a region to itself
        if region_id_1 == region_id_2 {
            return;
        }

        if self.region(region_id_2).unwrap().state.is_numbered() {
            // Swap the region IDs so that region_id_1 is the numbered region
            return self.fuse_regions(region_id_2, region_id_1);
        }

        let region_2 = self.remove_region(region_id_2).unwrap();
        let region_1 = self.regions[region_id_1.to_index()].as_mut().unwrap();

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

    pub(crate) fn is_complete(&self) -> bool {
        let total_cells = self.num_cols * self.num_rows;
        let marked_cells = self
            .regions()
            .map(|region| region.coords.len())
            .sum::<usize>();
        total_cells == marked_cells
    }
}
