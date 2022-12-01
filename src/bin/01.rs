#[aoc::main(01)]
fn main(input: &str) -> (usize, usize) {
    let mut elves: Vec<usize> = vec![];
    let mut elf = 0;

    for line in input.lines() {
        match line {
            "" => {
                elves.push(elf);
                elf = 0;
            },
            c => elf += c.parse::<usize>().unwrap(),
        }
    }

    elves.push(elf);

    elves.sort_by(|a,b| b.cmp(a));
    
    let p1 = elves[0];
    let p2 = elves.iter().take(3).sum();

    (p1, p2)
}