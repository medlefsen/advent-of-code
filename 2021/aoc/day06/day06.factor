USING: kernel aoc.shared locals accessors memoize arrays
math.parser sequences splitting math assocs ;
IN: aoc.day06

: <fishes> ( seq -- fishes )
  V{ } clone [ over inc-at ] reduce 
;

:: process-fishes ( fishes -- fishes )
  fishes [| age num |
    age 0 = [ 8 ] [ age 1 - ] if
    num
  ] assoc-map :> new-fishes
  8 new-fishes at 0 or 6 new-fishes at+
  new-fishes
;

: fishes-size ( fishes -- n ) values sum ;

: count-after-days ( fishes days -- n ) 
  [ process-fishes ] times fishes-size
;

INPUT: 
  first "," split [ string>number ] map <fishes>
;
PART1: 80 count-after-days ;
PART2: 256 count-after-days ;
