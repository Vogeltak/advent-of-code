use std::collections::HashSet;
use itertools::Itertools;

#[aoc::main(06)]
fn main(input: &str) -> (usize, usize) {
    let start_of_packet_marker = input.chars()
        .tuple_windows::<(_, _, _, _)>()
        .position(|(a,b,c,d)| HashSet::from([a, b, c, d]).len() == 4)
        .unwrap();

    let p1 = start_of_packet_marker + 4;

    // Traits on Tuples in Rust are only implemented for tuples of arity
    // twelve or less. This means that we cannot use our delicious functional
    // approach from above. I tried looking into implementing the traits
    // myself, but it got a bit too involved for the early morning.
    // See https://doc.rust-lang.org/std/primitive.tuple.html#trait-implementations-1.
    let mut p2 = 0;
    let input_chars = input.chars().collect_vec();
    for i in 0..input_chars.len() - 14 {
        let candidate = &input_chars[i..i+14];
        let set: HashSet<&char> = HashSet::from_iter(candidate);

        if set.len() == 14 {
            p2 = i + 14;
            break;
        }
    }

    (p1, p2)
}