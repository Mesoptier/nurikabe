use colored::*;
use std::{cell::RefCell, fmt::Display, rc::Rc};
use strategy::Strategy;

pub mod strategy;

#[cfg(test)]
mod test_util;

type Coord = (usize, usize);

fn coord_to_index(width: usize, coord: Coord) -> usize {
    coord.0 + coord.1 * width
}

fn index_to_coord(width: usize, index: usize) -> Coord {
    (index % width, index / width)
}

fn valid_neighbors(width: usize, height: usize, coord: Coord) -> Vec<Coord> {
    let (x, y) = coord;
    let mut neighbors = vec![];

    if 0 < x {
        neighbors.push((x - 1, y));
    }
    if x + 1 < width {
        neighbors.push((x + 1, y));
    }
    if 0 < y {
        neighbors.push((x, y - 1));
    }
    if y + 1 < height {
        neighbors.push((x, y + 1));
    }

    neighbors
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

#[derive(Debug)]
struct Region {
    state: State,
    /// Coordinates of cells in the region
    coords: Vec<Coord>,
    /// Coordinates of unknown cells neighboring the region
    unknowns: Vec<Coord>,
}

pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    regions: Vec<Rc<RefCell<Region>>>,
    total_black_cells: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize, givens: Vec<(Coord, usize)>) -> Self {
        let mut cells = vec![
            Cell {
                state: State::Unknown,
                region: None,
            };
            width * height
        ];
        let mut regions = vec![];

        let mut total_white_cells = 0;

        for &(coord, given) in &givens {
            let state = State::Numbered(given);
            let region_ptr = Rc::new(RefCell::new(Region {
                state,
                coords: vec![coord],
                unknowns: valid_neighbors(width, height, coord),
            }));
            regions.push(region_ptr.clone());
            cells[coord_to_index(width, coord)] = Cell {
                state: State::Numbered(given),
                region: Some(region_ptr.clone()),
            };

            total_white_cells += given;
        }

        Grid {
            width,
            height,
            cells,
            regions,
            total_black_cells: width * height - total_white_cells,
        }
    }

    fn coord_to_index(&self, coord: Coord) -> usize {
        coord_to_index(self.width, coord)
    }

    fn valid_neighbors(&self, coord: Coord) -> Vec<Coord> {
        valid_neighbors(self.width, self.height, coord)
    }

    fn valid_unknown_neighbors(&self, coord: Coord) -> Vec<Coord> {
        let mut neighbors = self.valid_neighbors(coord);
        neighbors.retain(|&coord| self.cells[self.coord_to_index(coord)].state == State::Unknown);
        neighbors
    }

    fn mark_cell(&mut self, coord: Coord, state: State) {
        let index = self.coord_to_index(coord);

        // TODO: Return Result:Err instead of panicking when contradiction occurs
        assert_eq!(self.cells[index].state, State::Unknown);

        {
            // Create new region containing only the given cell
            let region = Rc::new(RefCell::new(Region {
                state,
                coords: vec![coord],
                unknowns: self.valid_unknown_neighbors(coord),
            }));
            self.regions.push(region.clone());

            // Mark the given cell, and link it to the new region
            self.cells[index].state = state;
            self.cells[index].region = Some(region.clone());
        }

        // Update adjacent regions
        for adjacent_coord in self.valid_neighbors(coord) {
            let adjacent_index = self.coord_to_index(adjacent_coord);

            if let Some(adjacent_region_ptr) = self.cells[adjacent_index].region.clone() {
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
                        self.cells[index].region.clone().unwrap(),
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
            let index = self.coord_to_index(coord);
            self.cells[index].region = Some(r1.clone());
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
        let total_cells = self.width * self.height;
        let marked_cells = self.regions.iter().map(|region| region.borrow().coords.len()).sum::<usize>();
        total_cells == marked_cells
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = &self.cells[coord_to_index(self.width, (x, y))];

                let string = match cell.state {
                    State::Numbered(number) => format!("{:^3}", number),
                    _ => format!("{:3}", ""),
                };

                match cell.state {
                    State::Unknown => write!(f, "{}", string.on_white())?,
                    State::White | State::Numbered(_) => write!(f, "{}", string.on_bright_white())?,
                    State::Black => write!(f, "{}", string.on_black())?,
                };
            }

            writeln!(f, " ")?;
        }

        Ok(())
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

            for strategy in &self.strategies {
                result = strategy.apply(grid);
                if result {
                    break;
                }
            }

            if !result {
                eprintln!("no strategy applies");
                break;
            }

            println!("{}", grid);
        }
    }
}
