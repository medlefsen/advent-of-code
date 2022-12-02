USING: aoc.shared aoc.matrix kernel splitting sequences locals
accessors math sets heaps slots.syntax refs math.vectors arrays
assocs sequences.extras combinators.short-circuit.smart
math.parser heaps.private ;
IN: aoc.day15

TUPLE: pos risk visited cost heap-item ; final
: <pos> ( risk -- pos ) f f f pos boa ;

: visited? ( iter -- ? ) get-ref visited>> ;
: visited! ( iter -- ) get-ref t swap visited<< ;

: set-cost ( cost iter -- ) get-ref cost<< ;
: get-cost ( iter -- cost ) get-ref get[ cost ] ;
: get-risk ( iter -- risk ) get-ref get[ risk ] ;

: closer? ( cost iter -- ? ) get-cost [ < ] [ drop t ] if* ;

:: update-heap-item ( heap-item -- )
  heap-item get[ value ] get-cost :> new-cost
  heap-item new-cost set[ key ] drop
  heap-item get[ heap index ] 0 swap sift-down
;

: queue-push ( iter queue -- )
  over visited? [ 2drop ] [| iter queue |
    iter get-ref get[ heap-item ] [
      update-heap-item
    ] [ 
     iter iter get-cost queue heap-push*
     iter get-ref heap-item<<
    ] if*
  ] if
;

: queue-pop ( queue -- iter )
  heap-pop drop f over get-ref heap-item<<
;

:: update-cost ( cur nei -- )
  cur get-cost nei get-risk + :> cost
  cost nei closer? [ cost nei set-cost ] when
;

:: dijkstra ( from to -- cost )
  <min-heap> :> queue
  0 from set-cost

  from
  [ dup to = ]
  [| cur |
    cur visited!
    cur neigh [| iter |
      cur iter update-cost
      iter queue queue-push
    ] each
    queue queue-pop
  ] until
  get-cost
; 

: from-to ( matrix -- from to )
  [ [ 0 0 ] dip <matrix-iter> ]
  [ [ [ width 1 - ] [ height 1 - ] bi ] keep <matrix-iter> ]
  bi
;

:: shift-matrix ( matrix n -- matrix )
  matrix [
    [ 
      get[ risk ] n + dup 9 > [ 9 - ] when <pos>
    ] map
  ] map
;

:: expand-matrix ( matrix n -- matrix )
  n <iota> [| y |
    n <iota> [| x |
       matrix x y + shift-matrix
    ] map
    unclip [ [ append ] 2map ] reduce
  ] map-concat
;

INPUT: [ [ 48 - <pos> ] { } map-as ] map ;
PART1: from-to dijkstra ;
PART2: 5 expand-matrix from-to dijkstra ;
