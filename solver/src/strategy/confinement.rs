use crate::grid::State;
use crate::strategy::{MarkSet, Strategy, StrategyResult};
use crate::Grid;

pub struct Confinement;

impl Strategy for Confinement {
    fn name(&self) -> &str {
        "Confinement"
    }

    fn apply(&self, grid: &mut Grid) -> StrategyResult {
        let mut mark_set = MarkSet::new();

        grid.iter()
            .filter(|(_, cell)| cell.state.is_none())
            .try_for_each(|(coord, _)| {
                grid.regions_iter().try_for_each(|(region_id, region)| {
                    if grid.is_region_confined(region_id, [coord])? {
                        mark_set.insert(coord, region.state);
                    }
                    Ok(())
                })
            })?;

        grid.regions_iter()
            .filter(|(_, region)| matches!(region.state, State::Numbered(number) if region.len() < number))
            .try_for_each(|(region_id, region)| {
                region.unknowns.iter().try_for_each(|&coord| {
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
                        .try_for_each(|(other_region_id, _)| {
                            if grid.is_region_confined(other_region_id, assume_visited.iter().copied())? {
                                mark_set.insert(coord, State::Black);
                            }
                            Ok(())
                        })
                })
            })?;

        mark_set.apply(grid)
    }
}

#[cfg(test)]
mod tests {
    use super::Confinement;
    use crate::strategy::test_strategy;

    test_strategy!(
        test_confinement_numbered,
        Confinement,
        "4.\n..",
        Some("4W\nWW")
    );
}
