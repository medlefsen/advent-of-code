use std::fs::read_to_string;

use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "src/day5.pest"]
struct Day5Parser;

#[derive(Copy, Clone, Debug)]
struct Instruction {
    pub count: usize,
    pub from: usize,
    pub to: usize,
}

impl Instruction {
    fn apply(&self, state: &mut State) {
        for _ in 0..self.count {
            let value = state[self.from].pop().unwrap();
            state[self.to].push(value);
        }
    }
    fn apply_grouped(&self, state: &mut State) {
        let from_len = state[self.from].len();
        let mut values : Vec<_> = state[self.from].drain((from_len-self.count)..from_len).collect();
        state[self.to].append(&mut values);
    }
}

type State = Vec<Vec<char>>;
type Input = (State, Vec<Instruction>);

fn parse_initial_state(initial_state: Pair<Rule>) -> State {
    let mut state = State::new();
    let rows = initial_state.into_inner();
    for row in rows.rev() {
        let cells = row.into_inner();
        for (i, cell) in cells.enumerate() {
            if i + 1 > state.len() { state.resize(i + 1, Default::default()) }
            if let Some(label) = cell.into_inner().next() {
                state[i].push(label.as_str().chars().next().unwrap());
            }
        }
    }
    state
}

fn parse_instructions(instructions: Pair<Rule>) -> Vec<Instruction> {
    let instruction_list = instructions.into_inner();
    instruction_list.map(|instruction| {
        let mut parts = instruction.into_inner();
        Instruction {
            count: parts.next().unwrap().as_str().parse().unwrap(),
            from: parts.next().unwrap().as_str().parse::<usize>().unwrap() - 1,
            to: parts.next().unwrap().as_str().parse::<usize>().unwrap() - 1,
        }
    }).collect()
}

fn parse_input(text: &str) -> Option<Input> {
    match Day5Parser::parse(Rule::day5, &text) {
        Ok(mut pairs) => {
            let mut day5 = pairs.next().unwrap().into_inner();
            let initial_state = parse_initial_state(day5.next().unwrap());
            let instructions = parse_instructions(day5.next().unwrap());
            Some( (initial_state, instructions) )
        }
        Err(err) => {
            println!("Error parsing input: {}", err);
            None
        }
    }
}

pub fn part1() {
    let input = read_to_string("inputs/day5/input.txt").unwrap();
    if let Some((mut state, instructions)) = parse_input(&input) {
        for inst in instructions {
            inst.apply(&mut state);
        }
        println!("{}", state.iter().map(|s| s.iter().last().unwrap() ).collect::<String>());
    }
}
pub fn part2() {
    let input = read_to_string("inputs/day5/input.txt").unwrap();
    if let Some((mut state, instructions)) = parse_input(&input) {
        for inst in instructions {
            inst.apply_grouped(&mut state);
        }
        println!("{}", state.iter().map(|s| s.iter().last().unwrap() ).collect::<String>());
    }
}