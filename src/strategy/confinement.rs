use std::collections::HashSet;

use crate::analysis::is_region_confined;
use crate::grid::State;
use crate::strategy::Strategy;
use crate::Grid;

pub struct Confinement;

impl Strategy for Confinement {
    fn name(&self) -> &str {
        "Confinement"
    }

    fn apply(&self, grid: &mut Grid) -> bool {
        let mut mark_as_white = HashSet::new();
        let mut mark_as_black = HashSet::new();

        grid.iter()
            .filter(|(_, cell)| cell.state.is_none())
            .for_each(|(coord, _)| {
                grid.regions_iter()
                    .filter(|(region_id, _)| is_region_confined(grid, *region_id, [coord]))
                    .for_each(|(_, region)| {
                        if region.state.is_black() {
                            mark_as_black.insert(coord);
                        } else {
                            mark_as_white.insert(coord);
                        }
                    })
            });

        grid.regions_iter()
            .filter(|(_, region)| matches!(region.state, State::Numbered(number) if region.coords.len() < number))
            .for_each(|(region_id, region)| {
                region.unknowns.iter().for_each(|&coord| {
                    let mut assume_visited = vec![coord];
                    assume_visited.extend(grid.valid_unknown_neighbors(coord));

                    grid.valid_neighbors(coord)
                        .map(|coord| grid.cell(coord))
                        .filter(|cell| matches!(cell.state, Some(State::White)))
                        .for_each(|cell| {
                            let region = grid.region(cell.region.unwrap()).unwrap();
                            assume_visited.extend(region.unknowns.iter().copied());
                        });

                    grid.regions_iter()
                        .filter(|(other_region_id, _)| *other_region_id != region_id)
                        .filter(|(_, other_region)| other_region.state.is_numbered())
                        .for_each(|(other_region_id, _)| {
                            if is_region_confined(grid, other_region_id, assume_visited.iter().copied()) {
                                mark_as_black.insert(coord);
                            }
                        })
                })
            });

        let result = !mark_as_black.is_empty() || !mark_as_white.is_empty();

        for coord in mark_as_black {
            grid.mark_cell(coord, State::Black);
        }
        for coord in mark_as_white {
            grid.mark_cell(coord, State::White);
        }

        result
    }
}
