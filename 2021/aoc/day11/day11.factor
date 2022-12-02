USING: kernel aoc.shared splitting accessors locals sequences
sets combinators math assocs memoize delegate slots.syntax
aoc.matrix combinators.short-circuit.smart refs fry ;
IN: aoc.day11

TUPLE: octopus energy ;
C: <octopus> octopus
: flashed? ( octopus -- ? ) get[ energy ] 10 = ;
: inc-energy ( octopus -- ) [ 1 + ] change-energy drop ;
: step-octopus ( octopus -- ? )
  {
   { [ dup flashed? ] [ drop f ] }
   [ [ inc-energy ] [ flashed? ] bi ]
  } cond 
;

TUPLE: cavern octopuses ;
C: <cavern> cavern
CONSULT: matrix cavern get[ octopuses ] ;

DEFER: inc-octopus
: flash-octopus ( iter -- )
adj [ inc-octopus ] each ;

: inc-octopus ( iter -- )
  dup get-ref step-octopus
  [ dup flash-octopus ] when drop
;

: reset-flashed ( iter -- ? )
  get-ref { [ flashed? ] [ 0 set[ energy ] ] } &&
;

: cavern-step ( cavern -- flashed )
  <matrix-seq>
  [ [ inc-octopus ] each ]
  [ [ reset-flashed ] count ]
  bi
;

: all-flashed? ( cavern -- ? )
  <matrix-seq> [ get-ref energy>> 0 = ] all?
;

: until-flashed ( cavern -- n )
  0 [ over all-flashed? ] [ over cavern-step drop  1 + ] until
  nip
;
INPUT: [ [ 48 - <octopus> ] { } map-as ] map <cavern> ;
PART1: '[ _ cavern-step ] 100 swap replicate sum ; 
PART2: until-flashed ; 
