#[derive(Debug)]
struct Game {
    id: u16,
    samples: Vec<Sample>,
}

impl Game {
    fn is_possible_with(&self, red: u16, green: u16, blue: u16) -> bool {
        !self
            .samples
            .iter()
            .any(|s| s.red > red || s.green > green || s.blue > blue)
    }

    fn min_set_of_cubes(&self) -> (u32, u32, u32) {
        let mut max = (0, 0, 0);
        for s in self.samples.iter() {
            max.0 = max.0.max(s.red as u32);
            max.1 = max.1.max(s.green as u32);
            max.2 = max.2.max(s.blue as u32);
        }

        max
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let (game, samples) = value.split_once(':').unwrap();
        let id = game.split_whitespace().last().unwrap().parse().unwrap();

        let samples = samples
            .split(';')
            .map(|s| s.trim())
            .map(Sample::from)
            .collect();

        Game { id, samples }
    }
}

#[derive(Debug)]
struct Sample {
    red: u16,
    green: u16,
    blue: u16,
}

impl From<&str> for Sample {
    fn from(value: &str) -> Self {
        let mut res = Sample {
            red: 0,
            green: 0,
            blue: 0,
        };

        value.split(", ").for_each(|s| {
            let (n, color) = s.split_once(' ').unwrap();
            let n = n.parse().unwrap();
            match color {
                "red" => res.red = n,
                "green" => res.green = n,
                "blue" => res.blue = n,
                _ => {}
            }
        });

        res
    }
}

#[aoc::main(02)]
fn main(input: &str) -> (usize, u32) {
    let games = input.lines().map(Game::from).collect::<Vec<_>>();

    let p1 = games
        .iter()
        .filter(|g| g.is_possible_with(12, 13, 14))
        .fold(0, |acc, g| acc + g.id);

    let p2 = games
        .iter()
        .map(Game::min_set_of_cubes)
        .map(|(r, g, b)| r * g * b)
        .sum::<u32>();

    (p1.into(), p2)
}
