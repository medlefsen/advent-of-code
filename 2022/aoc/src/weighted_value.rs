use std::cmp::Ordering;

#[derive(Debug)]
pub struct WeightedValue<W: PartialEq + Eq + PartialOrd + Ord + Clone, T: Clone> {
    pub weight: W,
    pub value: T,
}

impl<W: PartialEq + Eq + PartialOrd + Ord + Clone, T: Clone> WeightedValue<W,T> {
    pub fn new(value: T, weight: W) -> Self {
        Self { value, weight }
    }
}

impl<W: PartialEq + Eq + PartialOrd + Ord + Clone, T: Clone> PartialEq for WeightedValue<W, T> {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl<W: PartialEq + Eq + PartialOrd + Ord + Clone, T: Clone> Eq for WeightedValue<W, T> {}

impl<W: PartialEq + Eq + PartialOrd + Ord + Clone, T: Clone> PartialOrd for WeightedValue<W,T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl<W:PartialEq + Eq + PartialOrd + Ord + Clone, T: Clone> Ord for WeightedValue<W, T>
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}
