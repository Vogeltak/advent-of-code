use std::path::PathBuf;

use hashbrown::HashMap;
use itertools::Itertools;

#[aoc::main(07)]
fn main(input: &str) -> (i64, i64) {
    let mut wd = PathBuf::new();
    let mut dirs: HashMap<PathBuf, i64> = HashMap::new();

    for l in input.lines() {
        let cmd = l.trim().split_ascii_whitespace().collect_vec();
        match cmd.len() {
            3 => {
                let dir = cmd[2];
                if dir == ".." {
                    wd.pop();
                } else {
                    wd.push(dir);
                }
            },
            2 => {
                match (cmd[0], cmd[1]) {
                    ("$", "ls") => continue,
                    ("dir", _) => continue,
                    (size, _) => {
                        let size = size.parse::<i64>().unwrap();
                        for dir in wd.as_path().ancestors() {
                            dirs.entry(dir.to_path_buf().clone())
                                .and_modify(|n| *n += size)
                                .or_insert(size);
                        }
                    }
                }
            },
            _ => unreachable!(),
        }
    }

    let p1 = dirs.values()
        .filter(|&&size| size <= 100_000)
        .sum();

    let total_unused = 70_000_000 - dirs[&PathBuf::from("/")];

    let p2 = dirs.values()
        .filter(|&&size| total_unused + size >= 30_000_000) 
        .min()
        .copied()
        .unwrap();

    (p1, p2)
}