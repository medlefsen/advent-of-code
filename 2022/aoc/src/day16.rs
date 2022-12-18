use std::cmp::{max, Ordering};
use std::collections::{HashMap, HashSet, VecDeque};
use std::collections::hash_map::Entry;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::mem::{swap, take};
use std::ops::{Add, Deref};
use std::rc::Rc;
use itertools::Itertools;
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

#[derive(Clone, Hash, Eq, PartialEq)]
struct MultiCaveState {
    valves: Vec<Rc<Valve>>,
    cur: (String,String),
}

#[derive(Clone, Eq, PartialEq)]
enum Action {
    Open,
    Move(String),
}

#[derive(Clone)]
struct Output {
    initial: u32,
    minute: u32,
    rate: u32,
}

impl Output {
    fn total(&self) -> u32 {
        self.total_at(30)
    }

    fn total_at(&self, min: u32) -> u32 {
        self.initial + self.rate * (min - self.minute)
    }
}

impl Eq for Output {}

impl PartialEq<Self> for Output {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for Output {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Output {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total().cmp(&other.total())
    }
}


impl MultiCaveState {
    fn get(&self, name: &String) -> &Valve {
        self.valves.iter().find(|v| &v.name == name ).unwrap()
    }

    fn done(&self) -> bool {
        self.valves.iter().all(|v| v.open)
    }

    fn current_rate(&self) -> u32 {
        self.valves.iter()
            .filter(|valve| valve.open)
            .map(|valve| valve.rate)
            .sum()
    }

    fn actions(&self, name: &String) -> Vec<Action> {
        let valve = self.get(name);
        let mut actions : Vec<_> = valve.tunnels.iter().cloned().map(|n| Action::Move(n)).collect();
        if !valve.open && valve.rate > 0 {
            actions.push(Action::Open);
        }
        actions
    }

    fn next_states(&self) -> Vec<Self> {
        if self.done() { return vec![self.clone()] }

        let first_actions = self.actions(&self.cur.0).into_iter();
        let second_actions = self.actions(&self.cur.1).into_iter();

        first_actions.cartesian_product(second_actions)
            .filter(|(m,e)| {
                !(m == &Action::Open && e == &Action::Open && self.cur.0 == self.cur.1 )
            })
            .map(|(m,e)| self.action(m,e))
            .collect()
    }

    fn action(&self, a: Action, b: Action) -> Self {
        let valves = self.valves.iter().map(|valve| {
            if a == Action::Open && valve.name == self.cur.0 {
                Rc::new(Valve { open: true, ..(**valve).clone() })
            } else if b == Action::Open && valve.name == self.cur.1 {
                Rc::new(Valve { open: true, ..(**valve).clone() })
            } else {
                valve.clone()
            }
        }).collect();
        let cur_0 = if let Action::Move(name) = a { name } else { self.cur.0.clone() };
        let cur_1 = if let Action::Move(name) = b { name } else { self.cur.1.clone() };
        let cur = if cur_0 < cur_1 { (cur_0, cur_1) } else { (cur_1, cur_0) };
        Self {
            valves,
            cur,
        }
    }
}

impl Debug for MultiCaveState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut first = false;
        for valve in &self.valves {
            let cur_marker = if valve.name == self.cur.0 { "*" } else { "" };
            let eleph_marker = if valve.name == self.cur.1 { "*" } else { "" };
            if first {
                first = false;
            } else {
                f.write_str(", ")?;
            }
            f.write_fmt(format_args!("{}{}{:?}", cur_marker, eleph_marker, **valve))?;
        }
        f.write_fmt(format_args!(" -> {}", self.current_rate()))?;
        Ok(())
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
    let mut open : VecDeque<(CaveState, u32)> = VecDeque::new();
    open.push_back((start.clone(), 0));
    scores.insert(start.clone(), 0);
    for i in 1..=30 {
        println!("{}: {} open", i, open.len());
        let mut cur_open = take(&mut open);
        for (state, score) in &cur_open {
            let total = score + state.current_rate();
            if state.done() {
                update_state(&mut scores, &mut open, state, total);
            } else {
                if !state.open && state.rate > 0 {
                    let open_state = state.open();
                    update_state(&mut scores, &mut open, &open_state, total);
                }
                for move_state in state.moves() {
                    update_state(&mut scores, &mut open, &move_state, total);
                }
            }
        }
    }
    let (state, score) = scores.iter().max_by(|(_,a),(_,b)| a.cmp(b) ).unwrap();
    println!("Final {}: {:?}", score, state);
}

fn update_state<T: Eq + Hash + Clone, O: Clone + Ord>(scores: &mut HashMap<T, O>, open: &mut VecDeque<(T, O)>, state: &T, total: O) {
    match scores.entry(state.clone()) {
        Entry::Occupied(mut o) => {
            let cur_score = o.get_mut();
            if total > *cur_score {
                *cur_score = total.clone();
                open.push_back((state.clone(), total));
            }
        }
        Entry::Vacant(o) => {
            o.insert(total.clone());
            open.push_back((state.clone(), total));
        }
    }
}

pub fn part2() {
    let valves = parse_input();
    let mut scores : HashMap<MultiCaveState, Output> = HashMap::new();
    let start = MultiCaveState { valves: valves.clone(), cur: ("AA".into(), "AA".into()) };
    let mut open : VecDeque<(MultiCaveState, Output)> = VecDeque::new();
    open.push_back((start.clone(), Output { minute: 0, initial: 0, rate: 0}));
    scores.insert(start.clone(), Output { minute: 0, initial: 0, rate: 0});
    for minute in 0..26 {
        println!("{}: {} open", minute, open.len());
        let mut cur_open = take(&mut open);
        for (state, score) in &cur_open {
            let output = Output { initial: score.total_at(minute), minute, rate: state.current_rate() };
            for new_state in &state.next_states() {
                update_state(&mut scores, &mut open, new_state, output.clone());
            }
        }
    }
    let (state, score) = scores.iter().max_by(|(_,a),(_,b)| a.cmp(b) ).unwrap();
    println!("Final {}: {:?}", score.total_at(26), state);

}