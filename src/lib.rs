use colored::*;
use std::{fmt::Display, rc::Rc};

type Coord = (usize, usize);

fn coord_to_index(width: usize, coord: Coord) -> usize {
    coord.0 + coord.1 * width
}

fn index_to_coord(width: usize, index: usize) -> Coord {
    (index % width, index / width)
}

enum RegionKind {
    White(Option<usize>),
    Black,
}

struct Region {
    kind: RegionKind,
    cells: Vec<Coord>,
}

struct Grid {
    width: usize,
    height: usize,
    givens: Vec<(Coord, usize)>,
    regions: Vec<Rc<Region>>,
    regions_by_cell: Vec<Option<Rc<Region>>>,
}

impl Grid {
    fn new(width: usize, height: usize, givens: Vec<(Coord, usize)>) -> Self {
        let mut regions = vec![];
        let mut regions_by_cell = vec![None; width * height];

        for &(coord, given) in &givens {
            let region_ptr = Rc::new(Region {
                kind: RegionKind::White(Some(given)),
                cells: vec![coord],
            });
            regions.push(region_ptr.clone());
            regions_by_cell[coord_to_index(width, coord)] = Some(region_ptr.clone());
        }

        Self {
            width,
            height,
            givens,
            regions,
            regions_by_cell,
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = coord_to_index(self.width, (x, y));

                if let Some(region_ptr) = &self.regions_by_cell[index] {
                    match region_ptr.kind {
                        RegionKind::Black => write!(f, "{:3}", " ".on_black())?,
                        RegionKind::White(None) => write!(f, "{:3}", " ".on_bright_white())?,
                        RegionKind::White(Some(given)) => {
                            write!(f, "{}", format!("{:^3}", given).on_bright_white())?
                        }
                    }
                } else {
                    write!(f, "{:^3}", " ".on_white())?;
                }
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
