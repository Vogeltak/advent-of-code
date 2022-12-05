use itertools::Itertools;

#[derive(Debug)]
struct Operation {
    count: usize,
    from: usize,
    to: usize,
}

impl Operation {
    fn execute(&self, stacks: &mut [Vec<&str>]) {
        println!("Count: {}, From: {}, Size of From: {}", self.count, self.from - 1, stacks[self.from - 1].len());
        for _ in 0..self.count {
            let val = stacks[self.from - 1].pop().unwrap();
            stacks[self.to - 1].push(val);
        }
    }
}

#[aoc::main(05)]
fn main(input: &str) -> (usize, usize) {
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
    
    for op in ops {
        op.execute(&mut stacks);
    }

    let p1 = stacks.iter()
        .map(|stack| stack.last().unwrap())
        .fold("".to_string(), |acc, &x| format!("{acc}{x}"));

    println!("Part 1: {}", p1);
    (0, 0)
}