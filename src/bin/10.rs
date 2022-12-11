#[aoc::main(10)]
fn main(input: &str) -> (i32, String) {
    let mut instructions = input
        .lines()
        .map(|l| l.split_once(' ').map(|(_, i)| i.parse::<i32>().unwrap()))
        .cycle();

    let (mut rx, mut add) = (1, None);
    let (mut p1, mut p2) = (0, String::new());

    for cycle in 1..240 {
        if [20, 60, 100, 140, 180, 220].contains(&cycle) {
            p1 += cycle * rx;
        }

        let y = (cycle - 1) % 40;
        if y == 0 {
            p2.push('\n');
        }
        if ((rx - y) as i32).abs() < 2 {
            p2.push('█');
        } else {
            p2.push(' ');
        }

        match add.take() {
            Some(v) => rx += v,
            None => add = instructions.next().unwrap(),
        }
    }

    (p1, p2)
}
