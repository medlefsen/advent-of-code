USING: kernel locals aoc.shared splitting math.parser sequences
memoize math.statistics math math.ranges ;
IN: aoc.day07

:: linear-alignment-fuel ( pos crabs -- n )
  crabs [ pos - abs ] map sum
;

:: linear-alignment-options ( crabs -- seq )
  crabs minmax 1 <range>
  [ crabs linear-alignment-fuel ] map
;

: lowest-linear-alignment-fuel ( crabs -- n )
 linear-alignment-options minmax drop
;

:: exp-alignment-fuel ( pos crabs -- n )
  crabs [ pos - abs dup 2 / 1/2 + * ] map sum
;

:: exp-alignment-options ( crabs -- seq )
  crabs minmax 1 <range>
  [ crabs exp-alignment-fuel ] map
;

: lowest-exp-alignment-fuel ( crabs -- n )
 exp-alignment-options minmax drop
;

INPUT: first "," split [ string>number ] map ;
PART1: lowest-linear-alignment-fuel ;
PART2: lowest-exp-alignment-fuel ;
