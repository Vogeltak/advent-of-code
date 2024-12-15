use itertools::Itertools;
use regex::Regex;
use std::sync::LazyLock;

// A robot's defimition
// p=36,69 v=61,39
static RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap());

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone, Debug)]
struct Robot {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

impl Robot {
    fn step(&mut self) {
        self.x = (self.x + self.vx).rem_euclid(WIDTH);
        self.y = (self.y + self.vy).rem_euclid(HEIGHT);
    }

    #[allow(non_contiguous_range_endpoints)]
    fn quadrant(&self) -> Option<Quadrant> {
        match (self.x, self.y) {
            (0..50, 0..51) => Some(Quadrant::TopLeft),
            (51..101, 0..51) => Some(Quadrant::TopRight),
            (0..50, 52..103) => Some(Quadrant::BottomLeft),
            (51..101, 52..103) => Some(Quadrant::BottomRight),
            _ => None,
        }
    }
}

impl From<&str> for Robot {
    fn from(value: &str) -> Self {
        let (x, y, vx, vy) = RE
            .captures_iter(value)
            .next()
            .unwrap()
            .iter()
            .skip(1)
            .map(|i| i.unwrap().as_str().parse().unwrap())
            .collect_tuple()
            .unwrap();

        Self { x, y, vx, vy }
    }
}

fn display(robots: &[Robot]) {
    let mut grid: [[u8; WIDTH as usize]; HEIGHT as usize] =
        [[b'.'; WIDTH as usize]; HEIGHT as usize];

    for r in robots {
        grid[r.y as usize][r.x as usize] = b'#';
    }

    (0..HEIGHT as usize).for_each(|row| {
        for col in 0..WIDTH as usize {
            print!("{}", grid[row][col] as char);
        }
        println!();
    });
}

fn heuristic(robots: &[Robot]) -> bool {
    robots.iter().filter(|r| (40..61).contains(&r.x)).count() > robots.len() / 2
}

#[aoc::main(14)]
fn main(input: &str) -> (usize, usize) {
    let mut robots = input.lines().map(Robot::from).collect_vec();
    let mut p1_robots = robots.clone();

    (0..100).for_each(|_| p1_robots.iter_mut().for_each(|r| r.step()));

    let (mut tl, mut tr, mut bl, mut br) = (0, 0, 0, 0);
    p1_robots
        .iter()
        .filter_map(|r| r.quadrant())
        .for_each(|q| match q {
            Quadrant::TopLeft => tl += 1,
            Quadrant::TopRight => tr += 1,
            Quadrant::BottomLeft => bl += 1,
            Quadrant::BottomRight => br += 1,
        });

    let p1 = tl * tr * bl * br;

    let mut p2 = 0;

    loop {
        p2 += 1;
        robots.iter_mut().for_each(|r| r.step());

        if heuristic(&robots) {
            display(&robots);
            println!("after {} seconds (q?): ", p2);
            let mut user = String::new();
            std::io::stdin()
                .read_line(&mut user)
                .expect("failed to read line");

            if user.trim_end() == "q" {
                break;
            }
        }
    }

    (p1, p2)
}
