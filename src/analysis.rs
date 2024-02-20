use std::collections::{HashSet, VecDeque};

use by_address::ByAddress;

use crate::{Coord, Grid, State};

/// Check if a cell is unreachable by a white/numbered region.
pub fn is_cell_unreachable(
    grid: &Grid,
    coord: Coord,
    assume_black: impl IntoIterator<Item = Coord>,
) -> bool {
    let index = grid.coord_to_index(coord);
    if grid.cells[index].state != State::Unknown {
        return false;
    }

    // The maximum size a white region can be if we still want to be able to join it with a numbered region
    let maximum_white_region_size = grid
        .regions
        .iter()
        .filter_map(|region_ptr| {
            if let State::Numbered(max_region_size) = region_ptr.borrow().state {
                Some(max_region_size - region_ptr.borrow().coords.len())
            } else {
                None
            }
        })
        .max()
        .unwrap_or(0);

    let mut explored = HashSet::from([coord]);
    explored.extend(assume_black);
    let mut queue = VecDeque::from([(coord, 1)]);

    while let Some((cur_coord, depth)) = queue.pop_front() {
        // Collect numbered/white regions adjacent to the current cell
        let mut adj_numbered_regions = HashSet::new();
        let mut adj_white_regions = HashSet::new();
        for adj_coord in grid.valid_neighbors(cur_coord) {
            let adj_index = grid.coord_to_index(adj_coord);
            if let Some(adj_region_ptr) = &grid.cells[adj_index].region {
                match adj_region_ptr.borrow().state {
                    State::Numbered(_) => {
                        adj_numbered_regions.insert(ByAddress(adj_region_ptr.clone()));
                    }
                    State::White => {
                        adj_white_regions.insert(ByAddress(adj_region_ptr.clone()));
                    }
                    State::Black => {}
                    State::Unknown => unreachable!(),
                };
            }
        }

        if adj_numbered_regions.len() >= 2 {
            // Cannot join two numbered regions
            // -> current cell is not reachable
            continue;
        }

        // Determine minimum size of the region formed by fusing:
        //  1. the adjacent white regions,
        //  2. the path from the `coord` to `cur_coord`.
        let mut min_region_size = depth;
        for region_ptr in &adj_white_regions {
            let region = region_ptr.0.borrow();
            min_region_size += region.coords.len();
        }

        if !adj_numbered_regions.is_empty() {
            // Path reached a numbered region
            let region = adj_numbered_regions.iter().next().unwrap().0.borrow();
            if let State::Numbered(max_region_size) = region.state {
                // Could the current path be fused to the numbered region?
                if min_region_size + region.coords.len() <= max_region_size {
                    // Current path might be reachable from the numbered region
                    return false;
                } else {
                    // Current cell is not reachable
                    continue;
                }
            } else {
                unreachable!();
            }
        }

        if !adj_white_regions.is_empty() {
            // Path reached a white region
            // Could the region formed by fusing the current path to the
            // adjacent white regions ever be connected to a numbered region?
            if min_region_size + 1 <= maximum_white_region_size {
                // Current path might be reachable through the adjacent white regions
                return false;
            } else {
                // Current cell is not reachable
                continue;
            }
        }

        for adj_coord in grid.valid_unknown_neighbors(cur_coord) {
            if !explored.contains(&adj_coord) {
                explored.insert(adj_coord);
                queue.push_back((adj_coord, depth + 1));
            }
        }
    }

    true
}
