USING: kernel aoc.shared sequences splitting math.parser math
math.bits slots.syntax accessors locals combinators
sequences.cords sequences.repeating math.order ;
IN: aoc.day16

TUPLE: bit-stream bits pos ;
C: <bit-stream> bit-stream

TUPLE: packet
 { version integer }
 { type integer }
 { data object }
;
C: <packet> packet

PREDICATE: operator-pkt < packet type>> 4 = not ;
PREDICATE: sum-pkt < packet type>> 0 = ;
PREDICATE: product-pkt < packet type>> 1 = ;
PREDICATE: min-pkt < packet type>> 2 = ;
PREDICATE: max-pkt < packet type>> 3 = ;
PREDICATE: literal-pkt < packet type>> 4 = ;
PREDICATE: gt-pkt < packet type>> 5 = ;
PREDICATE: lt-pkt < packet type>> 6 = ;
PREDICATE: eq-pkt < packet type>> 7 = ;

GENERIC: eval-pkt ( pkt -- n )

: eval-subpkts ( pkt -- seq ) get[ data ] [ eval-pkt ] map ;

M: sum-pkt eval-pkt ( pkt -- n ) eval-subpkts sum ;
M: product-pkt eval-pkt ( pkt -- n ) eval-subpkts product ;
M: min-pkt eval-pkt ( pkt -- n )
  eval-subpkts unclip [ min ] reduce
;

M: max-pkt eval-pkt ( pkt -- n )
  eval-subpkts unclip [ max ] reduce
;

M: literal-pkt eval-pkt ( pkt -- n ) get[ data ] ;
M: gt-pkt eval-pkt ( pkt -- n ) eval-subpkts first2 > 1 0 ? ;
M: lt-pkt eval-pkt ( pkt -- n ) eval-subpkts first2 < 1 0 ? ;
M: eq-pkt eval-pkt ( pkt -- n ) eval-subpkts first2 = 1 0 ? ;

:: bits-at ( start size bits -- bits )
  start start size + bits subseq
;

: >num ( bits -- n ) <reversed> bits>number ;

:: next-bits ( size stream -- bits )
  stream get[ pos bits ] size swap bits-at
  [ stream [ size + ] change-pos drop ] dip
;

: next-num ( size stream -- n ) next-bits >num ;

<PRIVATE
: push-bits ( bits n stream -- ) next-bits append! drop ;

:: (parse-literal) ( bits stream -- )
  1 stream next-num 1 =
  bits 4 stream push-bits
  [ bits stream (parse-literal) ] when
; inline recursive

PRIVATE>

: parse-literal ( stream -- data )
  V{ } clone [ swap (parse-literal) ] keep >num
;

DEFER: parse-packet
:: parse-n-packets ( n stream -- packets )
  n <iota> [ drop stream parse-packet ] map
;

:: parse-bit-packets ( n-bits stream -- packets )
  stream get[ pos ] n-bits + :> end
  [ stream get[ pos ] end < ]
  [ stream parse-packet ]
  produce
;

:: parse-sub-packets ( stream -- packets )
  1 stream next-num 0 = [ 
    15 stream next-num stream parse-bit-packets
  ] [
    11 stream next-num stream parse-n-packets
  ] if
;

:: parse-packet ( stream -- packet )
  3 stream next-num
  3 stream next-num
  dup 4 =
  [ stream parse-literal ] [ stream parse-sub-packets ] if
  <packet>
;

: parse-bits ( bits -- packet )
  0 <bit-stream> parse-packet
;

:: prefix-zeros ( bits size -- bits )
  size bits length - :> n
  { f } n repeat bits cord-append
;

: hex>bits ( hex -- bits )
  [ hex> make-bits <reversed> ] keep length 4 * prefix-zeros
;

: sum-versions ( packet -- n )
  [ get[ version ] ]
  [
    dup operator-pkt? 
    [ get[ data ] [ sum-versions ] map sum ] [ drop 0 ] if
  ]
  bi +
; inline recursive

INPUT: first hex>bits ;
PART1: parse-bits sum-versions ;
PART2: parse-bits eval-pkt ;
