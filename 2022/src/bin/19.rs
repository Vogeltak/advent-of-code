use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    time_left: u8,
    ore: u64,
    clay: u64,
    obsidian: u64,
    geodes: u64,
    ore_bots: u64,
    clay_bots: u64,
    obsidian_bots: u64,
    geode_bots: u64,
}

impl State {
    fn new(orig_time: u8) -> Self {
        Self {
            time_left: orig_time,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
            ore_bots: 1,
            clay_bots: 0,
            obsidian_bots: 0,
            geode_bots: 0,
        }
    }

    /// Assumes `time_left >= 1`
    fn transition(&mut self) {
        self.ore += self.ore_bots;
        self.clay += self.clay_bots;
        self.obsidian += self.obsidian_bots;
        self.geodes += self.geode_bots;
        self.time_left -= 1;
    }

    fn can_afford(&self, bp: &Blueprint, bot: BotType) -> bool {
        match bot {
            BotType::Ore => self.ore >= bp.ore_bot_ore_cost,
            BotType::Clay => self.ore >= bp.clay_bot_ore_cost,
            BotType::Obsidian => {
                self.ore >= bp.obsidian_bot_ore_cost && self.clay >= bp.obsidian_bot_clay_cost
            }
            BotType::Geode => {
                self.ore >= bp.geode_bot_ore_cost && self.obsidian >= bp.geode_bot_obsidian_cost
            }
        }
    }

    /// Assumes the availability of sufficient resources
    fn construct_bot(&mut self, bp: &Blueprint, bot: BotType) {
        match bot {
            BotType::Ore => {
                self.ore -= bp.ore_bot_ore_cost;
                self.ore_bots += 1;
            }
            BotType::Clay => {
                self.ore -= bp.clay_bot_ore_cost;
                self.clay_bots += 1;
            }
            BotType::Obsidian => {
                self.ore -= bp.obsidian_bot_ore_cost;
                self.clay -= bp.obsidian_bot_clay_cost;
                self.obsidian_bots += 1;
            }
            BotType::Geode => {
                self.ore -= bp.geode_bot_ore_cost;
                self.obsidian -= bp.geode_bot_obsidian_cost;
                self.geode_bots += 1;
            }
        }
    }

    /// State expansion to all states reachable via a single edge
    fn expand(&self, bp: &Blueprint) -> Vec<State> {
        use BotType::*;

        let mut new_states = vec![];

        for bot in [Ore, Clay, Obsidian, Geode] {
            // Pruning:
            // don't construct new bots for resources if we already have
            // enough to spend every minute to build any bot we want.
            match bot {
                Ore => {
                    if self.ore_bots
                        >= bp
                            .ore_bot_ore_cost
                            .max(bp.clay_bot_ore_cost)
                            .max(bp.obsidian_bot_ore_cost)
                            .max(bp.geode_bot_ore_cost)
                    {
                        continue;
                    }
                }
                Clay => {
                    if self.clay_bots >= bp.obsidian_bot_clay_cost {
                        continue;
                    }
                }
                Obsidian => {
                    if self.obsidian_bots >= bp.geode_bot_obsidian_cost {
                        continue;
                    }
                }
                _ => {}
            }

            if self.can_afford(bp, bot) {
                let mut candidate = self.clone();
                candidate.transition();
                candidate.construct_bot(bp, bot);
                new_states.push(candidate);
            }
        }

        // do-nothing case
        let mut nothing = self.clone();
        nothing.transition();
        new_states.push(nothing);

        return new_states;
    }
}

#[derive(Clone, Copy, Debug)]
enum BotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug)]
struct Blueprint {
    id: u64,
    ore_bot_ore_cost: u64,
    clay_bot_ore_cost: u64,
    obsidian_bot_ore_cost: u64,
    obsidian_bot_clay_cost: u64,
    geode_bot_ore_cost: u64,
    geode_bot_obsidian_cost: u64,
}

fn bfs(bp: &Blueprint, minutes: u8) -> u64 {
    let mut q = VecDeque::new();
    q.push_back(State::new(minutes));
    let mut seen = HashSet::new();
    let mut max_geodes = 0;

    while let Some(state) = q.pop_front() {
        // Base case
        if state.time_left == 0 {
            max_geodes = max_geodes.max(state.geodes);
            continue;
        }

        // Prune if state cannot beat current max heuristically
        let termial: u64 = (0..state.time_left as u64)
            .map(|x| x + state.geode_bots)
            .sum();
        if state.geodes + termial <= max_geodes {
            continue;
        }

        for mut new_state in state.expand(bp) {
            // Reduce state growth by capping resource count
            let x = new_state.time_left as u64
                * bp.ore_bot_ore_cost
                    .max(bp.clay_bot_ore_cost)
                    .max(bp.obsidian_bot_ore_cost)
                    .max(bp.geode_bot_ore_cost)
                - new_state.ore_bots * ((new_state.time_left as u64).checked_sub(1).unwrap_or(0));
            new_state.ore = new_state.ore.min(x);
            let x = new_state.time_left as u64 * bp.obsidian_bot_clay_cost
                - new_state.clay_bots * ((new_state.time_left as u64).checked_sub(1).unwrap_or(0));
            new_state.clay = new_state.clay.min(x);
            let x = new_state.time_left as u64 * bp.geode_bot_obsidian_cost
                - new_state.obsidian_bots
                    * ((new_state.time_left as u64).checked_sub(1).unwrap_or(0));
            new_state.obsidian = new_state.obsidian.min(x);

            if !seen.contains(&new_state) {
                q.push_back(new_state.clone());
                seen.insert(new_state);
            }
        }
    }

    println!(
        "BP #{:02}: {:2} geodes (eval {} states)",
        bp.id,
        max_geodes,
        seen.len()
    );

    max_geodes
}

#[aoc::main(19)]
fn main(input: &str) -> (u64, u64) {
    // Read blueprints into list
    let re = Regex::new(r"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$").unwrap();
    let blueprints = input
        .lines()
        .map(|line| {
            let (
                id,
                ore_bot_ore_cost,
                clay_bot_ore_cost,
                obsidian_bot_ore_cost,
                obsidian_bot_clay_cost,
                geode_bot_ore_cost,
                geode_bot_obsidian_cost,
            ) = re
                .captures(line)
                .expect("line didn't match regex")
                .iter()
                .skip(1) // first one is always the entire match
                .map(|opt| {
                    opt.expect("capturing group didn't capture?")
                        .as_str()
                        .parse::<u64>()
                        .expect("capture wasn't number?")
                })
                .collect_tuple()
                .expect("didn't get 7 things from regex capture?");
            Blueprint {
                id,
                ore_bot_ore_cost,
                clay_bot_ore_cost,
                obsidian_bot_ore_cost,
                obsidian_bot_clay_cost,
                geode_bot_ore_cost,
                geode_bot_obsidian_cost,
            }
        })
        .collect::<Vec<_>>();

    // Sequentially bfs
    println!("part 1");
    let p1 = blueprints.par_iter().map(|bp| bfs(bp, 24) * bp.id).sum();
    println!("part 2");
    let p2 = blueprints
        .iter()
        .take(3)
        .par_bridge()
        .map(|bp| bfs(bp, 32))
        .product();

    (p1, p2)
}
