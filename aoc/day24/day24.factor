USING: kernel aoc.shared splitting math math.parser locals
accessors assocs sequences combinators sets slots.syntax 
combinators.short-circuit.smart fry math.ranges formatting
memoize sequences.extras vectors sequences.product 
hashtables arrays present math.order ;
IN: aoc.day24

CONSTANT: regs { "w" "x" "y" "z" }
SINGLETONS: op-mul op-div op-add op-mod op-eq ;

TUPLE: op type terms ;
C: <op> op

TUPLE: alu input regs ;
: <alu> ( input -- alu ) V{ } clone alu boa ;

TUPLE: sym-alu pos regs ;
: <sym-alu> ( -- alu ) 0 V{ } clone sym-alu boa ;

TUPLE: value assoc ;
C: <value> value

TUPLE: inst type args ;
C: <inst> inst

TUPLE: source { inputs array } ;
:: <source> ( pos num -- source )
  pos 1 + f <array> :> inputs
  num pos inputs set-nth
  inputs source boa
;

:: (union-inputs) ( a b d q: ( a b -- c )  -- inputs )
  a b [ length ] bi@ max <iota> [| i |
    a b [| s | i s ?nth d or ] bi@ q call
  ] map
; inline

: union-max ( a b -- source )
 [ inputs>> ] bi@ 0 [ max ] (union-inputs) source boa
;

: union-min ( a b -- source )
 [ inputs>> ] bi@ 9 [ min ] (union-inputs) source boa
;

:: <inp> ( pos -- val ) 1 9 [a,b] [| num |
    num pos num <source> 2array
  ] { } map-as <value>
;

: <lit> ( num -- value ) source new 2array 1array <value> ;

: reg ( reg alu -- v ) regs>> at ;

: load ( a alu -- x )
  over regs in? [ regs>> at 0 <lit> or ]
  [ drop string>number <lit> ] if
;

: store ( x a alu -- ) regs>> set-at ;

: inst-inp ( seq alu -- ) 
  { [ pos>> <inp> ]
    [ [ swap first ] dip store ]
    [ [ 1 + ] change-pos drop ]
  } cleave 
;

:: val-push ( num sources assoc -- )
  num assoc at [ sources union-max ] [ sources ] if*
  num assoc set-at
;

:: noop? ( value quot: ( a -- ? ) -- ? )
  {
    [ value assoc>> length 1 = ]
    [ value assoc>> first first quot call ]
  } &&
; inline

:: value-product-each ( a b quot: ( a b -- ) -- )
  a assoc>> b assoc>> 2array
  [ first2 quot call ] product-each
; inline

:: merge-values ( a b op: ( x y -- num ) -- num sources )
    a first b first op call
    a second b second union-min
; inline

:: (map-values) ( a b quot: ( a b -- c ) -- v ) 
  H{ } clone :> ht
  a b [
    quot merge-values ht val-push
  ] value-product-each
  ht >alist <value>
; inline

:: map-values ( a b quot: ( a b -- c ) noop: ( a -- ? ) -- v )
  a noop noop? b and 
  b noop noop? a and or
  [ a b quot (map-values) ] unless*
; inline

:: binary-op ( seq alu quot: ( a b -- v ) noop: ( a -- ? ) -- )
  seq [ alu load ] map first2 ! ( a b )
  quot noop map-values :> val
  val seq first alu store
; inline

: 1= ( x y -- 1/0 ) = 1 0 ? ;

: inst-add ( seq alu -- ) [ + ] [ 0 = ] binary-op ;
: inst-mul ( seq alu -- ) [ * ] [ 1 = ] binary-op ; 
: inst-div ( seq alu -- ) [ /i ] [ drop f ] binary-op ; 
: inst-mod ( seq alu -- ) [ mod ] [ drop f ] binary-op ; 
: inst-eql ( seq alu -- ) [ 1= ] [ drop f ] binary-op ; 

CONSTANT: insts {
 { "inp" [ inst-inp ] }
 { "add" [ inst-add ] }
 { "mul" [ inst-mul ] }
 { "div" [ inst-div ] }
 { "mod" [ inst-mod ] }
 { "eql" [ inst-eql ] }
}

M: inst present ( obj -- string )
  get[ type args ] "," join "%s(%s)" sprintf
;

M: value present ( obj -- string )
  assoc>> keys dup length 20 > [
    length "(%d nums)" sprintf
  ] [
   [ number>string ] map "," join "(%s)" sprintf
  ] if 
;

M: sym-alu present ( obj -- string )
  regs>> [ "%s: %s" sprintf ] { } assoc>map ", " join
;

USE: io
: print-state ( inst alu -- )
  "%s (%s)\n" printf
  flush
;

: call-inst ( inst alu -- )
  [ [ get[ args type ] insts at ] dip swap call( seq alu -- ) ]
  2keep print-state
;

:: run-insts ( insts alu -- alu )
  insts [ alu call-inst ] each alu
;

:: check-number ( prog num -- ? )
  num number>string
  { [ "0" swap in? ]
    [ prog <alu> run-insts regs>> "z" swap at 0 = ]
  } &&
;

CONSTANT: max-num 99999999999999
CONSTANT: min-num 9999999999999

INPUT: [ " " split1 " " split <inst> ] map ;
! PART1: '[ _ swap check-number ] [ max-num min-num [a,b) ] dip find nip ;
PART1: <sym-alu> run-insts "z" swap reg ;
