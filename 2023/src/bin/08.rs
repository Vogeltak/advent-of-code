use std::collections::HashMap;

use anyhow::{anyhow, Result};
use gcd::Gcd;
use itertools::Itertools;

#[derive(Clone, Debug)]
enum Instruction {
    Left,
    Right,
}

impl TryFrom<char> for Instruction {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            'L' => Ok(Instruction::Left),
            'R' => Ok(Instruction::Right),
            _ => Err(anyhow!("unsupported instruction")),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct NodeId(String);

#[derive(Clone, Debug)]
struct Node {
    left: NodeId,
    right: NodeId,
}

impl Node {
    fn follow(&self, i: &Instruction) -> NodeId {
        match i {
            Instruction::Left => self.left.clone(),
            Instruction::Right => self.right.clone(),
        }
    }
}

#[aoc::main(08)]
fn main(input: &str) -> (usize, usize) {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();
    let instructions = instructions
        .chars()
        .map(Instruction::try_from)
        .filter_map(Result::ok)
        .collect_vec();

    let nodes: HashMap<NodeId, Node> = nodes
        .lines()
        .map(|l| {
            let (id, node) = l.split_once(" = ").unwrap();
            let (l, r) = node
                .trim_matches(|c| c == '(' || c == ')')
                .split_once(", ")
                .unwrap();

            (
                NodeId(id.to_string()),
                Node {
                    left: NodeId(l.to_string()),
                    right: NodeId(r.to_string()),
                },
            )
        })
        .collect();

    let mut cur = NodeId("AAA".to_string());
    let goal = NodeId("ZZZ".to_string());
    let p1 = instructions
        .iter()
        .cycle()
        .take_while(|&i| {
            cur = nodes.get(&cur).unwrap().follow(i);
            cur != goal
        })
        .count();

    let p2 = nodes
        .keys()
        .filter(|n| n.0.ends_with('A'))
        .map(|n| {
            let mut cur = n.clone();
            instructions
                .iter()
                .cycle()
                .take_while(|&i| {
                    cur = nodes.get(&cur).unwrap().follow(i);
                    !cur.0.ends_with('Z')
                })
                .count()
                + 1
        })
        .fold(1, |acc, x| (acc * x) / acc.gcd(x));

    (p1 + 1, p2)
}
