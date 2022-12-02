USING: kernel splitting aoc.shared math math.parser locals
accessors sequences memoize regexp arrays math.vectors io
math.order combinators assocs ;
IN: aoc.day05

: <pos> ( x y -- pos ) 2array ;
:: x>> ( pos -- x ) 0 pos nth ;
:: y>> ( pos -- y ) 1 pos nth ;

TUPLE: line start end ;
C: <line> line

:: next-num ( n e -- n )
  n e <=> { 
   { +lt+ [ n 1 + ] }
   { +eq+ [ n ] }
   { +gt+ [ n 1 - ] }
  } case
;

:: each-line-pos ( line quot: ( pos -- ) -- )
  line start>>
  [ dup line end>> = ]
  [| pos |
    pos quot call
    pos x>> line end>> x>> next-num
    pos y>> line end>> y>> next-num
    <pos> 
  ]
  until quot call
; inline

:: overlaps ( lines -- overlaps )
  H{ } clone :> overlaps
  lines [| line |
    line [ overlaps inc-at ] each-line-pos
  ] each
  overlaps
;

: count-multiple-overlaps ( lines -- n )
  overlaps values [ 1 > ] count
;

: without-diag ( lines -- lines )
  [| line |
    line start>> x>> line end>> x>> =
    line start>> y>> line end>> y>> =
    or
  ] filter
;

INPUT: [
  " -> " split1
  [ "," split1 [ string>number ] bi@ <pos> ] bi@
  <line>
] map ;
PART1: without-diag count-multiple-overlaps ;
PART2: count-multiple-overlaps ;
