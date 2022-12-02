USING: kernel aoc.shared sequences locals accessors math
splitting aoc.matrix slots.syntax sets math.parser hash-sets
math.order io ;
IN: aoc.day13

TUPLE: instructions dots folds ;
C: <instructions> instructions

TUPLE: dot x y ;
C: <dot> dot

TUPLE: fold dir coord ;
C: <fold> fold

:: translate-coord ( fold coord -- coord )
  coord fold <
  [ coord ]
  [ fold 0 coord fold - - + ]
  if
;

:: fold-vert ( dots y -- dots )
  dots [| dot |
    dot x>> 
    y dot y>> translate-coord
    <dot>
  ] map >hash-set members
;

:: fold-horiz ( dots x -- dots )
  dots [| dot |
    x dot x>> translate-coord
    dot y>>
    <dot>
  ] map >hash-set members
;

: apply-fold ( dots fold -- dots )
  get[ coord dir ] "x" = [
    fold-horiz
  ] [
    fold-vert
  ] if
; 

: parse-dots ( lines -- dots )
  [ "," split [ string>number ] map first2 <dot> ] map
;

: parse-folds ( lines -- folds )
  [ " " split last "=" split first2 string>number <fold> ] map
;

:: print-dots ( dots -- str )
  dots unclip
  [ [ x>> ] [ y>> ] [ [ bi@ max ] curry ] bi@ 2bi <dot> ]
  reduce get[ x y ] [ 1 + <iota> ] bi@ :> ( xs ys )
  ys [| y | xs [| x |
      x y <dot> dots member? [ "#" ] [ "." ] if
    ] map concat
  ] map "\n" join
;

: apply-instructions ( instructions -- dots )
  get[ folds dots ] [ apply-fold ] reduce
;


INPUT: { "" } split first2 [ parse-dots ] [ parse-folds ] bi*
<instructions> ;

PART1: get[ dots folds ] first apply-fold length ;
PART2: apply-instructions print-dots ;
