USING: kernel math sequences math.parser grouping
aoc.shared ;
IN: aoc.day01

: count-increases ( seq -- n ) 2 <clumps> [ first2 < ] count ;
: sum-windows ( seq -- seq ) 3 <clumps> [ sum ] map ;

INPUT: [ string>number ] map ;
PART1: count-increases ;
PART2: sum-windows count-increases ;
