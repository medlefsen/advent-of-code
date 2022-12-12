use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::mem::swap;
use std::ops::{Add, Div, Mul, Rem};
use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "src/day11.pest"]
struct InputParser;

trait ParseInto<T> {
    fn parse_into(self) -> T;
}

trait ParseNext<T> {
    fn parse_next(&mut self) -> T;
}

impl<'a, T,R,O> ParseNext<O> for T
  where
      T: Iterator<Item=R>,
      R: ParseInto<O>
{
    fn parse_next(&mut self) -> O {
        self.next().unwrap().parse_into()
    }
}

trait Square {
    fn square(self) -> Self;
}

trait FromInitial {
    fn from_initial<T: Iterator<Item=i32>>(initial_val: i32, divisors: T) -> Self;
}

trait Item: Debug + Clone + FromInitial + Add<i32, Output=Self> + Mul<i32, Output=Self> + Div<i32, Output=Self> + Rem<i32,Output=i32> + Square {
}

impl<T> Item for T
    where T: Debug + Clone + FromInitial + Add<i32, Output=Self> + Mul<i32, Output=Self> + Div<i32, Output=Self> + Rem<i32,Output=i32> + Square {
}

#[derive(Clone, Copy, Debug)]
struct IntItem(i64);

impl FromInitial for IntItem {
    fn from_initial<T: Iterator<Item=i32>>(initial_val: i32, _: T) -> Self {
        Self(initial_val as i64)
    }
}

impl Square for IntItem {
    fn square(self) -> Self {
        Self(self.0 * self.0)
    }
}

impl Add<i32> for IntItem {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Self(self.0 + rhs as i64)
    }
}

impl Div<i32> for IntItem {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self(self.0 / rhs as i64)
    }
}

impl Mul<i32> for IntItem {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self(self.0 * rhs as i64)
    }
}

impl Rem<i32> for IntItem {
    type Output = i32;

    fn rem(self, rhs: i32) -> Self::Output {
        (self.0 % rhs as i64) as i32
    }
}

#[derive(Clone, Copy, Debug)]
enum Operand {
    Old,
    Num(i32)
}
use Operand::*;
impl<'a> ParseInto<Operand> for Pair<'a, Rule> {
    fn parse_into(self) -> Operand {
        match self.as_str() {
            "old" => Old,
            num => Num(num.parse().unwrap()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Op { Add, Mul }
impl<'a> ParseInto<Op> for Pair<'a, Rule> {
    fn parse_into(self) -> Op {
        match self.as_str() {
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
    fn eval<T: Item>(&self, old: T) -> T
    {
        match (self.op, self.rhs) {
            (Op::Add, Num(v)) => old + v,
            (Op::Mul, Num(v)) => old * v,
            (Op::Mul, Old) => old.square(),
            _ => unreachable!()
        }
    }
}

impl<'a> ParseInto<Operation> for Pair<'a, Rule> {
    fn parse_into(self) -> Operation {
        let mut pairs = self.into_inner();
        Operation{
            op: pairs.parse_next(),
            rhs: pairs.parse_next(),
        }
    }
}

impl<'a, T> ParseInto<Vec<T>> for Pair<'a, Rule>
  where Pair<'a, Rule>: ParseInto<T>
{
    fn parse_into(self) -> Vec<T> {
        self.into_inner().map(|r| r.parse_into() ).collect()
    }
}

impl<'a> ParseInto<i32> for Pair<'a, Rule> {
    fn parse_into(self) -> i32 {
        self.as_str().parse().unwrap()
    }
}

impl<'a> ParseInto<i64> for Pair<'a, Rule> {
    fn parse_into(self) -> i64 {
        self.as_str().parse().unwrap()
    }
}

impl<'a> ParseInto<usize> for Pair<'a, Rule> {
    fn parse_into(self) -> usize {
        self.as_str().parse().unwrap()
    }
}

#[derive(Clone, Debug)]
struct ThrowDecision {
    divisor: i32,
    if_divisible_monkey: usize,
    if_not_divisible_monkey: usize,
}

impl<'a> ParseInto<ThrowDecision> for Pair<'a, Rule> {
    fn parse_into(self) -> ThrowDecision {
        let mut pairs = self.into_inner();
        let divisor = pairs.next().unwrap().into_inner().parse_next();
        let if_divisible_monkey = pairs.next().unwrap().into_inner().parse_next();
        let if_not_divisible_monkey = pairs.next().unwrap().into_inner().parse_next();
        ThrowDecision { divisor, if_divisible_monkey, if_not_divisible_monkey }
    }
}

impl ThrowDecision {
    fn throw_to<T: Rem<i32, Output=i32> + Clone + Debug>(&self, item: &T) -> usize
    {
        let m = item.clone() % self.divisor;
        if  m == 0 {
            self.if_divisible_monkey
        } else {
            self.if_not_divisible_monkey
        }
    }
}

#[derive(Clone, Debug)]
struct MonkeyInput {
    items: Vec<i32>,
    operation: Operation,
    throw_decision: ThrowDecision,
}

impl<'a> ParseInto<MonkeyInput> for Pair<'a, Rule> {
    fn parse_into(self) -> MonkeyInput {
        let mut pairs = self.into_inner();
        MonkeyInput {
            items: pairs.parse_next(),
            operation: pairs.parse_next(),
            throw_decision: pairs.parse_next(),
        }
    }
}

#[derive(Clone, Debug)]
struct RemMap(HashMap<i32,i32>);

impl FromInitial for RemMap {
    fn from_initial<T: Iterator<Item=i32>>(initial_val: i32, divisors: T) -> Self {
        Self(divisors.map(|divisor| (divisor, initial_val % divisor)).collect())
    }
}

impl Add<i32> for RemMap {
    type Output = RemMap;

    fn add(self, rhs: i32) -> Self::Output {
        Self(
            self.0.into_iter().map(|(divisor, rem)| (divisor, (rem + rhs) % divisor)).collect()
        )
    }
}
impl Mul<i32> for RemMap {
    type Output = RemMap;

    fn mul(self, rhs: i32) -> Self::Output {
        Self(
            self.0.into_iter().map(|(divisor, rem)| (divisor, (rem * rhs) % divisor)).collect()
        )
    }
}
impl Div<i32> for RemMap {
    type Output = RemMap;

    fn div(self, _: i32) -> Self::Output {
        unimplemented!();
    }
}
impl Square for RemMap {
    fn square(self) -> Self {
        Self(
            self.0.into_iter().map(|(divisor, rem)| (divisor, (rem * rem) % divisor)).collect()
        )
    }
}
impl Rem<i32> for RemMap {
    type Output = i32;

    fn rem(self, rhs: i32) -> Self::Output {
        self.0[&rhs]
    }
}

struct Monkey<ItemType: Item> {
    total_inspected: usize,
    items: Vec<ItemType>,
    operation: Operation,
    throw_decision: ThrowDecision,
}


fn parse_input() -> Vec<MonkeyInput> {
    let input = read_to_string("inputs/day11/input.txt").unwrap();
    match InputParser::parse(Rule::input, &input) {
        Ok(mut pairs) => {
            pairs.next().unwrap().into_inner().parse_next()
        }
        Err(err) => {
            println!("Error parsing input: {}", err);
            panic!();
        }
    }
}


#[derive(Clone, Debug)]
struct Throw<T> {
    item: T,
    monkey_idx: usize,
}

impl<ItemType: Item> Monkey<ItemType> {
    fn new(input: MonkeyInput, divisors: &Vec<i32>) -> Self {
        Self{
            total_inspected: 0,
            items: input.items.iter().map(|item| ItemType::from_initial(*item, divisors.iter().copied())).collect(),
            throw_decision: input.throw_decision,
            operation: input.operation,
        }
    }

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

    fn throw_items(&mut self) -> Vec<Throw<ItemType>> {
        let mut items : Vec<ItemType> = Vec::new();
        swap(&mut items, &mut self.items);
        let throws = items.into_iter()
            .map(|item| Throw { monkey_idx: self.throw_decision.throw_to(&item), item }).collect();
        throws
    }
}


struct MonkeyList<T: Item>(Vec<Monkey<T>>);
impl<T: Item> MonkeyList<T> {
    fn new(inputs: Vec<MonkeyInput>) -> Self{
        let divisors : Vec<i32> = inputs.iter().map(|i| i.throw_decision.divisor ).collect();
        Self(
            inputs.into_iter().map(|input| Monkey::new(input, &divisors)).collect()
        )
    }

    fn process_part1_round(&mut self) {
        for i in 0..self.0.len() {
            self.0[i].inspect_items();
            self.0[i].reduce_worry();
            for Throw { item, monkey_idx } in self.0[i].throw_items() {
                self.0[monkey_idx].items.push(item);
            }
        }
    }

    fn process_part2_round(&mut self) {
        for i in 0..self.0.len() {
            self.0[i].inspect_items();
            for Throw { item, monkey_idx } in self.0[i].throw_items() {
                self.0[monkey_idx].items.push(item);
            }
        }
    }

    fn score(mut self) -> usize {
        self.0.sort_by(|a, b| b.total_inspected.cmp(&a.total_inspected) );
        self.0[0].total_inspected * self.0[1].total_inspected
    }
}

pub fn part1() {
    let mut monkeys = MonkeyList::<IntItem>::new(parse_input());
    for _ in 0..20 {
        monkeys.process_part1_round();
    }
    println!("{:?}", monkeys.score());
}

pub fn part2() {
    let mut monkeys = MonkeyList::<RemMap>::new(parse_input());
    for _ in 0..10000 {
        monkeys.process_part2_round();
    }
    println!("{:?}", monkeys.score());
}