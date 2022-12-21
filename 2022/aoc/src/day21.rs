use std::collections::HashMap;
use std::ops::{Add, Sub, Mul, Div};
use pest::iterators::Pair;
use crate::parsing::{FromPair, ParseFile, ParseInto, ParseNext};

#[derive(Parser)]
#[grammar="src/day21.pest"]
struct InputParser;

type Num = i64;

#[derive(Clone, Debug)]
enum Val {
    Num(i64),
    Var(Vec<(Op, i64)>)
}
impl Val {
    fn num(&self) -> i64 {
        if let Self::Num(n) = self {
            *n
        } else {
            panic!("Not a num: {:?}", self);
        }

    }
    fn op<F: FnOnce(i64, i64) -> i64>(self, rhs: Self, op: Op, f: F) -> Self {
        match (self.clone(), rhs.clone()) {
            (Self::Num(a), Self::Num(b)) => { Self::Num(f(a, b)) },
            (Self::Var(mut reverse_ops), Self::Num(n)) => {
                op.reverse_rhs(n, &mut reverse_ops);
                Self::Var(reverse_ops)
            }
            (Self::Num(n), Self::Var(mut reverse_ops)) => {
                op.reverse_lhs(n, &mut reverse_ops);
                Self::Var(reverse_ops)
            }
            (_, _) => panic!("Can't do op on 2 vars:\n{:?}\n{:?}", self, rhs)
        }
    }

    fn solve(&self, equals: i64) -> Option<i64> {
        if let Self::Var(ops) = self {
            Some(ops.iter().rev().fold(equals, |lhs,(op, rhs)| {
               op.apply(Self::Num(lhs), Self::Num(*rhs)).num()
            }))
        } else {
            None
        }
    }
}
impl Add for Val {
    type Output = Val;

    fn add(self, rhs: Self) -> Self::Output {
        self.op(rhs, Op::Add, |a,b| a + b)
    }
}
impl Sub for Val {
    type Output = Val;

    fn sub(self, rhs: Self) -> Self::Output {
        self.op(rhs, Op::Sub, |a,b| a - b)
    }
}
impl Mul for Val {
    type Output = Val;

    fn mul(self, rhs: Self) -> Self::Output {
        self.op(rhs, Op::Mul, |a,b| a * b)
    }
}
impl Div for Val {
    type Output = Val;

    fn div(self, rhs: Self) -> Self::Output {
        self.op(rhs, Op::Div, |a,b| a / b)
    }
}

#[derive(Copy, Clone, Debug)]
enum Op { Add, Sub, Mul, Div, RDiv }

impl Op {
    fn reverse_rhs(&self, rhs: i64, ops: &mut Vec<(Op, i64)>) {
        match self {
            // X = 4 + H
            // X - 4 = H
            Op::Add => { ops.push((Op::Sub, rhs)) }
            // X = H - 4
            // X + 4 = H
            Op::Sub => { ops.push((Op::Add, rhs)) }
            // X = H * 4
            // X / 4 = H
            Op::Mul => { ops.push((Op::Div, rhs)) }
            // X = H / 4
            // X * 4 = H
            Op::Div => { ops.push((Op::Mul, rhs)) }
            _ => unreachable!()
        }
    }
    fn reverse_lhs(&self, lhs: i64, ops: &mut Vec<(Op, i64)>) {
        match self {
            // X = 4 + H
            // X - 4 = H
            Op::Add => { ops.push((Op::Sub, lhs)); }
            // X = 4 - H
            // X = -H + 4
            // X - 4 = -H
            // (X - 4) * -1 = H
            Op::Sub => {
                ops.push((Op::Mul, -1));
                ops.push((Op::Sub, lhs));
            }
            // X = 4 * H
            // X / 4 = H
            Op::Mul => { ops.push((Op::Div, lhs)); }
            // X = 4 / H
            // XH = 4
            // H = 4 / X
            Op::Div => {
                ops.push((Op::RDiv, lhs));
            }
            _ => unreachable!()
        }
    }
    fn apply(&self, lhs: Val, rhs: Val) -> Val {
        match self {
            Op::Add => { lhs + rhs}
            Op::Sub => { lhs - rhs }
            Op::Mul => { lhs * rhs }
            Op::Div => { lhs / rhs }
            Op::RDiv => { rhs / lhs }
        }
    }
}
impl FromPair<Rule> for Op {
    fn from_pair(pair: Pair<Rule>) -> Self {
        match pair.as_str() {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            "/" => Op::Div,
            s => panic!("Bad op: {}", s),
        }
    }
}

#[derive(Clone, Debug)]
enum Job {
    Value(Num),
    Expr(String, Op, String)
}
impl FromPair<Rule> for Job {
    fn from_pair(pair: Pair<Rule>) -> Self {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::num => {
                Self::Value(inner.parse_into())
            },
            Rule::expr => {
                let mut pairs = inner.into_inner();
                Self::Expr(
                    pairs.parse_next(),
                    pairs.parse_next(),
                    pairs.parse_next(),
                )
            }
            r => panic!("Bad job: {:?}", r),
        }
    }
}

struct Interpreter {
    monkeys: HashMap<String, Job>,
    values: HashMap<String, Val>,
}

impl Interpreter {
    fn new(monkeys: HashMap<String, Job>) -> Self {
        Self { monkeys, values: Default::default() }
    }

    fn eval(&mut self, name: &str) -> Val {
        if let Some(num) = self.values.get(name) {
            num.clone()
        } else {
            let num = match self.monkeys.get(name).unwrap().clone() {
                Job::Value(num) => { Val::Num(num) }
                Job::Expr(a, op, b) => {
                    op.apply(self.eval(&a), self.eval(&b))
                }
            };
            self.values.insert(name.into(), num.clone());
            num
        }
    }
}

fn parse_input() -> HashMap<String,Job> {
    let (input,) : (Vec<(String,Job)>,) =
        InputParser::parse_file(Rule::input, "inputs/day21/input.txt");
    HashMap::from_iter(input.into_iter())
}

pub fn part1() {
    let monkeys = parse_input();
    let mut interp = Interpreter::new(monkeys);
    println!("{:?}", interp.eval("root"));
}

pub fn part2() {
    let monkeys = parse_input();
    if let Job::Expr(left, _, right) = monkeys.get("root").unwrap().clone() {
        let mut interp = Interpreter::new(monkeys);
        interp.values.insert("humn".into(), Val::Var(Vec::new()));
        match (interp.eval(&left), interp.eval(&right)) {
            (var@ Val::Var(_), num @ Val::Num(_)) |
            (num @ Val::Num(_), var @ Val::Var(_)) => {
                println!("{}", var.solve(num.num()).unwrap())
            }
            _ => unreachable!()
        }

    }
}