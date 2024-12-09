#[derive(Clone, Debug, PartialEq, Eq)]
enum Element {
    Number((usize, i32)),
    Tombstone,
}

#[aoc::main(20)]
fn main(input: &str) -> (i32, usize) {
    let queue = input
        .lines()
        .filter_map(|l| l.parse::<i32>().ok())
        .enumerate()
        .map(|x| Element::Number(x))
        .collect::<Vec<Element>>();

    let mut mix = queue.clone();
    let modulus = mix.len();

    for el in queue {
        let cur_idx = mix.iter().position(|x| x == &el).unwrap();
        let n = match el {
            Element::Number((_, n)) => n,
            _ => unreachable!("all queue elements should be Number variants"),
        };
        let new_idx = (cur_idx as i32 + n).rem_euclid(modulus as i32) as usize;

        if cur_idx != new_idx {
            // let num = mix.get(cur_idx).unwrap().clone();
            // mix[cur_idx] = Element::Tombstone;
            let num = mix.remove(cur_idx);
            mix.insert(new_idx, num);
            // mix.remove(mix.iter().position(|el| *el == Element::Tombstone).unwrap());
        }
    }

    let zero = mix
        .iter()
        .position(|el| match el {
            Element::Number((_, n)) => *n == 0,
            _ => false,
        })
        .unwrap();

    let p1 = [1000, 2000, 3000]
        .iter()
        .map(|d| (zero + d).rem_euclid(modulus))
        .map(|i| match mix.get(i).unwrap() {
            Element::Number((_, n)) => n,
            _ => unreachable!(),
        })
        .sum();

    (p1, 0)
}
