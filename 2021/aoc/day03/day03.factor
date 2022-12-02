USING: aoc.shared kernel sequences bit-arrays math locals
accessors memoize ;
IN: aoc.day03

TUPLE: counts zero one ;
: <counts> ( -- pos ) 0 0 counts boa ;
: extract-counts ( pos-counts -- zero one ) 
  [ zero>> ] [ one>> ] bi ;

: num-cols ( rows -- size ) first length ;

:: col ( rows col -- seq )
  rows [| row | col row nth ] map
;

: count-col ( col -- counts ) 
  <counts>
  [ 
    [ [ 1 + ] change-one ] [ [ 1 + ] change-zero ] if
  ] reduce
;

:: filter-rows ( rows i quot: ( counts -- ? ) -- rows )
  rows i col count-col quot call( c -- ? ) :> value
  rows [| row | i row nth value = ] filter
;

:: iterative-apply ( rows quot: ( counts -- ? ) -- row )
  rows num-cols <iota> rows
  [| rows i |
    rows length 1 >
    [ rows i quot filter-rows ]
    [ rows ]
    if
  ] reduce
;

! zeros > ones -> f, zeroes < ones -> t, zeroes = ones -> t
: most-common ( counts -- ? ) extract-counts <= ;

! zeros > ones -> t, zeroes < ones -> f, zeroes = ones -> f
: least-common ( counts -- ? ) extract-counts > ;

: map-as-int ( ... seq f: ( ... elt -- ... ? ) -- ... int )
  ?{ } map-as reverse bit-array>integer
; inline
 
: gamma ( input -- result )
  flip [ count-col most-common ] map-as-int
;

: epsilon ( input -- result ) 
  flip [ count-col least-common ] map-as-int
;

: oxygen-rating ( input -- result )
  [ most-common ] iterative-apply
  first [ ] map-as-int
;

: scrubber-rating ( input -- result )
  [ least-common ] iterative-apply
  first [ ] map-as-int
;

INPUT: [ [ CHAR: 1 = ] [ ] map-as ] map ;
PART1: [ gamma ] [ epsilon ] bi@ * ;
PART2: [ oxygen-rating ] [ scrubber-rating ] bi@ * ;
