use std::collections::{HashSet, VecDeque};

use crate::grid::Region;
use crate::{Coord, Grid, RegionID, SolverError, State};

impl Grid {
    /// Check if a cell is unreachable by a white/numbered region.
    pub fn is_cell_unreachable(
        &self,
        coord: Coord,
        assume_black: impl IntoIterator<Item = Coord>,
    ) -> bool {
        if self.cell(coord).state != None {
            return false;
        }

        // The maximum size a white region can be if we still want to be able to join it with a numbered region
        let max_white_region_len = self
            .regions()
            .filter_map(|region| {
                if let State::Numbered(max_region_len) = region.state {
                    Some(max_region_len.saturating_sub(region.len()))
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
            for adj_coord in self.valid_neighbors(cur_coord) {
                if let Some(adj_region_id) = self.cell(adj_coord).region {
                    match self.region(adj_region_id).unwrap().state {
                        State::Numbered(_) => {
                            adj_numbered_regions.insert(adj_region_id);
                        }
                        State::White => {
                            adj_white_regions.insert(adj_region_id);
                        }
                        State::Black => {}
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
            let mut extra_region_len = depth;
            for &region_id in &adj_white_regions {
                extra_region_len += self.region(region_id).unwrap().len();
            }

            if !adj_numbered_regions.is_empty() {
                // Path reached a numbered region
                let region_id = *adj_numbered_regions.iter().next().unwrap();
                let region = self.region(region_id).unwrap();
                if let State::Numbered(max_region_len) = region.state {
                    // Could the current path be fused to the numbered region?
                    if extra_region_len + region.len() <= max_region_len {
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
                if extra_region_len + 1 <= max_white_region_len {
                    // Current path might be reachable through the adjacent white regions
                    return false;
                } else {
                    // Current cell is not reachable
                    continue;
                }
            }

            for adj_coord in self.valid_unknown_neighbors(cur_coord) {
                if !explored.contains(&adj_coord) {
                    explored.insert(adj_coord);
                    queue.push_back((adj_coord, depth + 1));
                }
            }
        }

        true
    }

    pub(crate) fn is_region_confined(
        &self,
        region_id: RegionID,
        assume_visited: impl IntoIterator<Item = Coord>,
    ) -> Result<bool, SolverError> {
        let region = self.region(region_id).unwrap();

        let mut open = VecDeque::from_iter(region.unknowns.iter().copied());

        let mut visited = HashSet::new();
        visited.extend(region.coords.iter().copied());
        visited.extend(assume_visited);

        // Set of cells that may connect to the region
        let mut closed = HashSet::new();
        closed.extend(region.coords.iter().copied());

        while let Some(coord) = open.pop_front() {
            if !visited.insert(coord) {
                continue;
            }

            if !self.is_region_like_incomplete(region.state, closed.len()) {
                return Ok(false);
            }

            let other_region = self
                .cell(coord)
                .region
                .and_then(|region_id| self.region(region_id));

            match region.state {
                State::Numbered(_) => match other_region.map(|region| region.state) {
                    Some(State::Numbered(_)) => {
                        // Two numbered regions should never be adjacent
                        return Err(SolverError::Contradiction);
                    }
                    Some(State::White) => {
                        // Consume the white region
                    }
                    Some(State::Black) => {
                        // Numbered region cannot consume black regions
                        continue;
                    }
                    None => {
                        if self
                            .valid_neighbors(coord)
                            .filter_map(|adj_coord| self.cell(adj_coord).region)
                            .filter(|adj_region_id| {
                                self.region(*adj_region_id)
                                    .map(|adj_region| adj_region.state.is_numbered())
                                    .unwrap_or(false)
                            })
                            .any(|adj_region_id| region_id != adj_region_id)
                        {
                            // Unknown cell is adjacent to another numbered region, so it cannot be consumed
                            continue;
                        }

                        // Consume the unknown cell otherwise
                    }
                },
                State::White => match other_region.map(|region| region.state) {
                    Some(State::Numbered(_)) => {
                        // White region reached a numbered region, so it is not confined
                        return Ok(false);
                    }
                    Some(State::White) | None => {
                        // Consume the white region / unknown cell
                    }
                    Some(State::Black) => {
                        // White region cannot consume black regions
                        continue;
                    }
                },
                State::Black => match other_region.map(|region| region.state) {
                    Some(State::Numbered(_) | State::White) => {
                        // Black region cannot consume numbered/white regions
                        continue;
                    }
                    Some(State::Black) | None => {
                        // Consume the black region / unknown cell
                    }
                },
            }

            // Consume the region/cell
            if let Some(other_region) = other_region {
                closed.extend(other_region.coords.iter().copied());
                visited.extend(other_region.coords.iter().copied());
                open.extend(other_region.unknowns.iter().copied());
            } else {
                closed.insert(coord);
                visited.insert(coord);
                open.extend(self.valid_neighbors(coord));
            }
        }

        Ok(self.is_region_like_incomplete(region.state, closed.len()))
    }

    pub(crate) fn is_region_incomplete(&self, region: &Region) -> bool {
        self.is_region_like_incomplete(region.state, region.len())
    }

    pub(crate) fn is_region_like_incomplete(&self, region_state: State, region_len: usize) -> bool {
        match region_state {
            State::White => true,
            State::Black => region_len < self.total_black_cells,
            State::Numbered(number) => region_len < number,
        }
    }

    /// Returns `true` if the region is larger than should be possible.
    pub(crate) fn is_region_overfilled(&self, region: &Region) -> bool {
        match region.state {
            State::White => false,
            State::Black => region.len() > self.total_black_cells,
            State::Numbered(number) => region.len() > number,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::grid::RegionID;
    use crate::{Coord, Grid};

    #[test]
    fn test_is_region_confined() {
        let grid = Grid::from_str("4.\n..").unwrap();
        let region_id = RegionID(0);
        assert_eq!(grid.is_region_confined(region_id, []), Ok(false));
        assert_eq!(
            grid.is_region_confined(region_id, [Coord::new(0, 1)]),
            Ok(true)
        );
    }
}
