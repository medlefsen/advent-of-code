USING: delegate arrays vectors accessors math refs slots.syntax
locals combinators.short-circuit.smart sequences kernel
combinators.smart ;
IN: aoc.matrix

MIXIN: matrix
GENERIC: width ( matrix -- n )
GENERIC: height ( matrix -- n )
GENERIC: matrix-at ( x y matrix -- n )
GENERIC: matrix-set-at ( v x y matrix -- )

M: matrix width ( matrix -- n )
  ?first [ length ] [ 0 ] if* 
;
M: matrix height ( matrix -- n ) length ;
M: matrix matrix-at ( x y matrix -- n ) nth nth ;
M: matrix matrix-set-at ( v x y matrix -- ) nth set-nth ;

PROTOCOL: matrix width height matrix-at matrix-set-at ;

INSTANCE: array matrix
INSTANCE: vector matrix

TUPLE: matrix-seq matrix ;
C: <matrix-seq> matrix-seq

TUPLE: matrix-iter x y matrix ;
C: <matrix-iter> matrix-iter

INSTANCE: matrix-iter ref 
M: matrix-iter get-ref ( ref -- obj )
  get[ x y matrix ] matrix-at
;
M: matrix-iter set-ref ( obj ref -- )
  get[ x y matrix ] matrix-set-at
;

: matrix-iter-valid? ( iter -- ? )
 {
   [ get[ y ] 0 >= ]
   [ get[ x ] 0 >= ]
   [ get[ y matrix ] width < ]
   [ get[ x matrix ] width < ]
 } &&
;

GENERIC: move ( x y iter -- iter/? )
GENERIC: up ( iter -- iter/? )
GENERIC: down ( iter -- iter/? )
GENERIC: left ( iter -- iter/? )
GENERIC: right ( iter -- iter/? )
GENERIC: leftup ( iter -- iter/? )
GENERIC: leftdown ( iter -- iter/? )
GENERIC: rightup ( iter -- iter/? )
GENERIC: rightdown ( iter -- iter/? )
GENERIC: adj ( iter -- seq )
MIXIN: matrix-iterator
INSTANCE: matrix-iter matrix-iterator

: matrix-iter-at ( x y matrix -- iter/? )
 <matrix-iter> dup matrix-iter-valid? [ drop f ] unless
;

M: matrix-iter move ( x y iter -- iter/? )
  matrix>> matrix-iter-at
;

M:: matrix-iterator up ( iter -- iter )
  iter x>> iter y>> 1 - iter move 
;
M:: matrix-iterator down ( iter -- iter )
  iter x>> iter y>> 1 + iter move
; 
M:: matrix-iterator left ( iter -- iter )
  iter x>> 1 - iter y>> iter move
; 
M:: matrix-iterator right ( iter -- iter )
  iter x>> 1 + iter y>> iter move
; 
M:: matrix-iterator leftup ( iter -- iter )
  iter x>> 1 - iter y>> 1 - iter move 
;
M:: matrix-iterator rightdown ( iter -- iter )
  iter x>> 1 + iter y>> 1 + iter move
; 
M:: matrix-iterator leftdown ( iter -- iter )
  iter x>> 1 - iter y>> 1 + iter move
; 
M:: matrix-iterator rightup ( iter -- iter )
  iter x>> 1 + iter y>> 1 - iter move
; 

M: matrix-iterator adj ( iter -- seq )
  {
    [ left ] [ up ] [ down ] [ right ]
    [ leftup ] [ rightup ] [ leftdown ] [ rightdown ]
  } cleave>array sift
;

:: up-unsafe ( iter -- iter )
  iter x>> iter y>> 1 - iter matrix>> <matrix-iter> 
;
:: down-unsafe ( iter -- iter )
  iter x>> iter y>> 1 + iter matrix>> <matrix-iter>
; 
:: left-unsafe ( iter -- iter )
  iter x>> 1 - iter y>> iter matrix>> <matrix-iter>
; 
:: right-unsafe ( iter -- iter )
  iter x>> 1 + iter y>> iter matrix>> <matrix-iter>
; 
:: leftup-unsafe ( iter -- iter )
  iter x>> 1 - iter y>> 1 - iter matrix>> <matrix-iter> 
;
:: rightdown-unsafe ( iter -- iter )
  iter x>> 1 + iter y>> 1 + iter matrix>> <matrix-iter>
; 
:: leftdown-unsafe ( iter -- iter )
  iter x>> 1 - iter y>> 1 + iter matrix>> <matrix-iter>
; 
:: rightup-unsafe ( iter -- iter )
  iter x>> 1 + iter y>> 1 - iter matrix>> <matrix-iter>
; 

: adj-grid ( iter -- seq )
  {
    [ leftup-unsafe ] [ up-unsafe ] [ rightup-unsafe ]
    [ left-unsafe ] [ ] [ right-unsafe ]
    [ leftdown-unsafe ] [ down-unsafe ] [ rightdown-unsafe ]
  } cleave>array
;
: neigh ( iter -- seq )
  {
    [ left ] [ up ] [ down ] [ right ]
  } cleave>array sift
;


INSTANCE: matrix-seq immutable-sequence

M: matrix-seq length ( matrix-seq -- n )
  matrix>> [ width ] [ height ] bi *
;

M: matrix-seq nth ( n matrix-seq -- elt )
  matrix>> [ width /mod swap ] keep <matrix-iter>
;

M: matrix-seq set-nth ( elt n matrix-seq -- )
  matrix>> [ width /mod swap ] keep nth set-nth
;
