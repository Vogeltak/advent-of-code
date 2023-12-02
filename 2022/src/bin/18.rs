use std::collections::HashSet;

use itertools::Itertools;

fn sides((x, y, z): (i16, i16, i16)) -> [(i16, i16, i16); 6] {
    [
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
}

#[aoc::main(18)]
fn main(input: &str) -> (usize, usize) {
    let cubes = input
        .lines()
        .filter_map(|l| {
            l.split(',')
                .map(|i| i.parse().unwrap())
                .collect_tuple::<(i16, i16, i16)>()
        })
        .collect::<HashSet<_>>();
    let max = cubes.iter().flat_map(|&(x, y, z)| [x, y, z]).max().unwrap() + 1;
    let (mut seen, mut stack) = (HashSet::new(), vec![(0, 0, 0)]);
    while let Some(p) = stack.pop() {
        for (x, y, z) in sides(p) {
            if !cubes.contains(&(x, y, z))
                && !seen.contains(&(x, y, z))
                && [x, y, z].iter().all(|&i| -1 <= i && i <= max)
            {
                seen.insert((x, y, z));
                stack.push((x, y, z));
            }
        }
    }

    let p1 = cubes
        .iter()
        .flat_map(|&p| sides(p))
        .filter(|s| !cubes.contains(s))
        .count();

    let p2 = cubes
        .iter()
        .flat_map(|&p| sides(p))
        .filter(|s| seen.contains(s))
        .count();

    (p1, p2)
}
