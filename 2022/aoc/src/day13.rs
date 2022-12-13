use std::cmp::Ordering;
use std::rc::Rc;
use pest::iterators::Pair;
use crate::parsing::{FromPair, ParseFile, ParseInto, ParseNext};

#[derive(Parser)]
#[grammar = "src/day13.pest"]
struct InputParser;

type List = Vec<Elem>;
#[derive(Clone, Debug, PartialEq, Eq)]
enum Elem {
    Num(i32),
    List(Rc<List>),
}

impl PartialOrd<Self> for Elem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Elem {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Num(l), Self::Num(r)) => l.cmp(r),
            (Self::List(l), Self::List(r)) => {
                l.iter().zip(r.iter()).find_map(|(a,b)| {
                    let or = a.cmp(b);
                    if or.is_eq() { None } else { Some(or) }
                }).unwrap_or(l.len().cmp(&r.len()))
            },
            (l @ Self::Num(_), r @ Self::List(_)) =>
                Self::List(Rc::new(vec![l.clone()])).cmp(r),
            (l @ Self::List(_), r @ Self::Num(_)) =>
                l.cmp(&Self::List(Rc::new(vec![r.clone()]))),
        }
    }
}

impl FromPair<Rule> for Elem {
    fn from_pair(pair: Pair<Rule>) -> Self {
        let inner_pair = pair.into_inner().next().unwrap();

        match inner_pair.as_rule() {
            Rule::num => Elem::Num(inner_pair.parse_into()),
            Rule::list => Elem::List(inner_pair.parse_into()),
            _ => panic!("Unexpected elem: {}", inner_pair)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Packet {
  list: List
}

impl FromPair<Rule> for Packet {
    fn from_pair(pair: Pair<Rule>) -> Self {
        let mut pairs = pair.into_inner();
        Self { list: pairs.parse_next() }
    }
}

#[derive(Debug)]
struct Input {
  packet_pairs: Vec<(Packet, Packet)>
}
impl FromPair<Rule> for Input {
    fn from_pair(pair: Pair<Rule>) -> Self {
        let mut pairs = pair.into_inner();
        Self { packet_pairs: pairs.parse_next() }
    }
}

fn parse_input() -> Input {
    InputParser::parse_file(Rule::input, "inputs/day13/input.txt")
}

fn divider_packet(n: i32) -> Packet {
    Packet { list: vec![Elem::List(
        Rc::new(vec![Elem::Num(n)])
    )]}
}

pub fn part1() {
    let Input { packet_pairs } = parse_input();
    let sum : usize = packet_pairs.iter().enumerate().filter(|(_,(p1, p2))| {
        p1 <= p2
    }).map(|(i,_)| i + 1).sum();
    println!("{:?}", sum);
}

pub fn part2() {
    let Input { packet_pairs } = parse_input();
    let mut packets : Vec<_> = packet_pairs.into_iter().map(|(a,b)| [a,b]).flatten().collect();
    let div1 = divider_packet(2);
    let div2 = divider_packet(6);
    packets.push(div1.clone());
    packets.push(div2.clone());
    packets.sort();
    let score = (packets.binary_search(&div1).unwrap() + 1) *
        (packets.binary_search(&div2).unwrap() + 1);
    println!("{}", score);
}