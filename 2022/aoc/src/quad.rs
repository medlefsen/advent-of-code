use std::cmp::Ordering;
use std::fmt::Debug;
use std::iter::Sum;
use std::ops::{Add, Index, IndexMut, Mul, Sub};

#[derive(Default, Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Quad<T>([T; 4]);

impl<T: Copy> Quad<T> {
    pub fn new(a: T, b: T, c: T, d: T) -> Self {
        Self([a,b,c,d])
    }
}
impl<T: Copy + Default> Quad<T> {
    pub fn from_input<I>(costs: &Vec<(T, I)>) -> Self
        where I : Into<usize> + Copy
    {
        let mut cust_arr: [T; 4] = Default::default();
        for (cost, index) in costs {
            cust_arr[(*index).into()] = *cost;
        }
        return Self(cust_arr);
    }
}
impl<T: Copy + Add<Output=T>> Quad<T> {
    pub fn sum(&self) -> T::Output {
        self[0] + self[1] + self[2] + self[3]
    }
}
impl<T: Copy + Ord> Quad<T> {
    pub fn clamp(&self, min: T, max: T) -> Self {
        Self::new(
            self[0].clamp(min,max),
            self[1].clamp(min,max),
            self[2].clamp(min,max),
            self[3].clamp(min,max),
        )
    }
}
impl<T: Copy> Quad<T> {
    pub fn pw_mul<OT>(self, rhs: Quad<OT>) -> Quad<T::Output>
    where
        OT: Copy,
        T: Mul<OT>,
        T::Output: Copy,
    {
        Quad::new(
            self[0] * rhs[0],
            self[1] * rhs[1],
            self[2] * rhs[2],
            self[3] * rhs[3],
        )
    }
}

impl<T> Index<usize> for Quad<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for Quad<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: PartialOrd> PartialOrd for Quad<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self < other {
            Some(Ordering::Less)
        } else if self > other {
            Some(Ordering::Greater)
        } else if self == other {
            Some(Ordering::Equal)
        } else {
            None
        }
    }

    fn lt(&self, other: &Self) -> bool {
        self[0] < other[0] &&
            self[1] < other[1] &&
            self[2] < other[2] &&
            self[3] < other[3]
    }

    fn gt(&self, other: &Self) -> bool {
        self[0] > other[0] &&
            self[1] > other[1] &&
            self[2] > other[2] &&
            self[3] > other[3]
    }

    fn ge(&self, other: &Self) -> bool {
        self[0] >= other[0] &&
            self[1] >= other[1] &&
            self[2] >= other[2] &&
            self[3] >= other[3]
    }

    fn le(&self, other: &Self) -> bool {
        self[0] <= other[0] &&
            self[1] <= other[1] &&
            self[2] <= other[2] &&
            self[3] <= other[3]
    }
}

impl<T: Add<Output = T> + Copy> Add for Quad<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
            self.0[3] + rhs.0[3],
        ])
    }
}

impl<T: Sub<Output = T> + Copy> Sub for Quad<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self[0] - rhs[0],
            self[1] - rhs[1],
            self[2] - rhs[2],
            self[3] - rhs[3],
        )
    }
}

impl<T: Mul<Output = T> + Copy> Mul for Quad<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Quad::new(
            self[0] * rhs[0],
            self[1] * rhs[1],
            self[2] * rhs[2],
            self[3] * rhs[3],
        )
    }
}

impl<T: Mul<T, Output = T> + Copy> Mul<T> for Quad<T> {
    type Output = Quad<T::Output>;

    fn mul(self, rhs: T) -> Quad<T::Output> {
        Quad::new(
            self[0] * rhs,
            self[1] * rhs,
            self[2] * rhs,
            self[3] * rhs,
        )
    }
}

impl<T: Add<Output=T> + Copy + Default> Sum for Quad<T> {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.reduce(|a,b| a + b).unwrap_or_default()
    }
}
