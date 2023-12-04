use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
struct Card {
    id: u8,
    winning: HashSet<u8>,
    numbers: HashSet<u8>,
}

impl Card {
    fn process(&self) -> usize {
        self.numbers.intersection(&self.winning).count()
    }
}

#[aoc::main(04)]
fn main(input: &str) -> (i32, usize) {
    let cards: Vec<Card> = input
        .lines()
        .map(|l| l.split_whitespace().filter_map(|x| x.parse::<u8>().ok()))
        .enumerate()
        .map(|(i, l)| Card {
            id: i as u8 + 1,
            winning: l.clone().take(10).collect(),
            numbers: l.skip(10).take(25).collect(),
        })
        .collect();

    let p1 = cards
        .iter()
        .map(|c| c.process())
        .filter(|n| n != &0)
        .map(|n| 2_i32.pow((n - 1) as u32))
        .sum();

    let mut wallet: HashMap<usize, usize> = (1..cards.len() + 1).map(|i| (i, 1)).collect();

    (1..cards.len() + 1).for_each(|i| {
        // Find a card to process
        let c = cards.iter().find(|c| c.id == i as u8).unwrap();
        // Determine #copies it wins
        let win = c.process();
        // Determine #copies of this card
        // let copies = cards.iter().filter(|c| c.id == i as u8).count();
        let copies = wallet.get(&i).unwrap().clone();
        // Add new copies to the list
        (i + 1..cards.len() + 1).take(win).for_each(|j| {
            *wallet.entry(j).or_insert(1) += copies;
        })
    });

    let p2 = wallet.values().sum();

    (p1, p2)
}
