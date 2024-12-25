use itertools::Itertools;

trait ToHeights {
    fn to_heights(self) -> Vec<usize>;
}

impl ToHeights for &str {
    fn to_heights(self) -> Vec<usize> {
        (0..5)
            .map(|i| {
                self.lines()
                    .filter_map(|l| l.chars().nth(i))
                    .filter(|c| *c == '#')
                    .count()
            })
            .collect_vec()
    }
}

trait Overlaps {
    fn overlaps(&self, other: &[usize]) -> bool;
}

impl Overlaps for Vec<usize> {
    fn overlaps(&self, other: &[usize]) -> bool {
        for i in 0..self.len() {
            if self[i] + other[i] > 7 {
                return true;
            }
        }

        false
    }
}

#[aoc::main(25)]
fn main(input: &str) -> (usize, usize) {
    let (locks, keys): (Vec<_>, Vec<_>) = input
        .split("\n\n")
        .partition(|s| s.lines().last().unwrap() == ".....");

    let locks = locks
        .into_iter()
        .map(|lock| lock.to_heights())
        .collect_vec();

    let keys = keys.into_iter().map(|lock| lock.to_heights()).collect_vec();

    let p1 = locks
        .iter()
        .cartesian_product(keys.iter())
        .filter(|(lock, key)| !lock.overlaps(&key))
        .count();

    (p1, 0)
}
