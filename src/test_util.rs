use crate::{Coord, Grid, State};

pub(crate) fn get_regions(grid: &Grid) -> Vec<(State, Vec<Coord>)> {
    let mut regions = grid
        .regions()
        .map(|region| {
            let state = region.state;
            let mut coords = region.coords.clone();
            coords.sort();
            (state, coords)
        })
        .collect::<Vec<_>>();
    regions.sort();
    regions
}
