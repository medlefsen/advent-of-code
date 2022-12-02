USING: aoc.shared kernel sequences splitting locals accessors
math.parser math.intervals slots.syntax arrays math
sequences.extras vectors ;
IN: aoc.day22

TUPLE: step on? cuboid ;
C: <step> step

TUPLE: cuboid xr yr zr ;
C: <cuboid> cuboid

GENERIC: cuboid-size ( cuboid -- size )
GENERIC#: cuboid-intersect 1 ( cuboid other -- cuboid )
CONSTANT: empty-cuboid T{
  cuboid f empty-interval empty-interval empty-interval
}

:: cuboid-in? ( cuboid interval -- ? )
  cuboid get{ xr yr zr } [ interval interval-subset? ] all?
;

M: cuboid cuboid-size ( cuboid -- size )
  get{ xr yr zr } [ interval-length ] map product
;

M: cuboid cuboid-intersect ( cuboid other -- cuboid )
  [ [ xr>> ] bi@ interval-intersect ] 
  [ [ yr>> ] bi@ interval-intersect ] 
  [ [ zr>> ] bi@ interval-intersect ] 
  2tri
  <cuboid>
;

TUPLE: cuboid-without cuboid without ;
C: <cuboid-without> cuboid-without

:: (cuboid-without) ( cuboid without -- cuboid )
  cuboid cuboid-size :> csize
   csize 0 = [ empty-cuboid ] [ 
     without cuboid-size :> without-size
     without-size csize >= [ empty-cuboid ] [
       without-size 0 = [ cuboid ] [
         cuboid without <cuboid-without>
       ] if
     ] if
   ] if
;

M: cuboid-without cuboid-size ( cuboid -- size )
  get[ cuboid without ] [ cuboid-size ] bi@ -
;

M:: cuboid-without cuboid-intersect ( cuboid other -- cuboid )
  cuboid get[ cuboid without ] [ other cuboid-intersect ] bi@
  (cuboid-without)
;

:: cuboid-without ( a b -- cuboid )
  a a b cuboid-intersect (cuboid-without)
;

: parse-cuboid ( str -- cuboid )
  "," split
  [ "=" split1 nip
    ".." split1 [ string>number ] bi@ 1 + [a,b) ]
  map first3 <cuboid>
;

: parse-step ( str -- step )
  " " split1 [ "on"  = ] [ parse-cuboid ] bi* <step>
;

:: step-without ( step cuboid -- step )
  step get[ on? cuboid ] cuboid cuboid-without <step>
;

:: add-step ( on step -- on )
  on [ step cuboid>> step-without ] map
  step on?>> [ step suffix ] when
;

: on-size ( seq -- size )
  [ get[ cuboid on? ] [ cuboid-size ] [ drop 0 ] if ] map-sum
;

: run-steps ( steps -- num-on )
 V{ } clone [ add-step ] reduce on-size
;

: in-50? ( step -- ? )
  cuboid>> -50 51 [a,b) cuboid-in?
;

INPUT: [ parse-step ] map ;
PART1: [ in-50? ] filter run-steps ;
PART2: run-steps ;
