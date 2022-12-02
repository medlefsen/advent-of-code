USING: aoc.shared grouping kernel locals math math.parser
memoize regexp sequences splitting accessors ;
IN: aoc.day04

TUPLE: board-pos value marked ;
: <board-pos> ( value -- board-pos ) f board-pos boa ;

TUPLE: bingo inputs boards ;
C: <bingo> bingo

:: mark-board ( board value -- )
 board [ 
   [| pos |
    pos value>> value =
    [ t pos marked<< ] [ ] if
   ] each
 ] each
;

:: mark-boards ( bingo value -- )
  bingo boards>> [ value mark-board ] each
;

:: board-won? ( board -- ? )
  board [ [ marked>> ] all? ] any?
  board flip [ [ marked>> ] all? ] any?
  or
;

: winning-boards ( bingo -- seq )
  boards>> [ board-won? ] filter
;

: score-board ( board -- score )
[
    [ marked>> not ] filter [ value>> ] map sum
] map sum
;

:: bingo-play-next ( bingo -- scores )
  bingo inputs>> pop :> input
  bingo input mark-boards
  bingo boards>> [ board-won? ] partition :> losers :> winners
  losers bingo boards<<
  winners [ score-board input * ] map
;

:: bingo-play-until-winner ( bingo -- score )
  [ ]
  [ dup empty? ] 
  [ drop bingo bingo-play-next ]
  while
;

:: bingo-play-until-last-winner ( bingo -- score )
  [ ]
  [ bingo boards>> empty? ] 
  [ drop bingo bingo-play-next ]
  until
;

: parse-board-row ( str -- row )
  R/ \d+/ all-matching-slices [ string>number <board-pos> ] map
;

: parse-inputs ( str -- inputs ) 
  "," split <reversed> [ string>number ] V{ } map-as
;

:: parse-bingo ( rows -- bingo )
  rows first parse-inputs :> inputs
  rows rest-slice 6 <groups>
  [
    rest-slice [ parse-board-row ] map
  ] map :> boards
  inputs boards <bingo> 
;

INPUT: parse-bingo ;
PART1: bingo-play-until-winner first ;
PART2: bingo-play-until-last-winner first ;
