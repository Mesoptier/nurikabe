use colored::*;
use std::{fmt::Display, rc::Rc};

type Coord = (usize, usize);

fn coord_to_index(width: usize, coord: Coord) -> usize {
    coord.0 + coord.1 * width
}

fn index_to_coord(width: usize, index: usize) -> Coord {
    (index % width, index / width)
}

#[derive(Copy, Clone)]
enum State {
    Unknown,
    White,
    Black,
    Numbered(usize),
}

#[derive(Clone)]
struct Cell {
    state: State,
    region: Option<Rc<Region>>,
}

struct Region {
    state: State,
    /// Coordinates of cells in the region
    coords: Vec<Coord>,
    /// Coordinates of unknown cells on the frontier of the region
    unknowns: Vec<Coord>,
}

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    regions: Vec<Rc<Region>>,
}

impl Grid {
    fn new(width: usize, height: usize, givens: Vec<(Coord, usize)>) -> Self {
        let mut cells = vec![
            Cell {
                state: State::Unknown,
                region: None,
            };
            width * height
        ];
        let mut regions = vec![];

        for &(coord, given) in &givens {
            let state = State::Numbered(given);
            let region_ptr = Rc::new(Region {
                state,
                coords: vec![coord],
                unknowns: vec![], // TODO fill unknowns
            });
            regions.push(region_ptr.clone());
            cells[coord_to_index(width, coord)] = Cell {
                state: State::Numbered(given),
                region: Some(region_ptr.clone()),
            };
        }

        Grid {
            width,
            height,
            cells,
            regions,
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = &self.cells[coord_to_index(self.width, (x, y))];

                match cell.state {
                    State::Unknown => write!(f, "{:^3}", " ".on_white())?,
                    State::White => write!(f, "{:3}", " ".on_bright_white())?,
                    State::Black => write!(f, "{:3}", " ".on_black())?,
                    State::Numbered(number) => {
                        write!(f, "{}", format!("{:^3}", number).on_bright_white())?
                    }
                };
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Grid;

    #[test]
    fn grid() {
        let grid = Grid::new(
            5,
            5,
            vec![((0, 0), 2), ((3, 0), 1), ((2, 1), 4), ((4, 2), 3)],
        );
        println!("{}", grid);

        let grid = Grid::new(
            5,
            5,
            vec![((3, 0), 6), ((4, 2), 3), ((2, 3), 2), ((1, 4), 2)],
        );
        println!("{}", grid);
    }
}
