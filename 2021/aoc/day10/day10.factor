USING: kernel aoc.shared splitting accessors locals sequences
sets combinators math continuations assocs memoize
sequences.extras sorting math.order ;
IN: aoc.day10

:: >slice ( str -- slice ) 0 str length str <slice> ;

ERROR: parse-error char pos ;
C: <parse-error> parse-error

TUPLE: parser input stack ;
: <parser> ( input -- parser ) >slice V{ } clone parser boa ;

: eoi? ( parser -- ? ) input>> empty? ;

MEMO: chunk-map ( -- assoc ) "[{<(" "]}>)" zip ;

: parse-next ( parser -- )
  dup input>> ?first
  { 
    { [ dup "[{<(" in? ] [ chunk-map at over stack>> push ] }
    { [ over stack>> last over = ] [ drop dup stack>> pop* ] }
    [ swap input>> from>> <parse-error> throw ]
  } cond
  [ rest-slice ] change-input drop
;

: error-score ( parse-error -- n )
  char>> {
   { CHAR: ) [ 3 ] }
   { CHAR: ] [ 57 ] }
   { CHAR: } [ 1197 ] }
   { CHAR: > [ 25137 ] }
   [ drop 0 ]
 } case
;

: parse ( str -- parser )
 <parser> [ dup eoi? ] [ dup parse-next ] until 
;

: score-stack ( seq -- score )
  reverse 0 [ ")]}>" index 1 + [ 5 * ] dip + ] reduce
;

: middle ( seq -- n )
  dup length 1 - 2 / swap nth
;

INPUT: ;
PART1: 0 [ 
 [ parse drop ] [ error-score nip + ] recover
] reduce
;

PART2: [ 
 [ parse stack>> score-stack ] [ 2drop f ] recover
] map sift [ <=> ] sort middle
;
