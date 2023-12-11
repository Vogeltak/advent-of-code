use itertools::Itertools;

fn solve(image: &[&[u8]], mut galaxies: Vec<(usize, usize)>, expansion_factor: usize) -> usize {
    // Find all empty rows and columns
    let (rows, cols) = (image.len(), image[0].len());
    let empty_rows = (0..rows).filter(|&r| image[r].iter().all(|&x| x == b'.'));
    let empty_cols = (0..cols).filter(|&c| (0..rows).all(|r| image[r][c] == b'.'));

    // Account for the expanded universe in our snapshot
    empty_rows.rev().for_each(|r| {
        galaxies.iter_mut().for_each(|g| {
            if g.0 > r {
                g.0 += expansion_factor - 1
            }
        })
    });
    for c in empty_cols.rev() {
        for g in &mut galaxies {
            if g.1 > c {
                g.1 += expansion_factor - 1;
            }
        }
    }

    // Find the shortest path between all galaxy pairs
    galaxies
        .iter()
        .tuple_combinations()
        .map(|(a, b)| a.0.abs_diff(b.0) + a.1.abs_diff(b.1))
        .sum()
}

#[aoc::main(11)]
fn main(input: &str) -> (usize, usize) {
    let image = input.lines().map(|l| l.as_bytes()).collect_vec();
    let galaxies = (0..image.len())
        .cartesian_product(0..image[0].len())
        .filter(|&(r, c)| image[r][c] == b'#')
        .collect_vec();

    let p1 = solve(image.as_slice(), galaxies.clone(), 2);
    let p2 = solve(image.as_slice(), galaxies, 1_000_000);

    (p1, p2)
}
