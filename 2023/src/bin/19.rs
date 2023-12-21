use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Workflow {
    id: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn process(&self, part: &Part) -> Dest {
        self.rules.iter().find_map(|r| r.apply(&part)).unwrap()
    }
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        let (id, rest) = value.split_once('{').unwrap();
        // rest = "rule,...,rule}"
        let rules = rest[..rest.len() - 1]
            .split(',')
            .map(Rule::from)
            .collect_vec();

        Self {
            id: id.to_string(),
            rules,
        }
    }
}

#[derive(Debug)]
struct Rule {
    operand: Option<String>,
    lt: Option<u32>,
    gt: Option<u32>,
    dst: Dest,
}

impl Rule {
    fn apply(&self, part: &Part) -> Option<Dest> {
        let op = match self.operand.as_ref().and_then(|s| Some(s.as_str())) {
            Some("x") => part.x,
            Some("m") => part.m,
            Some("a") => part.a,
            Some("s") => part.s,
            None => return Some(self.dst.clone()),
            _ => unreachable!(),
        };

        match (self.lt, self.gt) {
            (Some(op2), None) => (op < op2).then_some(self.dst.clone()),
            (None, Some(op2)) => (op > op2).then_some(self.dst.clone()),
            _ => None,
        }
    }
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let (operand, lt, gt, dst);

        if let Some((condition, dest)) = value.split_once(':') {
            if let Some((op1, op2)) = condition.split_once('<') {
                operand = Some(op1.to_string());
                lt = Some(op2.parse().unwrap());
                gt = None;
            } else {
                let (op1, op2) = condition.split_once('>').unwrap();
                operand = Some(op1.to_string());
                lt = None;
                gt = Some(op2.parse().unwrap());
            }

            dst = Dest::from(dest);
        } else {
            operand = None;
            lt = None;
            gt = None;
            dst = Dest::from(value);
        }

        Self {
            operand,
            lt,
            gt,
            dst,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Dest {
    Workflow(String),
    Rejected,
    Accepted,
}

impl From<&str> for Dest {
    fn from(value: &str) -> Self {
        match value {
            "R" => Self::Rejected,
            "A" => Self::Accepted,
            _ => Self::Workflow(value.to_string()),
        }
    }
}

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$").unwrap();
        let (x, m, a, s) = re
            .captures(value)
            .unwrap()
            .iter()
            .skip(1)
            .filter_map(|n| n.unwrap().as_str().parse().ok())
            .collect_tuple()
            .unwrap();

        Self { x, m, a, s }
    }
}

#[aoc::main(19)]
fn main(input: &str) -> (u32, u32) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows = workflows
        .lines()
        .map(Workflow::from)
        .map(|w| (w.id.clone(), w))
        .collect::<HashMap<String, Workflow>>();
    let parts = parts.lines().map(Part::from).collect_vec();

    let p1 = parts
        .iter()
        .map(|p| {
            let mut res;
            let mut wf = workflows.get("in").unwrap();

            loop {
                res = wf.process(p);

                match res {
                    Dest::Rejected | Dest::Accepted => break,
                    Dest::Workflow(id) => wf = workflows.get(id.as_str()).unwrap(),
                }
            }

            (p, res)
        })
        .filter(|(_, res)| *res == Dest::Accepted)
        .map(|(p, _)| p.x + p.m + p.a + p.s)
        .sum();

    (p1, 0)
}
