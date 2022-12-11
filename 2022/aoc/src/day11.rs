use std::fs::read_to_string;
use std::mem::swap;
use std::ops::{Add, Div, Mul, Rem};
use std::rc::Rc;
use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "src/day11.pest"]
struct InputParser;

trait Parseable {
    fn parse(pair: Pair<Rule>) -> Self;
}

trait Square {
    fn square(self) -> Self;
}

#[derive(Clone, Debug)]
enum Item {
    Val(i32),
    Add(Rc<Item>, i32),
    Mul(Rc<Item>, i32),
    Square(Rc<Item>),
}

impl From<Item> for i64 {
    fn from(tree: Item) -> Self {
        match tree {
            Item::Val(v) => v as i64,
            Item::Add(t, v) => i64::from((*t).clone()) + v as i64,
            Item::Mul(t, v) => i64::from((*t).clone()) * v as i64,
            Item::Square(t) => {
                let v : i64 = (*t).clone().into();
                v * v
            }
        }
    }
}

impl Parseable for Item {
    fn parse(pair: Pair<Rule>) -> Self {
        Self::Val(i32::parse(pair))
    }
}

impl Add<i32> for Item {
    type Output = Item;

    fn add(self, rhs: i32) -> Self::Output {
        Self::Add(Rc::new(self), rhs)
    }
}

impl Square for Item {
    fn square(self) -> Self {
        Self::Square(Rc::new(self))
    }
}

impl Mul<i32> for Item {
    type Output = Item;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::Mul(Rc::new(self), rhs)
    }
}

impl Div<i32> for Item {
    type Output = Item;

    fn div(self, rhs: i32) -> Self::Output {
        let lhs : i64 = self.into();
        Self::Val((lhs / rhs as i64) as i32)
    }
}

impl Rem<i32> for Item {
    type Output = i32;

    fn rem(self, rhs: i32) -> Self::Output {
        match self {
            Item::Val(v) => v % rhs,
            Item::Add(t, v) => (((*t).clone() % rhs) + v) % rhs,
            Item::Mul(t, v) => (((*t).clone() % rhs) * v) % rhs,
            Item::Square(t) => {
                let rem = (*t).clone() % rhs;
                (rem * rem) % rhs
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Operand {
    Old,
    Num(i32)
}
use Operand::*;
impl Parseable for Operand {
    fn parse(pair: Pair<Rule>) -> Self {
        match pair.as_str() {
            "old" => Old,
            num => Num(num.parse().unwrap()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Op { Add, Mul }
impl Parseable for Op {
    fn parse(pair: Pair<Rule>) -> Self {
        match pair.as_str() {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => unreachable!()
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Operation {
    rhs: Operand,
    op: Op,
}

impl Operation {
    fn eval(&self, old: Item) -> Item {
        match (self.op, self.rhs) {
            (Op::Add, Num(v)) => old + v,
            (Op::Mul, Num(v)) => old * v,
            (Op::Mul, Old) => old.square(),
            _ => unreachable!()
        }
    }
}

impl Parseable for Operation {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut pairs = pair.into_inner();
        let _ = Operand::parse(pairs.next().unwrap());
        let op = Op::parse(pairs.next().unwrap());
        let rhs = Operand::parse(pairs.next().unwrap());
        Self{ rhs, op }
    }
}

type Items = Vec<Item>;
impl Parseable for Items {
    fn parse(pair: Pair<Rule>) -> Self {
        pair.into_inner().map(|r| Item::parse(r) ).collect()
    }
}

impl Parseable for i32 {
    fn parse(pair: Pair<Rule>) -> Self {
        pair.as_str().parse().unwrap()
    }
}

impl Parseable for i64 {
    fn parse(pair: Pair<Rule>) -> Self {
        pair.as_str().parse().unwrap()
    }
}

impl Parseable for usize {
    fn parse(pair: Pair<Rule>) -> Self {
        pair.as_str().parse().unwrap()
    }
}

#[derive(Clone, Debug)]
struct Throw {
    item: Item,
    monkey_idx: usize,
}

#[derive(Clone, Debug)]
struct Monkey {
    total_inspected: usize,
    items: Vec<Item>,
    operation: Operation,
    divisible_test: i32,
    if_divisible_monkey: usize,
    if_not_divisible_monkey: usize,
}

impl Parseable for Monkey {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut pairs = pair.into_inner();
        pairs.next(); // id number
        let items = Items::parse(pairs.next().unwrap());
        let operation = Operation::parse(pairs.next().unwrap());
        let divisible_test = i32::parse(pairs.next().unwrap().into_inner().next().unwrap());
        let if_divisible_monkey = usize::parse(pairs.next().unwrap().into_inner().next().unwrap());
        let if_not_divisible_monkey = usize::parse(pairs.next().unwrap().into_inner().next().unwrap());
        Self { total_inspected: 0, items, operation, divisible_test, if_divisible_monkey, if_not_divisible_monkey }
    }
}

type MonkeyList = Vec<Monkey>;
impl Parseable for MonkeyList {
    fn parse(pair: Pair<Rule>) -> Self {
        pair.into_inner().map(|p| Monkey::parse(p) ).collect()
    }
}

fn parse_input() -> MonkeyList {
    let input = read_to_string("inputs/day11/input.txt").unwrap();
    match InputParser::parse(Rule::input, &input) {
        Ok(mut pairs) => {
            let pair = pairs.next().unwrap().into_inner().next().unwrap();
            MonkeyList::parse(pair)
        }
        Err(err) => {
            println!("Error parsing input: {}", err);
            panic!();
        }
    }
}

impl Monkey {
    fn inspect_items(&mut self) {
        self.total_inspected += self.items.len();
        for item in self.items.iter_mut() {
            *item = self.operation.eval(item.clone());
        }
    }

    fn reduce_worry(&mut self) {
        for item in &mut self.items {
            *item = item.clone() / 3;
        }
    }

    fn throw_to(&self, item: &Item) -> usize {
        if item.clone() % self.divisible_test == 0 {
            self.if_divisible_monkey
        } else {
            self.if_not_divisible_monkey
        }
    }

    fn throw_items(&mut self) -> Vec<Throw> {
        let mut items : Vec<Item> = Vec::new();
        swap(&mut items, &mut self.items);
        let throws = items.into_iter()
            .map(|item| Throw { monkey_idx: self.throw_to(&item), item }).collect();
        throws
    }
}

fn process_part1_round(monkeys: &mut MonkeyList) {
    for i in 0..monkeys.len() {
        monkeys[i].inspect_items();
        monkeys[i].reduce_worry();
        for Throw { item, monkey_idx } in monkeys[i].throw_items() {
            monkeys[monkey_idx].items.push(item);
        }
    }
}

fn process_part2_round(monkeys: &mut MonkeyList) {
    for i in 0..monkeys.len() {
        monkeys[i].inspect_items();
        for Throw { item, monkey_idx } in monkeys[i].throw_items() {
            monkeys[monkey_idx].items.push(item);
        }
    }
}

pub fn part1() {
   let mut monkeys = parse_input();
    for _ in 0..20 {
        process_part1_round(&mut monkeys);
    }
    monkeys.sort_by(|a, b| b.total_inspected.cmp(&a.total_inspected) );
    println!("{:?}", monkeys[0].total_inspected * monkeys[1].total_inspected);
}

pub fn part2() {
    let mut monkeys = parse_input();
    for _ in 0..10000 {
        process_part2_round(&mut monkeys);
    }
    monkeys.sort_by(|a, b| b.total_inspected.cmp(&a.total_inspected) );
    println!("{:?}", monkeys[0].total_inspected * monkeys[1].total_inspected);
}