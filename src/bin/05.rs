use itertools::Itertools;

#[derive(Debug)]
struct Operation {
    count: usize,
    from: usize,
    to: usize,
}

impl Operation {
    fn execute_p1(&self, stacks: &mut [Vec<&str>]) {
        for _ in 0..self.count {
            let val = stacks[self.from - 1].pop().unwrap();
            stacks[self.to - 1].push(val);
        }
    }

    fn execute_p2(&self, stacks: &mut [Vec<&str>]) {
        let offset = stacks[self.from - 1].len() - self.count;
        let mut moving = stacks[self.from - 1].drain(offset..).collect_vec();
        stacks[self.to - 1].append(&mut moving);
    }
}

#[aoc::main(05)]
fn main(input: &str) -> (String, String) {
    let (stacks, ops) = input.split_once("\n\n").unwrap();

    let mut stacks = stacks.lines()
        .map(|l| l.split_ascii_whitespace().collect_vec())
        .collect_vec();

    let ops = ops.lines()
        .map(|l| {
            let (x, y, z) = l.split_ascii_whitespace().next_tuple().unwrap();
            Operation{
                count: x.parse().unwrap(),
                from: y.parse().unwrap(),
                to: z.parse().unwrap(),
            }
        })
        .collect_vec();
    
    let mut stacks_p1 = stacks.clone();

    for op in &ops {
        op.execute_p1(&mut stacks_p1)
    }

    let p1 = stacks_p1.iter()
        .map(|stack| stack.last().unwrap())
        .join("");

    for op in ops {
        op.execute_p2(&mut stacks);
    }

    let p2 = stacks.iter()
        .map(|stack| stack.last().unwrap())
        .join("");

    (p1, p2)
}