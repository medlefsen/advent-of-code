USING: aoc.shared kernel splitting accessors math.parser
sequences math locals combinators arrays math.order sets assocs
sorting ;
IN: aoc.day09

TUPLE: iter x y hmap h ;
C: <iter> iter

: hmap-get ( x y hmap -- h/? ) ?nth { } or ?nth ;
:: move ( x y iter -- iter/? )
   x y iter hmap>> hmap-get :> h
   h { f 9 } in? [ f ] [ x y iter hmap>> h <iter> ] if
;

:: up ( iter -- iter ) iter x>> iter y>> 1 + iter move ; 
:: down ( iter -- iter ) iter x>> iter y>> 1 - iter move ; 
:: left ( iter -- iter ) iter x>> 1 - iter y>> iter move ; 
:: right ( iter -- iter ) iter x>> 1 + iter y>> iter move ; 

: adj ( iter -- seq )
  [ [ left ] [ up ] [ right ] [ down ] ] cleave 4array sift
;

: uphill-adj ( iter -- seq )
  dup adj [ h>> over h>> > ] filter swap drop ;

:: find-basin ( iter seen -- )
  iter uphill-adj seen without :> neigh
  iter seen adjoin
  neigh [ seen find-basin ] each 
;

: basin-size ( iter -- size )
  V{ } clone [ find-basin ] keep length
;

: adj-heights ( iter -- seq ) adj [ h>> ] map ;

: lowest-adj ( iter -- h ) adj-heights 9 [ min ] reduce ;
:: lowest? ( iter -- ? ) iter h>> iter lowest-adj < ;

:: hmap-each ( ... hmap quot: ( ... iter -- ... ) -- ... )
  hmap [| row y |
    row [| h x |
      x y hmap h <iter> quot call
    ] each-index 
  ] each-index
; inline

: hmap-reduce
( ... hmap iden quot: ( ... prev iter -- ... next ) -- ... r )
  swapd hmap-each
; inline

: find-lowpoints ( hmap -- low-points ) 
  V{ } clone [
    dup lowest? [ h>> over push ] [ drop ] if
  ] hmap-reduce
;

INPUT: [ [ 48 - ] { } map-as ] map ;
PART1: find-lowpoints [ 1 + ] map sum ;
PART2: V{ } clone [
  dup lowest? [ basin-size over push ] [ drop ] if
] hmap-reduce [ <=> ] sort 3 tail* product
;

