use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn bron_kerbosch<'a>(
    g: &'a HashMap<&'a str, HashSet<&'a str>>,
    r: &mut HashSet<&'a str>,
    mut p: HashSet<&'a str>,
    mut x: HashSet<&'a str>,
    cliques: &mut Vec<HashSet<&'a str>>,
) {
    if p.is_empty() {
        if x.is_empty() {
            cliques.push(r.clone());
        }
        return;
    }

    while let Some(n) = p.iter().copied().next() {
        let adj = &g[n];
        let new_p = p.intersection(adj).copied().collect();
        let new_x = x.intersection(adj).copied().collect();
        r.insert(n);
        bron_kerbosch(g, r, new_p, new_x, cliques);
        r.remove(n);
        p.remove(n);
        x.insert(n);
    }
}

#[aoc::main(23)]
fn main(input: &str) -> (usize, String) {
    let mut network = HashMap::<_, HashSet<_>>::new();
    input
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .for_each(|(a, b)| {
            network.entry(a).or_default().insert(b);
            network.entry(b).or_default().insert(a);
        });

    let mut cliques_of_three = HashSet::new();
    for &n1 in network.keys() {
        if !n1.starts_with("t") {
            continue;
        }

        for &n2 in network[n1].iter() {
            for &n3 in network[n2].iter() {
                if network[n3].contains(&n1) {
                    let mut clique = [n1, n2, n3];
                    clique.sort();
                    cliques_of_three.insert(clique);
                }
            }
        }
    }

    let p1 = cliques_of_three.len();
    let mut cliques = vec![];
    bron_kerbosch(
        &network,
        &mut HashSet::new(),
        network.keys().copied().collect(),
        HashSet::new(),
        &mut cliques,
    );
    let p2 = cliques
        .iter()
        .max_by_key(|c| c.len())
        .unwrap()
        .iter()
        .sorted()
        .join(",");

    (p1, p2)
}
