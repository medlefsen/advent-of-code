USING: aoc.shared kernel splitting sequences locals accessors
math.ranges sets assocs slots.syntax hash-sets math arrays
combinators.short-circuit.smart combinators sequences.extras 
prettyprint sorting.insertion multiline assocs.extras io
hashtables math.order fry ;
IN: aoc.day23

SYMBOLS: amber bronze copper desert hall ;
CONSTANT: rooms { amber bronze copper desert }
CONSTANT: amphipod-chars {
    { CHAR: A amber }
    { CHAR: B bronze }
    { CHAR: C copper }
    { CHAR: D desert }
}

CONSTANT: costs {
  { amber 1 }
  { bronze 10 }
  { copper 100 }
  { desert 1000 }
}

CONSTANT: hall-length 11
CONSTANT: exits {
  { amber 2 }
  { bronze 4 }
  { copper 6 }
  { desert 8 }
}
CONSTANT: entryways {
  { 2 amber }
  { 4 bronze }
  { 6 copper }
  { 8 desert }
}
STRING: board
#############
#...........#
###.#.#.#.###
  #.#.#.#.#
  #########
;
TUPLE: amphipod type room pos ;
: <amphipod> ( type room pos -- amp ) amphipod boa ;

TUPLE: universe amphipods ;
: <universe> ( amp -- uni )
  dup [ hashcode ] insertion-sort universe boa ;


TUPLE: action rest amp cost ;
C: <action> action

: merge-action ( action set -- )
  [ get[ cost rest amp ] suffix <universe> ] dip
  [ [ min ] when* ] change-at
;

: hall? ( action -- ? ) amp>> room>> hall = ;
: room? ( action -- ? ) hall? not ;

:: enter-room ( amp -- )
  amp pos>> entryways at [
    amp [ room<< ] [ 2 swap pos<< ] bi
  ] when*
;

:: exit-room ( amp -- )
 amp pos>> 2 = [
   amp [ dup room>> exits at swap pos<< ]
   [ hall swap room<< ] bi 
 ] when
;

: change-rooms ( action -- )
  {
    { [ dup hall? ] [ amp>> enter-room ] }
    [ amp>> exit-room ]
  } cond
;

: dist ( pos amp -- num amp )
  [ pos>> - abs ] keep
;

:: update-cost ( pos action -- )
  action amp>> pos>> pos - abs :> dist
  action amp>> type>> costs at dist * :> cost
  action [ cost + ] change-cost drop
;

:: update-pos ( pos action -- )
  action amp>> clone pos set[ pos ] action amp<<
;

: move ( pos action -- action )
  clone [
    [ update-cost ] [ update-pos ] 2bi
  ] keep
  dup change-rooms
;

: in-room ( action -- seq )
  get[ rest amp ] room>> '[ room>> _ = ] filter
;

:: collides? ( action -- ? )
  action rest>> [ action amp>> [ get{ room pos } ] bi@ = ] any?
;

: entryway? ( amp -- ? )
  { [ room>> hall = ] [ pos>> entryways at ] } &&
;

: exit? ( amp -- ? )
  { [ room>> hall = not ] [ pos>> exits at ] } &&
;

: valid-action? ( action -- ? )
  {
    [ collides? ]
    [ amp>> entryway? ]
    [ amp>> exit? ]
  } || not
;

: valid-moves ( moves action -- actions )
  '[ _ move ] map [ collides? not ] take-while
; 

: amp-in-dest? ( action -- ? ) get[ room type ] = ;
: in-dest? ( action -- ? ) amp>> amp-in-dest? ;
: can-enter? ( action -- ? )
  { 
    [ in-dest? ]
    [ in-room [ { [ pos>> 0 = ] [ get[ room type ] = ] } && ]
      all? ]
  } &&
;

: into-room-moves ( action -- actions )
  dup can-enter? [ { 1 0 } swap valid-moves 1 tail* ] [ drop { } ] if
;

:: (hall-moves) ( action -- actions )
  action amp>> pos>> [ 0 (a,b] ] [ hall-length (a,b) ] bi
  [ action valid-moves ] bi@ append
;

: into-hall-moves ( action -- actions )
  (hall-moves) [ dup room? [ into-room-moves ] [ 1array ] if ] map-concat
;

: room-blocked? ( action -- ? )
  in-room ?first [ pos>> 1 = ] [ f ] if*
;

: should-exit? ( action -- ? )
  { [ in-dest? ] [ in-room [ amp-in-dest? ] all? ] } && not
;

: can-exit? ( action -- ? )
  { [ should-exit? ] [ room-blocked? not ] } &&
;

:: from-room-moves ( action -- actions )
  action can-exit? [ 2 action move into-hall-moves ] [ { } ] if
;

: from-hall-moves ( action -- actions )
  (hall-moves) [ room? ] filter [ into-room-moves ] map-concat
;

: find-actions ( action -- actions )
  dup hall? [ from-hall-moves ] [ from-room-moves ] if
;

:: act ( universe i -- uni action )
  universe
  universe amphipods>> :> amps
  i amps nth :> amp
  i amps remove-nth amp 0 <action>
;

:: (step-universe) ( universe cost -- actions )
  V{ } clone :> actions
  universe amphipods>> [| amp i |
    i universe amphipods>> remove-nth amp cost <action>
    find-actions :> new-actions
    actions new-actions append! drop
  ] each-index
  actions
;

:: step-universe ( universe universes -- )
  universe universes delete-at* drop :> cost
  universe cost (step-universe)
 [ universes merge-action ] each
;

: universe-done? ( universe -- ? )
  amphipods>> [ get[ room type ] = ] all?
;

: universe-score ( universe -- num )
  amphipods>> [ get[ moves type ] costs at * ] map-sum
;

:: step-simulation ( universes -- universes ? )
  universes
  universes keys [ universe-done? not ] filter
  [ f ] [ [ universes step-universe ] each t ] if-empty
;

: run-simulation ( simulation -- universes )
  [ step-simulation ] loop
;

: lowest-score ( universes -- score )
  [ universe-score ] map infimum
;

:: parse-line ( line pos -- amps )
  line [ amphipod-chars at ] { } map-as sift
  <enumerated> [| i type |
    i rooms nth :> room
    type room pos <amphipod>
  ] { } assoc>map
;

:: universe>str ( universe -- str )
 board clone :> str
 amphipod-chars assoc-invert :> types

 universe amphipods>> [
   get[ type pos room ] :> ( type pos room )
   room hall = [ pos ] [ room exits at ] if 1 + :> col
   room hall = [ 1 ] [ 3 pos - ] if :> row
   type types at row 14 * col + str set-nth
 ] each
 str
;  

: print-universe ( universe -- )
  universe>str print
;

: print-sim ( universes -- universes )
  dup keys [ print-universe nl ] each
;

INPUT: [ 2 swap nth 1 parse-line ]
       [ 3 swap nth 0 parse-line ]
       bi append <universe> 0 2array 1array >hashtable ;
PART1: run-simulation values first; 
