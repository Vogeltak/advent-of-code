use itertools::Itertools;

#[derive(Debug)]
struct ClawMachine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

impl ClawMachine {
    // nr_a * a.0 + nr_b * b.0 = prize.0
    // nr_a * a.1 + nr_b * b.1 = prize.1
    // ---
    // nr_a = (prize.0 - nr_b * b.0) / a.0
    // (prize.0 - nr_b * b.0) / a.0 * a.1 + nr_b * b.1 = prize.1
    // (prize.0 * a.1 - nr_b * b.0 * a.1) / a.0 + nr_b * b.1 = prize.1
    // (prize.0 * a.1 - nr_b * b.0 * a.1) + nr_b * b.1 * a.0 = prize.1 * a.0
    // nr_b * b.1 * a.0 - nr_b * b.0 * a.1 = prize.1 * a.0 - prize.0 * a.1
    // nr_b * (b.1 * a.0 - b.0 * a.1) = prize.1 * a.0 - prize.0 * a.1
    // nr_b = (prize.1 * a.0 - prize.0 * a.1) / (b.1 * a.0 - b.0 * a.1)
    fn solve(&self, prize_offset: Option<i64>) -> i64 {
        let dz = prize_offset.unwrap_or_default();
        let (z0, z1) = (self.prize.0 + dz, self.prize.1 + dz);

        let nr_b = (z1 * self.a.0 - z0 * self.a.1) / (self.b.1 * self.a.0 - self.b.0 * self.a.1);
        let nr_a = (z0 - nr_b * self.b.0) / self.a.0;

        if (
            nr_a * self.a.0 + nr_b * self.b.0,
            nr_a * self.a.1 + nr_b * self.b.1,
        ) != ((z0, z1))
        {
            return 0;
        }

        nr_a * 3 + nr_b
    }
}

impl From<&str> for ClawMachine {
    fn from(value: &str) -> Self {
        let (x1, y1, x2, y2, prizex, prizey) = value
            .split(|c: char| !c.is_ascii_digit())
            .filter(|w| !w.is_empty())
            .map(|w| w.parse().unwrap())
            .collect_tuple()
            .unwrap();

        Self {
            a: (x1, y1),
            b: (x2, y2),
            prize: (prizex, prizey),
        }
    }
}

#[aoc::main(13)]
fn main(input: &str) -> (usize, usize) {
    let claw_machines = input.split("\n\n").map(ClawMachine::from).collect_vec();

    let p1 = claw_machines.iter().map(|m| m.solve(None) as usize).sum();
    let p2 = claw_machines
        .iter()
        .map(|m| m.solve(Some(10000000000000)) as usize)
        .sum();

    (p1, p2)
}
