use itertools::Itertools;

fn is_correct(update: &[u8], rules: &[(u8, u8)]) -> bool {
    let relevant_rules = rules
        .iter()
        .filter(|(a, b)| update.contains(a) && update.contains(b))
        .collect_vec();

    for i in 0..update.len() {
        if relevant_rules
            .iter()
            .filter(|(a, _)| *a == update[i])
            .any(|(_, b)| update[..i].contains(b))
        {
            return false;
        }
    }

    true
}

#[inline]
fn get_middle_page(update: &[u8]) -> u8 {
    update[update.len() / 2]
}

fn get_correct_order(update: &[u8], rules: &[(u8, u8)]) -> Vec<u8> {
    let relevant_rules = rules
        .iter()
        .filter(|(a, b)| update.contains(a) && update.contains(b))
        .collect_vec();

    let mut sorted_update = update.to_vec();
    sorted_update.sort_by(|a, b| {
        relevant_rules
            .iter()
            // Find all ordering rules about b, meaning, all pages that must
            // come *after* b
            .filter(|(aa, _)| aa == b)
            // If any of the ordering rules say that b must be before a,
            // we return [`Ordering::Equal`], if none of the rules say this,
            // then we return [`Ordering::Less`].
            .any(|(_, bb)| bb == a)
            .cmp(&true)
    });

    sorted_update
}

#[aoc::main(05)]
fn main(input: &str) -> (usize, usize) {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|l| {
            let (a, b) = l.split_once('|').unwrap();
            (a.parse::<u8>().unwrap(), b.parse::<u8>().unwrap())
        })
        .collect_vec();
    let updates = updates
        .lines()
        .map(|l| l.split(',').map(|i| i.parse::<u8>().unwrap()).collect_vec())
        .collect_vec();

    let p1 = updates
        .iter()
        .filter(|u| is_correct(u, &rules))
        .map(|u| get_middle_page(u) as usize)
        .sum();

    let p2 = updates
        .iter()
        .filter(|u| !is_correct(u, &rules))
        .map(|u| get_correct_order(u, &rules))
        .map(|u| get_middle_page(&u) as usize)
        .sum();

    (p1, p2)
}
