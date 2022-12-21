use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::collections::hash_map::Entry;
use pest::iterators::Pair;
use crate::parsing::{FromPair, ParseFile, ParseNext};
use crate::quad::Quad;

#[derive(Parser)]
#[grammar="src/day19.pest"]
struct InputParser;

#[derive(Copy, Clone, Debug)]
#[repr(usize)]
enum Resource {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}
use crate::day19::Resource::*;
use crate::weighted_value::WeightedValue;

type Count = u32;

impl Into<usize> for Resource {
    fn into(self) -> usize {
       self as usize
    }
}

impl FromPair<Rule> for Resource {
    fn from_pair(pair: Pair<Rule>) -> Self {
        match pair.as_str() {
            "ore" => Self::Ore,
            "clay" => Self::Clay,
            "obsidian" => Self::Obsidian,
            "geode" => Self::Geode,
            r => panic!("Invalid resource: {}", r),
        }
    }
}

#[derive(Debug)]
struct Robot {
    resource: Resource,
    costs: Vec<(Count, Resource)>
}

impl FromPair<Rule> for Robot {
    fn from_pair(pair: Pair<Rule>) -> Self {
        let mut pairs = pair.into_inner();
        Self {
            resource: pairs.parse_next(),
            costs: pairs.parse_next(),
        }
    }
}


struct Blueprint {
    robots: Quad<Quad<Count>>,
}

impl Blueprint {
    fn from_input(input: &Vec<Robot>) -> Self {
        let mut robots: Quad<Quad<Count>> = Default::default();
        for robot in input {
            robots[robot.resource as usize] = Quad::from_input(&robot.costs);
        }
        Self { robots }
    }

    fn build_robots(&self, state: State, robot_mask: Quad<Count>) -> Option<State> {
        let total_costs = self.robots.pw_mul(robot_mask).sum();
        if state.resources >= total_costs {
            Some(
                State {
                    robots: state.robots + robot_mask,
                    resources: state.resources + state.robots - total_costs,
                }
            )
        } else {
            None
        }
    }

    fn for_each_action<F: FnMut(State)>(&self, state: State, mut f: F) {
        lazy_static!{
          static ref MASKS : [Quad<Count>; 5] = [
                Quad::new(0,0,0,0),
                Quad::new(1,0,0,0),
                Quad::new(0,1,0,0),
                Quad::new(0,0,1,0),
                Quad::new(0,0,0,1),
            ];
        }

        for robot_mask in MASKS.iter() {
            if let Some(new_state) = self.build_robots(state, *robot_mask) {
                f(new_state);
            }
        }
    }

    fn calculate_max_geodes(&self, num_minutes: usize) -> Count {
        let state : State = Default::default();
        let mut queue : BinaryHeap<WeightedValue<usize, (usize, State)>>= Default::default();
        let mut scores : HashMap<State, usize> = Default::default();
        let mut scores2 : HashMap<(usize, Quad<Count>), Quad<Count>> = Default::default();
        queue.push(WeightedValue::new((0,state), num_minutes * 8));
        scores.insert(state, 0);
        scores2.insert((0, state.robots), state.resources);
        let mut counter: usize = 0;
        let mut max_geodes : u32 = 0;
        while let Some(val)  = queue.pop() {
            let (minute, state) = val.value;
            let geodes =state.resources[Geode.into()];
            if geodes > max_geodes {
                max_geodes = geodes;
            }
            if minute < num_minutes {
                if counter % 1000000 == 0 {
                    println!("{}: {} open", counter, queue.len());
                }
                counter += 1;
                let next_min = minute + 1;
                self.for_each_action(state, |next_state| {
                    let score = (num_minutes - next_min) * 8 + (next_state.resources[Geode.into()] as usize);
                    let best_score = scores.entry(next_state).or_insert(usize::MIN);
                    let best_score2 = scores2.entry((next_min, next_state.robots));
                    let has_best_score2 = match best_score2 {
                        Entry::Occupied(mut e) => {
                            let cur = e.get_mut();
                            !(*cur >= next_state.resources) && {
                                *cur = next_state.resources;
                                true
                            }
                        }
                        Entry::Vacant(v) => { v.insert(next_state.resources); true}
                    };

                    if score > *best_score && has_best_score2 {
                        *best_score = score;
                        queue.push(WeightedValue::new((next_min, next_state), score));
                    }
                })
            } else {
                break;
            }
        }
        return max_geodes;
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct State {
    robots: Quad<Count>,
    resources: Quad<Count>,
}

impl Default for State {
    fn default() -> Self {
        let mut robots: Quad<Count> = Default::default();
        robots[Ore.into()] = 1;
        Self {
            robots,
            resources: Default::default(),
        }
    }
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.resources[Geode.into()].cmp(&other.resources[Geode.into()])
    }
}

fn read_input() -> Vec<Blueprint> {
    let (inputs,) : (Vec<_>,) = InputParser::parse_file(Rule::input, "inputs/day19/input.txt");
    inputs.iter().map(|input| Blueprint::from_input(input)).collect()
}

pub fn part1() {
    let blueprints = read_input();
    let amounts : Vec<_> = blueprints.iter().map(|blueprint| {
        let geodes = blueprint.calculate_max_geodes(24);
        println!("{} geodes", geodes);
        geodes
    }).collect();
    println!("{:?}", amounts);
    let score : usize = amounts.iter().enumerate().map(|(i,a)| (i+1) * (*a as usize)).sum();
    println!("{}", score);
}

pub fn part2() {
    let blueprints = read_input();
    let amounts : Vec<_> = blueprints.iter().take(3).map(|blueprint| {
        let geodes = blueprint.calculate_max_geodes(32);
        println!("{} geodes", geodes);
        geodes
    }).collect();
    println!("{:?}", amounts);
    let score : u32 = amounts.into_iter().product();
    println!("{}", score);
}