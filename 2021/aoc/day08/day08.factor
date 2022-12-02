USING: aoc.shared kernel accessors splitting sequences
memoize assocs locals sets sorting math.order math.ranges
arrays math.parser vectors ;
IN: aoc.day08

TUPLE: line patterns digits ;
C: <line> line

TUPLE: decoder patterns map ;
:: <decoder> ( line -- decoder )
  line patterns>> >vector V{ } clone decoder boa
;

:: decode-pattern ( num pattern decoder -- pattern )
  pattern decoder patterns>> delete
  num pattern decoder map>> set-at
  pattern
;

:: decode-length ( num desired-length decoder -- pattern )
  decoder patterns>> [
    length desired-length =
  ] find :> pattern drop
  num pattern decoder decode-pattern
;

:: decode-subset ( num set decoder -- pattern )
  decoder patterns>> [| pattern |
    set pattern subset?
  ] filter :> patterns
  num patterns first decoder decode-pattern
;

:: with-decoder ( line quot: ( decoder -- ) -- map )
  line <decoder> quot [ map>> ] bi
; inline

:: decode-length-digits ( decoder -- )
  "8" 7 decoder decode-length :> s8
  "1" 2 decoder decode-length :> s1
  "4" 4 decoder decode-length :> s4
  "7" 3 decoder decode-length :> s7
;

:: decode-line ( decoder -- )
  "8" 7 decoder decode-length :> s8
  "1" 2 decoder decode-length :> s1
  "4" 4 decoder decode-length :> s4
  "7" 3 decoder decode-length :> s7
  s7 s1 without :> a
  s4 s1 without :> bd
  s4 s7 union :> abcdf
  "9" abcdf decoder decode-subset :> s9
  s9 abcdf without :> g
  abcdf g union :> abcdfg
  s8 abcdfg without :> e
  a bd e g 4array combine :> abdeg
  "6" abdeg decoder decode-subset :> s6
  s6 abdeg without :> 'f
  s1 'f without :> c
  "5" bd decoder decode-subset :> s5
  e 'f union :> ef
  "0" ef decoder decode-subset :> s0
  "2" e decoder decode-subset :> s2 
  "3" 'f decoder decode-subset :> s3 
;

: parse-line ( str -- line )
  " | " split1
  [ " " split [ [ <=> ] sort ] map ] bi@ <line>
;

:: count-digits ( line dmap -- count )  
  dmap keys :> mapped-digits
  line digits>> [ mapped-digits in? ] count
;

:: line-num ( line dmap -- num )
  line digits>> dmap substitute concat string>number
;

INPUT: [ parse-line ] map ;
PART1: [| line | 
  line [ decode-length-digits ] with-decoder :> dmap
  line dmap count-digits
] map sum ;

PART2: [| line |
  line [ decode-line ] with-decoder :> dmap
  line dmap line-num
] map sum ;
