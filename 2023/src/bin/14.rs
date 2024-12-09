use grid::Grid;

fn tilt_north(platform: &mut Grid<u8>) {}

#[aoc::main(14)]
fn main(input: &str) -> (usize, usize) {
    let mut platform = Grid::new_with_order(0, 0, grid::Order::ColumnMajor);
    input
        .lines()
        .for_each(|l| platform.push_row(l.as_bytes().to_vec()));

    println!("{}", platform[(7, 7)] as char);

    (0, 0)
}
