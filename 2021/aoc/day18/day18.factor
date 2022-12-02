USING: kernel aoc.shared splitting math.parser math sequences
locals arrays accessors peg.ebnf multiline strings slots.syntax
sequences.extras dlists math.functions sequences.product
combinators.short-circuit.smart deques math.order ;
IN: aoc.day18

TUPLE: elem value depth ;
C: <elem> elem

:: explode-left ( node -- )
  node obj>> :> cur
  node prev>> [
    obj>> [ cur value>> + ] change-value drop
  ] when*
  cur [ 1 - ] change-depth drop
  0 cur value<<
;

:: explode-right ( node list -- )
  node next>> [
   obj>> [ node obj>> value>> + ] change-value drop 
  ] when*
  node list delete-node
;

:: explode ( node list -- )
  node explode-left
  node next>> list explode-right
;

: find-explodable ( list -- node/f )
  [ obj>> depth>> 4 > ] dlist-find-node
;

:: split ( node list -- )
  node obj>> :> cur
  cur [ 1 + ] change-depth drop
  cur value>> 2 / [ floor ] [ ceiling ] bi :> ( lval rval )
  lval cur value<<
  rval cur depth>> <elem> node node next>> <dlist-node>
  :> new-node
  new-node [ set-next-prev ] [ set-prev-next ] bi
  list back>> node eq? [ new-node list back<< ] when
;

: find-split ( list -- i/f )
  [ obj>> value>> 10 >= ] dlist-find-node
;

:: reduce-num ( num -- ? )
  num {
    [ find-explodable dup [ num explode ] when* ]
    [ find-split dup [ num split ] when* ]
  } ||
;

: reduce-all ( num -- num )
  [ dup reduce-num ] loop
;

: add-num ( n n2 -- n )
  over back>> over front>>
  [ prev<< ] [ set-prev-next ] bi
  back>> over back<<
  dup [ [ 1 + ] change-depth drop ] dlist-each
  reduce-all
;

: pair-mag ( node -- n )
  dup next>> [ obj>> value>> ] bi@ [ 3 * ] dip 2 * +
;

: pair? ( node -- ? )
  dup next>> [ [ obj>> depth>> ] bi@ = ] [ drop f ] if*
;
: next-pair ( list -- node/f ) [ pair? ] dlist-find-node ;

:: reduce-pair-mag ( node list -- )
   node pair-mag node obj>> [ 1 - ] change-depth value<<
   node next>> list delete-node
;

: num-mag ( num -- n )
  [ dup next-pair dup ]
  [ over reduce-pair-mag ]
  while drop
  front>> obj>> value>>
; 

: copy-num ( num -- num )
  dlist>sequence [ clone ] map >dlist
;

:: (tree>seq) ( tree depth -- seq )
  tree sequence?
  [ tree [ depth 1 + (tree>seq) ] map-concat ]
  [ tree depth <elem> 1array ]
  if
; inline recursive

: tree>list ( tree -- list )
  0 (tree>seq) >dlist
;

EBNF: snailfish-num [=[
  num = [0-9] => [[ 48 - ]]
  pair = "[" elem "," elem "]" => [[ { 1 3 } swap nths ]]
  elem = { pair | num }
  rule = elem => [[ tree>list ]]
]=]

INPUT: [ snailfish-num ] map ;
PART1: unclip [ add-num ] reduce num-mag ;
PART2: dup 2array
  <product-sequence> [ first2 eq? not ] filter
  [ first2 [ copy-num ] bi@ add-num num-mag ] map
  unclip [ max ] reduce
;
