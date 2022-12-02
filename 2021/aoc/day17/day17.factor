USING: kernel aoc.shared splitting locals accessors math.parser
math slots.syntax math.order assocs sequences sequences.extras
combinators.short-circuit.smart math.ranges sequences.product
arrays ;
IN: aoc.day17

TUPLE: target x1 x2 y1 y2 ;
C: <target> target

TUPLE: probe dx dy x y maxy ;
: <probe> ( dx dy -- probe ) 0 0 0 probe boa ;

: to-zero ( n -- n )
 dup 0 <=> { { +gt+ -1 } { +lt+ 1 } { +eq+ 0 } } at + 
;

:: in-target? ( probe target -- ? )
  { [ probe x>> target get[ x1 x2 ] between? ]
    [ probe y>> target get[ y1 y2 ] between? ]
  } &&
;

:: past-target? ( probe target -- ? )
  { [ probe x>> target get[ x1 x2 ] max > ]
    [ probe y>> target get[ y1 y2 ] min < ]
  } ||
;

: probe-step ( probe -- )
 dup get[ x dx y dy ] [ + ] 2bi@ set[ x y ]
 [ to-zero ] change-dx
 [ 1 - ] change-dy
 dup get[ y maxy ] max set[ maxy ]
 drop
;

: fire-probe ( dx dy target -- probe )
 [ <probe> ] dip  
 [ 2dup { [ in-target? ] [ past-target? ] } || ]
 [ over probe-step  ]
 until
 drop
;

: hits-target? ( dx dy target -- ? )
 [ fire-probe ] keep in-target?
;

: search-range ( -- vels )
  0 1000 [a,b] -1000 1000 [a,b] 2array <product-sequence>
;

: filter-hits-target ( target vels -- probes )
 [ first2 pick fire-probe ] [ over in-target? ] map-filter nip
;

: hits-maxy ( probes -- y )
  unclip maxy>> [ maxy>> max ] reduce
;

INPUT: first
  "x=" split1 nip ", y=" split1
  [ ".." split1 [ string>number ] bi@ ] bi@ <target>
;

PART1: search-range filter-hits-target hits-maxy ;
PART2: search-range filter-hits-target length ; 
