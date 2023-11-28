use std::convert::TryFrom;

use hashbrown::HashMap;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: u32,
    open: bool,
    tunnels: Vec<String>
}

impl TryFrom<&str> for Valve {
    type Error = color_eyre::Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.strip_prefix("Valve ").unwrap();
        let (valve_name, value)= value.split_once(" has flow rate=").unwrap();
        let (flow_rate, value) = value.split_once("; tunnels lead to valves ").unwrap();
        let tunnels: Vec<String> = value.split(", ").map(|t| t.to_string()).collect_vec();

        Ok(Self {
            name: valve_name.to_string(),
            flow_rate: flow_rate.parse()?,
            open: false,
            tunnels,
        })
    }
}

#[aoc::main(16)]
fn main(input: &str) -> (usize, usize) {
    let mut valves: HashMap<String, Valve> = HashMap::new();
    for l in input.lines() {
        let valve = Valve::try_from(l).unwrap();
        valves.insert(valve.name.clone(), valve);
    }

    (0, 0)
}