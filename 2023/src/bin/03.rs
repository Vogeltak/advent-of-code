use std::collections::{HashMap, HashSet};

#[aoc::main(03)]
fn main(input: &str) -> (u32, u64) {
    let schematic = input.lines().map(str::as_bytes).collect::<Vec<_>>();

    let mut candidates: HashSet<(u8, u8)> = HashSet::new();
    let mut symbols: HashMap<(u8, u8, u8), HashSet<(u8, u8)>> = HashMap::new();
    for (r, row) in schematic.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if cell.is_ascii_digit() || cell == &b'.' {
                continue;
            }

            for (dr, dc) in [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                let (rr, mut cc) = ((r as i32 + dr) as usize, (c as i32 + dc) as usize);
                let Some(x) = schematic.get(rr).and_then(|l| l.get(cc)) else {
                    continue;
                };
                if !x.is_ascii_digit() {
                    continue;
                }

                loop {
                    if cc == 0 {
                        break;
                    }
                    let new_cc = (cc as i32 - 1) as usize;
                    let Some(prev_x) = schematic.get(rr).and_then(|l| l.get(new_cc)) else {
                        break;
                    };
                    if prev_x.is_ascii_digit() {
                        cc = new_cc;
                    } else {
                        break;
                    }
                }

                candidates.insert((rr as u8, cc as u8));
                symbols
                    .entry((r as u8, c as u8, *cell))
                    .or_insert(HashSet::new())
                    .insert((rr as u8, cc as u8));
            }
        }
    }

    let part_numbers: HashMap<_, _> = candidates
        .iter()
        .map(|(r, c)| {
            let mut ci = c.clone();
            let mut digits = vec![];
            while let Some(x) = schematic.get(*r as usize).and_then(|l| l.get(ci as usize)) {
                match (*x as char).to_digit(10) {
                    Some(n) => {
                        digits.push(n);
                        ci += 1;
                    }
                    None => break,
                }
            }
            ((*r, *c), digits.iter().fold(0, |acc, x| acc * 10 + x))
        })
        .collect();

    let p1 = part_numbers.values().sum();

    println!("Found {} symbols", symbols.len());

    let p2 = symbols
        .iter()
        .filter(|((_, _, s), v)| *s == b'*' && v.len() == 2)
        .map(|(_, v)| {
            v.iter()
                .map(|part| *part_numbers.get(part).unwrap() as u64)
                .product::<u64>()
        })
        .sum();

    (p1, p2)
}
