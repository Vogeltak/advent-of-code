use regex::Regex;

fn do_op(op: &str) -> usize {
    // Multiply operation is of the form `mul(xxx,xxx)`.
    // SAFETY: This is guaranteed by our regex match.
    let (l, r) = op
        .strip_prefix("mul(")
        .unwrap()
        .strip_suffix(")")
        .unwrap()
        .split_once(',')
        .unwrap();

    let (l, r): (usize, usize) = (l.parse().unwrap(), r.parse().unwrap());

    l * r
}

#[aoc::main(03)]
fn main(input: &str) -> (usize, usize) {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let muls = input.lines().flat_map(|l| re.find_iter(l));
    let p1 = muls.map(|m| do_op(m.as_str())).sum();

    let re = Regex::new(r"do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\)").unwrap();
    let cond_muls = input.lines().flat_map(|l| re.find_iter(l));

    // Simulate instructions being enabled and disabled
    let mut enabled = true;
    let enabled_muls = cond_muls.filter(|op| match op.as_str() {
        "do()" => {
            enabled = true;
            false
        }
        "don't()" => {
            enabled = false;
            false
        }
        _ => enabled,
    });
    let p2 = enabled_muls.map(|m| do_op(m.as_str())).sum();

    (p1, p2)
}
