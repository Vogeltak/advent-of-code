use itertools::Itertools;
use num_traits::pow;

struct Device {
    ra: i64,
    rb: i64,
    rc: i64,
    program: Program,
    ip: usize,
}

impl Device {
    fn new(ra: i64, rb: i64, rc: i64, program: Program) -> Self {
        Self {
            ra,
            rb,
            rc,
            program,
            ip: 0,
        }
    }

    fn reset(&mut self) {
        self.ra = 0;
        self.rb = 0;
        self.rc = 0;
        self.ip = 0;
    }

    fn run(&mut self) -> Vec<i64> {
        let mut out = vec![];

        while let Some(op) = self.program.get(self.ip) {
            // SAFETY: all operations are directly followed by an operand
            let operand = self.program.get(self.ip + 1).unwrap();
            let operand_val = match operand {
                Op::Operand(n) => n,
                _ => unreachable!(),
            };

            let mut jumped = false;

            match op {
                Op::Adv => {
                    let combo_val = self.get_combo_value(operand);
                    // i64 division should truncate (floor) to an integer by default
                    self.ra /= pow(2, combo_val as usize);
                }
                Op::Bxl => self.rb ^= operand_val as i64,
                Op::Bst => self.rb = self.get_combo_value(operand).rem_euclid(8),
                Op::Jnz => {
                    if self.ra > 0 {
                        self.ip = operand_val as usize;
                        jumped = true;
                    }
                }
                Op::Bxc => self.rb ^= self.rc,
                Op::Out => out.push(self.get_combo_value(operand).rem_euclid(8)),
                // same as Op::Adv
                Op::Bdv => self.rb = self.ra / pow(2, self.get_combo_value(operand) as usize),
                Op::Cdv => self.rc = self.ra / pow(2, self.get_combo_value(operand) as usize),
                Op::Operand(_) => unreachable!(),
            }

            if !jumped {
                // Make sure to also skip over the op's operand
                self.ip += 2;
            }
        }

        out
    }

    fn get_combo_value(&self, op: Op) -> i64 {
        use Op::*;
        match op {
            Operand(i) if (0..4).contains(&i) => i as i64,
            Operand(4) => self.ra,
            Operand(5) => self.rb,
            Operand(6) => self.rc,
            _ => unreachable!(),
        }
    }
}

struct Program {
    ops: Vec<Op>,
}

impl Program {
    fn get(&self, i: usize) -> Option<Op> {
        self.ops.get(i).cloned()
    }
}

impl From<Vec<Op>> for Program {
    fn from(value: Vec<Op>) -> Self {
        Self { ops: value }
    }
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
    Operand(u8),
}

impl From<u8> for Op {
    fn from(value: u8) -> Self {
        use Op::*;
        match value {
            0 => Adv,
            1 => Bxl,
            2 => Bst,
            3 => Jnz,
            4 => Bxc,
            5 => Out,
            6 => Bdv,
            7 => Cdv,
            // NOTE: we do not actually expect operands outside 0..8
            // Their value does not determine whether they are an operand.
            // Instead, it's their position in the program. Meaning that we'll
            // create those manually instead.
            _ => Operand(value),
        }
    }
}

// The plan is basically to search every subsequent (a << 3) + (0..8) for
// every program instruction/operand, starting from the back (the one we must
// output last), so the bits that must be most significant in reg A's starting value.
fn solve(mut device: Device, program: &[i64]) -> i64 {
    let mut candidates = vec![1];

    for sub_problem in (0..program.len()).rev() {
        println!("{:?}", candidates);
        let mut new_candidates = vec![];

        for a in candidates.iter().flat_map(|a| (0..8).map(move |i| a + i)) {
            device.reset();
            device.ra = a;
            let res = device.run();
            if res == program[sub_problem..] {
                if sub_problem == 0 {
                    return a;
                }
                new_candidates.push(a << 3);
            }
        }

        candidates = new_candidates;
    }

    0
}

#[aoc::main(17)]
fn main(input: &str) -> (String, i64) {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let (ra, rb, rc): (i64, i64, i64) = registers
        .lines()
        .map(|l| l.parse().unwrap())
        .collect_tuple()
        .unwrap();
    let og_program: Vec<i64> = program.split(',').map(|n| n.parse().unwrap()).collect_vec();
    let program = og_program
        .iter()
        .enumerate()
        .map(|(i, op)| match i % 2 == 0 {
            true => Op::from(*op as u8),
            false => Op::Operand(*op as u8),
        })
        .collect_vec();

    let mut device = Device::new(ra, rb, rc, program.into());

    let p1 = device.run().into_iter().join(",");
    let p2 = solve(device, &og_program);

    (p1, p2)
}
