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
        self.rules.iter().map(|&(d, s, r)| {
            let mut out = vec![];
            // [x.0                                   x.1)
            //          [s           s+r+1]
            // [BEFORE ][INTER            ][AFTER        )
            let before = (x.0, x.1.min(s));
            let inter = (x.0.max(s), x.1.min(s + r));
            let after = (x.0.max(s + r), x.1);

            if before.1 > before.0 {
                out.push(before);
            }
            if inter.1 > inter.0 {
                out.push((inter.0 - s + d, inter.1 - s + d));
            }
            if after.1 > after.0 {
                out.push(after);
            }
        })
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
        .fold(seeds, |seeds, map| {
            seeds.iter().map(|x| map.convert_single(*x)).collect()
        })
        .iter()
        .cloned()
        .min()
        .unwrap();

    (p1, 0)
}
