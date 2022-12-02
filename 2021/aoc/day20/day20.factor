USING: aoc.shared kernel sequences splitting locals accessors
aoc.matrix bit-arrays refs arrays math.ranges math delegate ;
IN: aoc.day20

TUPLE: image pixels fill-pixel ;
C: <image> image 
CONSULT: matrix image pixels>> ;

:: pixel-value ( iter -- ? )
  iter matrix-iter-valid? [ iter get-ref CHAR: # = ]
  [ iter matrix>> fill-pixel>> ] if
;

: algorithm-index ( iter -- i )
  adj-grid <reversed> [ pixel-value ] ?{ } map-as
  bit-array>integer
;

: fill-pixel ( algorithm image -- ? )
  fill-pixel>> [ last ] [ first ] if CHAR: # =
;

:: enhance ( image algorithm -- image )
  -2 image height 2 + (a,b] [| y |
     -2 image width 2 + (a,b] [| x |
      x y image <matrix-iter> algorithm-index
      algorithm nth
    ] "" map-as
  ] map
  algorithm image fill-pixel
  <image>
; 

: enhance-n ( algorithm image n -- image )
  [ over enhance ] times nip
;

: count-lit ( image -- n )
  pixels>> [ [ CHAR: # = ] count ] map-sum
;

INPUT: { "" } split1 [ first ] dip f <image> 2array ;
PART1: first2 2 enhance-n count-lit ;
PART2: first2 50 enhance-n count-lit ;
