use itertools::Itertools;

#[derive(Debug)]
struct Map {
    rules: Vec<(u64, u64, u64)>,
}

impl Map {
    fn convert_single(&self, x: u64) -> u64 {
        self.rules
            .iter()
            .find(|&&(_, s, r)| (s..s + r).contains(&x))
            .map(|(d, s, _)| d + x - s)
            .unwrap_or(x)
    }

    fn convert_range(&self, x: (u64, u64)) -> Vec<(u64, u64)> {
        let mut mapped = vec![];
        let mut unmapped = vec![x];
        for (d, s, r) in self.rules.iter() {
            let mut m = vec![];
            for (start, end) in unmapped {
                // [start                                 end)
                //          [s           s+r+1]
                // [BEFORE ][INTER            ][AFTER        )
                let before = (start, end.min(*s));
                let inter = (start.max(*s), end.min(s + r));
                let after = (start.max(s + r), end);

                if before.1 > before.0 {
                    m.push(before);
                }
                if inter.1 > inter.0 {
                    mapped.push((inter.0 - s + d, inter.1 - s + d));
                }
                if after.1 > after.0 {
                    m.push(after);
                }
            }
            unmapped = m;
        }

        mapped.extend(unmapped);
        mapped
    }
}

impl FromIterator<(u64, u64, u64)> for Map {
    fn from_iter<T: IntoIterator<Item = (u64, u64, u64)>>(iter: T) -> Self {
        let rules = iter.into_iter().collect();
        Map { rules }
    }
}

#[aoc::main(05)]
fn main(input: &str) -> (u64, u64) {
    let (seeds, rest) = input.split_once("\n\n").unwrap();
    let seeds = seeds
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let layers = rest
        .split("\n\n")
        .map(|s| {
            s.lines()
                .skip(1)
                .map(|l| {
                    l.split_whitespace()
                        .map(|s| s.parse::<u64>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect::<Map>()
        })
        .collect::<Vec<_>>();

    let p1 = layers
        .iter()
        .fold(seeds.clone(), |seeds, map| {
            seeds.iter().map(|x| map.convert_single(*x)).collect()
        })
        .iter()
        .cloned()
        .min()
        .unwrap();

    let seeds = seeds.iter().cloned().tuples::<(_, _)>().collect_vec();

    let p2 = layers
        .iter()
        .fold(seeds, |seeds, map| {
            seeds
                .iter()
                .map(|x| map.convert_range(*x))
                .flatten()
                .collect_vec()
        })
        .iter()
        .map(|r| r.0)
        .min()
        .unwrap();

    (p1, p2)
}
