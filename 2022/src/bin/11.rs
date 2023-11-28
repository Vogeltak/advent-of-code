use std::rc::Rc;
use std::{
    convert::TryFrom,
    ops::{Add, Mul},
};

use itertools::Itertools;

enum Op {
    Add(u64),
    Mul(u64),
    Special,
}

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    op: Rc<dyn Fn(u64) -> u64>,
    divisor: u64,
    test: Rc<dyn Fn(u64) -> usize>,
    inspections: usize,
}

impl TryFrom<&str> for Monkey {
    type Error = color_eyre::Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // Monkey identifier isn't used from the scope of a single monkey
        let mut lines = value.lines().skip(1);

        let starting_items = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Starting items: ")
            .unwrap()
            .split(", ")
            .map(|i| i.parse::<u64>().unwrap())
            .collect_vec();

        let (operator, operand) = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Operation: new = old ")
            .unwrap()
            .split_once(' ')
            .unwrap();

        let op_enum = match operator {
            "+" => Op::Add(operand.parse::<u64>().unwrap()),
            "*" => match operand.parse::<u64>() {
                Ok(i) => Op::Mul(i),
                Err(_e) => Op::Special,
            },
            _ => unreachable!(),
        };

        let op = Rc::new(move |lhs: u64| -> u64 {
            match op_enum {
                Op::Add(x) => lhs.add(x),
                Op::Mul(x) => lhs.mul(x),
                Op::Special => lhs.mul(lhs),
            }
        });

        let divisor: u64 = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Test: divisible by ")
            .unwrap()
            .parse()?;

        let monkey_true = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("If true: throw to monkey ")
            .unwrap()
            .parse()?;

        let monkey_false = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("If false: throw to monkey ")
            .unwrap()
            .parse()?;

        let test = Rc::new(move |item: u64| -> usize {
            match item % divisor {
                0 => monkey_true,
                _ => monkey_false,
            }
        });

        Ok(Self {
            items: starting_items,
            op,
            divisor,
            test,
            inspections: 0,
        })
    }
}

impl Monkey {
    // Return type:
    // Throw item with worry level a to monkey b
    fn inspect_item(&mut self, manage_worry: impl Fn(u64) -> u64) -> Option<(u64, usize)> {
        if let Some(i) = self.items.pop() {
            self.inspections += 1;
            let i = manage_worry((self.op)(i));
            Some((i, (self.test)(i)))
        } else {
            None
        }
    }
}

fn simulate(mut monkeys: Vec<Monkey>, rounds: usize, f: impl Fn(u64) -> u64) -> usize {
    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            while let Some((i, to)) = monkeys[m].inspect_item(&f) {
                monkeys[to].items.push(i);
            }
        }
    }

    monkeys
        .iter()
        .map(|m| m.inspections)
        .sorted()
        .rev()
        .take(2)
        .product()
}

#[aoc::main(11)]
fn main(input: &str) -> (usize, usize) {
    let monkeys = input
        .split("\n\n")
        .map(|m| Monkey::try_from(m).unwrap())
        .collect_vec();

    let sqd = monkeys.iter().map(|m| m.divisor).product::<u64>();

    let p1 = simulate(monkeys.clone(), 20, |i| i / 3);
    let p2 = simulate(monkeys, 10000, |i| i % sqd);

    (p1, p2)
}
