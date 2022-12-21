use std::fs::read_to_string;
use std::ops::Index;

#[derive(Clone, Debug)]
struct Node {
    next: usize,
    prev: usize,
    value: i64,
}

#[derive(Clone, Debug)]
struct List {
    nodes: Vec<Node>,
}

fn dec(i: usize, len: usize) -> usize {
    (if i == 0 { len } else { i }) - 1
}

fn inc(i: usize, len: usize) -> usize {
    let n = i + 1;
    if n == len { 0 } else { n }
}

impl List {
    fn new(values: Vec<i64>) -> Self {
        let len = values.len();
        Self {
            nodes: values.into_iter().enumerate().map(
                | (i, value) | Node { value, prev: dec(i,len), next: inc(i,len)}
            ).collect(),
        }
    }

    fn move_forward(&mut self, ind: usize) {
        let prev = self.nodes[ind].prev;
        let next = self.nodes[ind].next;
        let next_next = self.nodes[next].next;

        self.nodes[prev].next = next;
        self.nodes[next].prev = prev;

        self.nodes[ind].prev = next;
        self.nodes[next].next = ind;

        self.nodes[ind].next = next_next;
        self.nodes[next_next].prev = ind;
    }

    fn move_back(&mut self, ind: usize) {
       self.move_forward(self.nodes[ind].prev);
    }

    fn move_by(&mut self, ind: usize, mut amount: i64) {
        amount %= (self.len() as i64) - 1;
        if amount < 0 {
            for _ in amount..0 { self.move_back(ind); }
        } else {
            for _ in 0..amount { self.move_forward(ind); }
        }
    }

    fn find(&self, val: i64) -> Option<usize> {
        self.nodes.iter().enumerate().find(|(_, n)| n.value == val).map(|(i,_)| i )
    }

    fn prev(&self, ind: usize) -> usize {
        self.nodes[ind].prev
    }

    fn next(&self, ind: usize) -> usize {
        self.nodes[ind].next
    }

    fn advance(&self, mut ind: usize, mut amount: i64) -> usize {
        amount %= self.nodes.len() as i64;
        if amount < 0 {
            for _ in 0..amount { ind = self.prev(ind) }
        } else {
            for _ in 0..amount { ind = self.next(ind) }
        }
        ind
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn as_vec(&self) -> Vec<i64> {
        let mut node = 0;
        let mut vec = vec![self.nodes[node].value];
        while self.nodes[node].next != 0 {
            let old_node = node;
            node = self.nodes[node].next;
            if self.nodes[node].prev != old_node { panic!("{} != {}: {:?}", self.nodes[node].prev, old_node, self.nodes) }
            vec.push(self.nodes[node].value);
        }
        vec
    }
}

impl Index<usize> for List {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index].value
    }
}

fn read_input() -> Vec<i64> {
    read_to_string("inputs/day20/input.txt").unwrap()
        .lines().map(|s| s.parse().unwrap() ).collect()
}

pub fn part1() {
    let input = read_input();
    let mut list = List::new(input.clone());
    for (ind, amount) in input.iter().enumerate() {
        let mut mod_list = list.clone();
        list.move_by(ind, *amount);
    }
    let zero = list.find(0).unwrap();
    let total :i64 = [1000,2000,3000].iter().map(|amount| {
        list[list.advance(zero, *amount)]
    }).sum();
    println!("{}", total);
}

pub fn part2() {
    let decryption_key = 811589153;
    let input : Vec<_> = read_input().iter().map(|v| v * decryption_key).collect();
    let mut list = List::new(input.clone());
    for _ in 0..10 {
        for (ind, amount) in input.iter().enumerate() {
            list.move_by(ind, *amount);
        }
    }
    let zero = list.find(0).unwrap();
    let total :i64 = [1000,2000,3000].iter().map(|amount| {
        list[list.advance(zero, *amount)]
    }).sum();
    println!("{}", total);
}
