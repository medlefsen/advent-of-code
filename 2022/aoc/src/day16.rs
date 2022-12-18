use std::cmp::{max, Ordering};
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::ops::{Add, Deref};
use std::rc::Rc;
use pest::iterators::Pair;
use crate::a_star::{a_star, AStarNode};
use crate::parsing::{FromPair, ParseFile, ParseNext};

#[derive(Parser)]
#[grammar="src/day16.pest"]
struct InputParser;

#[derive(Clone, Hash, Eq, PartialEq)]
struct Valve {
    name: String,
    rate: u32,
    open: bool,
    tunnels: Rc<Vec<String>>,
}

impl Debug for Valve {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let open_state = if self.open { 'O'} else {'X'};
        f.write_fmt(format_args!("{}({},{})", self.name, self.rate , open_state))
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct CaveState {
    valves: Vec<Rc<Valve>>,
    cur: String,
}

impl CaveState {
    fn done(&self) -> bool {
        self.valves.iter().all(|v| v.open)
    }

    fn current_rate(&self) -> u32 {
        self.valves.iter()
            .filter(|valve| valve.open)
            .map(|valve| valve.rate)
            .sum()
    }

    fn moves(&self) -> Vec<Self> {
        self.deref().tunnels.iter().map(|name| {
            let iter = Self {
                valves: self.valves.clone(),
                cur: name.clone(),
            };
            iter
        }).collect()
    }

    fn open(&self) -> Self {
        let valves = self.valves.iter().map(|valve| {
            if valve.name == self.name {
                Rc::new(Valve { open: true, ..(**valve).clone()})
            } else {
                valve.clone()
            }
        }).collect();
        Self{ valves, cur: self.cur.clone() }
    }
}

impl Debug for CaveState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut first = false;
        for valve in &self.valves {
            let cur_marker = if valve.name == self.cur { "*" } else { "" };
            if first {
                first = false;
            } else {
                f.write_str(", ")?;
            }
            f.write_fmt(format_args!("{}{:?}", cur_marker, **valve))?;
        }
        f.write_fmt(format_args!(" -> {}", self.current_rate()))?;
        Ok(())
    }
}

impl Deref for CaveState {
    type Target = Valve;

    fn deref(&self) -> &Self::Target {
        self.valves.iter().find(|v| v.name == self.cur).unwrap()
    }
}

impl FromPair<Rule> for Valve {
    fn from_pair(pair: Pair<Rule>) -> Self {
        let mut pairs = pair.into_inner();
        Self {
            name: pairs.parse_next(),
            rate: pairs.parse_next(),
            tunnels: pairs.parse_next(),
            open: false,
        }
    }
}

fn parse_input() -> Vec<Rc<Valve>> {
    let (input,) = InputParser::parse_file(Rule::input, "inputs/day16/input.txt");
    input
}

pub fn part1() {
    let valves = parse_input();
    let mut scores : HashMap<CaveState, u32> = HashMap::new();
    let start = CaveState { valves: valves.clone(), cur: "AA".into() };
    scores.insert(start.clone(), 0);
    for i in 1..=30 {
        let states :Vec<_>= scores.iter().map(|(s,c)| (s.clone(),c.clone())).collect();
        println!("{}: {} states", i, states.len());
        for (state, score) in &states {
            let total = score + state.current_rate();
            scores.entry(state.open()).and_modify(|v| *v = max(*v, total)).or_insert(total);
            if !state.open && state.rate > 0 {
                scores.entry(state.open()).and_modify(|v| *v = max(*v, total)).or_insert(total);
            }
            for move_state in state.moves() {
                scores.entry(move_state.clone()).and_modify(|v| *v = max(*v, total)).or_insert(total);
            }
        }
    }
    let (state, score) = scores.iter().max_by(|(_,a),(_,b)| a.cmp(b) ).unwrap();
    println!("Final {}: {:?}", score, state);
}

pub fn part2() {

}