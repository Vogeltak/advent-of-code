use std::cmp::{Ordering, max};

use itertools::Itertools;
use serde_json::Value;

fn compare(left: &Value, right: &Value) -> Ordering {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => l.as_u64().unwrap().cmp(&r.as_u64().unwrap()),
        (Value::Array(l), Value::Array(r)) => {
            for i in 0..max(l.len(), r.len()) {
                match (l.get(i), r.get(i)) {
                    (None, _) => return Ordering::Less,
                    (_, None) => return Ordering::Greater,
                    (Some(x), Some(y)) => match compare(x, y) {
                        Ordering::Equal => {},
                        c => return c,
                    }
                }
            }
            Ordering::Equal
        },
        (Value::Number(_), Value::Array(_)) => compare(&Value::Array(vec![left.clone()]), right),
        (Value::Array(_), Value::Number(_)) => compare(left, &Value::Array(vec![right.clone()])),
        _ => unreachable!(),
    }
}

#[aoc::main(13)]
fn main(input: &str) -> (usize, usize) {
    let pairs = input
        .split("\n\n")
        .map(|pair| pair.lines().map(|p| serde_json::from_str::<Value>(p).unwrap()).collect_vec())
        .collect_vec();

    let p1 = pairs
        .iter()
        .positions(|p| compare(&p[0], &p[1]) == Ordering::Less)
        .map(|i| i + 1)
        .sum();
    
    let dividers = [
        serde_json::from_str::<Value>("[[2]]").unwrap(),
        serde_json::from_str::<Value>("[[6]]").unwrap(),
    ];

    let mut pairs = pairs
        .iter()
        .flatten()
        .collect_vec();

    pairs.extend(dividers.iter());
    pairs.sort_by(|a, b| compare(a, b));

    let p2 = pairs
        .iter()
        .positions(|p| dividers.contains(p))
        .map(|i| i + 1)
        .product();

    (p1, p2)
}