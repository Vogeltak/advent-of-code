use grid::Grid;
use itertools::Itertools;

#[derive(Debug)]
struct Pattern {
    grid: Grid<u8>,
}

impl From<&str> for Pattern {
    fn from(value: &str) -> Self {
        let mut grid = Grid::new(0, 0);
        value
            .lines()
            .map(|l| l.as_bytes())
            .for_each(|row| grid.push_row(row.to_vec()));

        Self { grid }
    }
}

impl Pattern {
    fn summarize(&self, smudges: bool) -> usize {
        match self.find_reflection(smudges).unwrap() {
            Reflection::Col(i, _) => i + 1,
            Reflection::Row(i, _) => (i + 1) * 100,
        }
    }

    fn find_reflection(&self, smudges: bool) -> Option<Reflection> {
        // Find all column reflections
        let mut candidates = (0..self.grid.cols())
            .tuple_windows()
            .map(|(a, b)| Reflection::Col(a, b))
            .filter(|r| self.is_correct_reflection(r, smudges))
            .collect_vec();

        // Find all row reflections
        candidates.extend(
            (0..self.grid.rows())
                .tuple_windows()
                .map(|(a, b)| Reflection::Row(a, b))
                .filter(|r| self.is_correct_reflection(r, smudges)),
        );

        if candidates.len() != 1 {
            println!(
                "unexpected number of reflections ({}) found for {:#?}",
                candidates.len(),
                self.grid
            );
        }

        candidates.pop()
    }

    fn is_correct_reflection(&self, refl: &Reflection, smudges: bool) -> bool {
        let (before, after) = match refl {
            Reflection::Col(a, b) => (
                (0..=*a)
                    .map(|i| self.grid.iter_col(i).collect_vec())
                    .collect_vec(),
                (*b..self.grid.cols())
                    .map(|i| self.grid.iter_col(i).collect_vec())
                    .collect_vec(),
            ),
            Reflection::Row(a, b) => (
                (0..=*a)
                    .map(|i| self.grid.iter_row(i).collect_vec())
                    .collect_vec(),
                (*b..self.grid.rows())
                    .map(|i| self.grid.iter_row(i).collect_vec())
                    .collect_vec(),
            ),
        };

        // before.iter().rev().zip(after.iter()).all(|(a, b)| a.eq(b))
        let off_by: usize = before
            .iter()
            .rev()
            .zip(after.iter())
            .map(|(a, b)| a.iter().zip(b.iter()).filter(|(x, y)| x != y).count())
            .sum();

        off_by == if smudges { 1 } else { 0 }
    }
}

/// A reflection always points to the last included row/column on its first part.
#[derive(Clone, Debug)]
enum Reflection {
    Col(usize, usize),
    Row(usize, usize),
}

#[aoc::main(13)]
fn main(input: &str) -> (usize, usize) {
    let patterns = input.split("\n\n").map(Pattern::from).collect_vec();

    let p1 = patterns.iter().map(|p| p.summarize(false)).sum();
    let p2 = patterns.iter().map(|p| p.summarize(true)).sum();

    (p1, p2)
}
