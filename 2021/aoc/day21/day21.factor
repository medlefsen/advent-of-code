USING: aoc.shared kernel splitting sequences locals math.parser
math literals accessors arrays assocs slots.syntax
sequences.repeating sequences.product hashtables ;
IN: aoc.day21

SYMBOL: p1
SYMBOL: p2

:: <board> ( p1pos p2pos -- board )
 ${ p1 p2 } p1pos p2pos 2array zip
; 

TUPLE: game-state board current scores ;
: <game-state> ( board -- game ) p1 V{ } clone game-state boa ;

: copy-game-state ( state -- state ) 
  clone [ [ clone ] map ] change-board 
  [ [ clone ] map ] change-scores
;

:: move-piece ( n state -- )
  state get[ current board ] [ n + 10 mod ] change-at
;

:: score-player ( state -- )
  state get[ current board ] at 1 +
  state get[ current scores ] at+
;

: switch-current ( state -- )
  [ p1 = [ p2 ] [ p1 ] if ] change-current drop
;

:: move-player ( rolls state -- )
   rolls sum state move-piece
   state score-player
   state switch-current
;

GENERIC: take-turn ( game -- )
GENERIC: game-over? ( game -- ? )
GENERIC: game-result ( game -- r )

TUPLE: practice-game state dice rolls ;
: <practice-game> ( board -- practice-game )
  <game-state> 1 0 practice-game boa
;

:: roll-practice-dice ( game -- n )
   game dice>> [
       1 + dup 100 > [ drop 1 ] when game dice<<
       game [ 1 + ] change-rolls drop
   ] keep
;

M:: practice-game take-turn ( game -- )
   3 [ game roll-practice-dice ] replicate
   game state>>
   move-player
;

M: practice-game game-over? ( game -- ? )
  state>> scores>> values [ f ] [ supremum 1000 >= ] if-empty
;

M: practice-game game-result ( game -- r )
  [ state>> scores>> values infimum ] [ rolls>> ] bi *
;

TUPLE: dirac-game universes ;
: <dirac-game> ( board -- game )
  <game-state> 1 2array 1array >hashtable dirac-game boa
;

:: update-universes ( game prev new-seq -- )
  prev game universes>> delete-at* drop :> prev-count
  new-seq [| state |
    prev-count state game universes>> at+
  ] each
;

: take-universe-turn ( game state -- )
  { { 1 2 3 } } 3 repeat [
    over copy-game-state [ move-player ] keep
  ] product-map
  update-universes
;

: game-universe-over? ( state -- ? )
  scores>> values [ f ] [ supremum 21 >= ] if-empty
;

M:: dirac-game take-turn ( game -- )
  game universes>> keys [| state |
    state game-universe-over? [
        game state take-universe-turn
    ] unless
  ] each
;

M: dirac-game game-over? ( game -- ? )
  universes>> keys [ game-universe-over? ] all?
;

: universe-winner ( state -- p )
  scores>> dup first2 [ second ] bi@ >
  [ first ] [ last ] if first
;

M:: dirac-game game-result ( game -- r )
  V{ } clone game universes>> [| state num |
    num state universe-winner pick at+
  ] assoc-each values supremum
;

: play-game ( game -- result )
  [ dup game-over? ] [ dup take-turn ] until game-result
;

INPUT:
  [ ": " split1 nip string>number 1 - ] map first2 <board>
;
PART1: <practice-game> play-game ;
PART2: <dirac-game> play-game ;
