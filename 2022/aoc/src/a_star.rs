use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;
use std::cmp::{Ordering, Reverse};

pub trait AStarNode: Sized + Clone + Eq + Hash {
    fn neighbors(&self) -> Vec<Self>;
    fn estimate_cost_to(&self, other: &Self) -> usize;
}

pub fn a_star<Node>(start: Node, end: Node) -> Option<Vec<Node>>
    where Node: AStarNode
{
    let mut a_star = AStar::new();
    a_star.start(start, &end);

    while let Some(cur) = a_star.next() {
        if cur == end {
            return Some(a_star.path(&cur));
        }

        let score = a_star.score(&cur) + 1;
        for edge in cur.neighbors() {
            if score < a_star.score(&edge) {
                a_star.update_best_path(&cur, &edge, &end, score);
            }
        }
    }
    None
}

struct AStar<Node: AStarNode> {
    enqueued : HashSet<Node>,
    queue : BinaryHeap<WeightedNode<Node>>,
    came_from : HashMap<Node, Node>,
    scores : HashMap<Node, usize>,
}

impl<Node> AStar<Node>
    where Node: AStarNode
{
    fn new() -> Self {
        Self {
            enqueued: Default::default(),
            queue: Default::default(),
            came_from: Default::default(),
            scores: Default::default(),
        }
    }

    fn start(&mut self, start: Node, end: &Node) {
        self.scores.insert(start.clone(), 0);
        self.enqueued.insert(start.clone());
        self.queue.push(WeightedNode::new(start.clone(), start.estimate_cost_to(&end)));
    }

    fn score(&self, node: &Node) -> usize {
        self.scores.get(node).copied().unwrap_or(usize::MAX)
    }

    fn update_best_path(&mut self, from: &Node, to: &Node, end: &Node, score: usize) {
        self.scores.insert(to.clone(), score);
        self.came_from.insert(to.clone(), from.clone());
        if !self.enqueued.contains(&to) {
            self.enqueued.insert(to.clone());
            self.queue.push(WeightedNode::new(to.clone(), score + to.estimate_cost_to(&end)));
        }
    }

    fn path<'a, 'b: 'a>(&'b self, mut to: &'a Node) -> Vec<Node> {
        let mut path : Vec<Node> = vec![to.clone()];
        while self.came_from.contains_key(to) {
            to = &self.came_from[to];
            path.push(to.clone());
        }
        path.reverse();
        return path;
    }

    fn next(&mut self) -> Option<Node> {
        self.queue.pop().map(|weighted_node| {
            self.enqueued.remove(&weighted_node.node);
            weighted_node.node
        })
    }
}

#[derive(Eq)]
struct WeightedNode<T: AStarNode> {
    weight: Reverse<usize>,
    node: T,
}

impl<T: AStarNode> WeightedNode<T> {
    fn new(node: T, weight: usize) -> Self {
        Self { node, weight: Reverse(weight) }
    }
}

impl<T: AStarNode> PartialEq for WeightedNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl<T: AStarNode> PartialOrd for WeightedNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl<T: AStarNode> Ord for WeightedNode<T>
    where WeightedNode<T>: Eq
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}
