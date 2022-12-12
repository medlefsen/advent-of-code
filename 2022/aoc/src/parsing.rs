use std::fmt::Debug;
use std::str::FromStr;
use pest::iterators::Pair;
use pest::RuleType;

pub trait FromPair<R> {
    fn from_pair(pair: Pair<R>) -> Self;
}

pub trait ParseInto<T> {
    fn parse_into(self) -> T;
}

impl<'a, R, T: FromPair<R>> ParseInto<T> for Pair<'a, R> {
    fn parse_into(self) -> T {
        T::from_pair(self)
    }
}

pub trait ParseNext<T> {
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

impl<R: RuleType, T: FromPair<R>> FromPair<R> for Vec<T> {
    fn from_pair(pair: Pair<R>) -> Self {
        pair.into_inner().map(|r| r.parse_into() ).collect()
    }
}

macro_rules! type_as_parse_next {
    ($v:ident, $t:ident) => { $v.parse_next() }
}

macro_rules! tuple_from_pair {
  ($( $t:ident ),+) => {
    impl<
        R: RuleType,
        $($t: FromPair<R>),+
    > FromPair<R> for ($($t),+,) {
        fn from_pair(pair: Pair<R>) -> Self {
            let mut pairs = pair.into_inner();
            (
                $( type_as_parse_next!(pairs, $t) ),+
            )
        }
    }
  }
}

tuple_from_pair!(T1);
tuple_from_pair!(T1, T2);
tuple_from_pair!(T1, T2, T3);
tuple_from_pair!(T1, T2, T3, T4);
tuple_from_pair!(T1, T2, T3, T4, T5);
tuple_from_pair!(T1, T2, T3, T4, T5, T6);
tuple_from_pair!(T1, T2, T3, T4, T5, T6, T7);
tuple_from_pair!(T1, T2, T3, T4, T5, T6, T7, T8);

trait FromPairStr: FromStr {}

impl<R,T> FromPair<R> for T
where
    R: RuleType,
    T: FromStr + FromPairStr,
    T::Err: Debug,

{
    fn from_pair(pair: Pair<R>) -> Self {
        pair.as_str().parse().unwrap()
    }
}

impl FromPairStr for i32 {}
impl FromPairStr for u32 {}
impl FromPairStr for i64 {}
impl FromPairStr for u64 {}
impl FromPairStr for usize {}
