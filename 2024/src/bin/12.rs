use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Plot {
    row: usize,
    column: usize,
    perimeter: usize,
}

impl PartialEq<(usize, usize)> for Plot {
    fn eq(&self, other: &(usize, usize)) -> bool {
        self.row == other.0 && self.column == other.1
    }
}

fn price(regions: &HashMap<u8, Vec<HashSet<Plot>>>) -> usize {
    regions
        .iter()
        .map(|(_, regions)| {
            regions
                .iter()
                .map(|plots| {
                    plots
                        .iter()
                        .map(|p| p.perimeter * plots.len())
                        .sum::<usize>()
                })
                .sum::<usize>()
        })
        .sum()
}

fn bulk_price(garden: &[&[u8]], regions: &HashMap<u8, Vec<HashSet<Plot>>>) -> usize {
    regions
        .iter()
        // Calculate price per region
        .map(|(plant, plant_regions)| {
            plant_regions
                .iter()
                .map(|plots| {
                    plots
                        .iter()
                        // Determine number of corners for every plot
                        .map(|p| {
                            [(-1, 0), (0, 1), (1, 0), (0, -1), (-1, 0)]
                                .into_iter()
                                .tuple_windows()
                                .filter(|(d1, d2)| {
                                    let n1 = garden.get((p.row as isize + d1.0) as usize).and_then(
                                        |row| row.get((p.column as isize + d1.1) as usize),
                                    );
                                    let n2 = garden.get((p.row as isize + d2.0) as usize).and_then(
                                        |row| row.get((p.column as isize + d2.1) as usize),
                                    );
                                    match (n1, n2) {
                                        (Some(p1), Some(p2)) if p1 == plant && p1 == p2 => {
                                            // Find out if we're an inner corner.
                                            // I.e., if d1+d2 points to !plant plot
                                            let d = (d1.0 + d2.0, d1.1 + d2.1);
                                            let n3 = garden
                                                .get((p.row as isize + d.0) as usize)
                                                .and_then(|row| {
                                                    row.get((p.column as isize + d.1) as usize)
                                                });
                                            match n3 {
                                                Some(p) => p != plant,
                                                None => unreachable!(),
                                            }
                                        }
                                        // Outside corner: both of the plots are not of our plant type
                                        (Some(p1), Some(p2)) => p1 != plant && p2 != plant,
                                        // We're at the border, but if our neighbor is us we're still not a corner
                                        (Some(p1), None) => p1 != plant,
                                        (None, Some(p1)) => p1 != plant,
                                        // We're the corner-corner, so definitely a corner
                                        (None, None) => true,
                                    }
                                })
                                .count()
                        })
                        .sum::<usize>()
                        * plots.len()
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

#[aoc::main(12)]
fn main(input: &str) -> (usize, usize) {
    // 140x140, area is 19,600
    let garden = input.lines().map(|l| l.as_bytes()).collect_vec();

    let mut regions = HashMap::<u8, Vec<HashSet<Plot>>>::new();

    // Loop over every plot
    // - let's say it's plant type A
    // - look at plots on all 4 sides
    // - perimeter is #plots != A
    // - for all plots == A -> find their existing region (HashSet)
    // - if there are > 1 regions, the current plot makes them join
    // - if one of the neighbors is not in a region yet, skip it (we'll get to it later)

    for r in 0..garden.len() {
        for c in 0..garden[0].len() {
            let plant = garden[r][c];
            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .map(|(dr, dc)| (r + dr as usize, c + dc as usize))
                .filter(|(rr, cc)| {
                    garden
                        .get(*rr)
                        .and_then(|row| row.get(*cc))
                        .is_some_and(|neighbor_plant| neighbor_plant == &plant)
                })
                .collect_vec();
            let perimeter = 4 - neighbors.len();
            let plant_regions = regions.entry(plant).or_default();
            let neighbor_regions = plant_regions
                .iter()
                .enumerate()
                .filter(|(_, region)| {
                    neighbors
                        .iter()
                        .any(|n| region.iter().any(|plot| plot == n))
                })
                .map(|(i, _)| i)
                .collect_vec();
            let target_region = match neighbor_regions.len() {
                // Join regions
                2.. => {
                    let mut joined_region = HashSet::new();
                    neighbor_regions
                        .into_iter()
                        .rev()
                        .map(|i| plant_regions.remove(i))
                        .for_each(|r| joined_region.extend(r));
                    plant_regions.push(joined_region);
                    plant_regions
                        .last_mut()
                        .expect("should have a last element because we just pushed")
                }
                // There's just one neighboring region for this plant type
                1 => plant_regions
                    .get_mut(neighbor_regions[0])
                    .expect("by invariant of match arm"),
                // There are no neighboring regions, so we create a new one
                0 => {
                    plant_regions.push(HashSet::new());
                    plant_regions
                        .last_mut()
                        .expect("should have a last element since we just pushed")
                }
            };
            // Insert the current plot into its region
            target_region.insert(Plot {
                row: r,
                column: c,
                perimeter,
            });
        }
    }

    let p1 = price(&regions);
    let p2 = bulk_price(&garden, &regions);

    (p1, p2)
}
