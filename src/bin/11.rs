use std::{convert::TryFrom, ops::{Add, Mul}};

use itertools::Itertools;

struct Monkey {
    items: Vec<i32>,
    op: Box<dyn Fn(i32) -> i32>,
    test: Box<dyn Fn(i32) -> usize>,
    inspections: usize,
}

enum Op {
    Add(i32),
    Mul(i32),
    Special,
}

impl TryFrom<&str> for Monkey {
    type Error = color_eyre::Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // Monkey identifier isn't used from the scope of a single monkey
        let mut lines = value.lines().skip(1);

        let starting_items = lines
            .next().unwrap()
            .trim()
            .strip_prefix("Starting items: ").unwrap()
            .split(", ")
            .map(|i| i.parse::<i32>().unwrap())
            .collect_vec();

        let (operator, operand) = lines
            .next().unwrap()
            .trim()
            .strip_prefix("Operation: new = old ").unwrap()
            .split_once(' ').unwrap();

        let op_enum = match operator {
            "+" => Op::Add(operand.parse::<i32>().unwrap()),
            "*" => match operand.parse::<i32>() { 
                Ok(i) => Op::Mul(i),
                Err(_e) => Op::Special,
            }
            _ => unreachable!(),
        };

        let op = Box::new(move |lhs: i32| -> i32 {
            match op_enum {
                Op::Add(x) => lhs.add(x),
                Op::Mul(x) => lhs.mul(x),
                Op::Special => lhs.mul(lhs),
            }
        });

        let divisor: i32 = lines
            .next().unwrap()
            .trim()
            .strip_prefix("Test: divisible by ").unwrap()
            .parse()?;
        
        let monkey_true = lines
            .next().unwrap()
            .trim()
            .strip_prefix("If true: throw to monkey ").unwrap()
            .parse()?;

        let monkey_false = lines
            .next().unwrap()
            .trim()
            .strip_prefix("If false: throw to monkey ").unwrap()
            .parse()?;

        let test = Box::new(move |item: i32| -> usize {
            match item % divisor {
                0 => monkey_true,
                _ => monkey_false,
            }
        });

        Ok(Self {
            items: starting_items,
            op,
            test,
            inspections: 0,
        })
    }
}

impl Monkey {
    // Return type:
    // Throw item with worry level a to monkey b
    fn inspect_item(&mut self) -> Option<(i32, usize)> {
        if let Some(i) = self.items.pop() {
            self.inspections += 1;
            let i = (self.op)(i) / 3;
            Some((i, (self.test)(i)))
        } else {
            None
        }
    }
}

#[aoc::main(11)]
fn main(input: &str) -> (usize, usize) {
    let mut monkeys = input
        .split("\n\n")
        .map(|m| Monkey::try_from(m).unwrap())
        .collect_vec();

    // Simulate all 20 rounds
    for _ in 0..20 {
        for m in 0..monkeys.len() {
            while let Some((i, to)) = monkeys[m].inspect_item() {
                monkeys[to].items.push(i);
            }
        }
    }

    let p1 = monkeys
        .iter()
        .map(|m| m.inspections)
        .sorted()
        .rev()
        .take(2)
        .product();

    (p1, 0)
}