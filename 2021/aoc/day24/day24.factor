USING: kernel aoc.shared splitting math math.parser locals
accessors assocs sequences combinators sets slots.syntax 
combinators.short-circuit.smart fry math.ranges formatting
memoize sequences.extras ;
IN: aoc.day24

CONSTANT: regs { "w" "x" "y" "z" }
SINGLETONS: op-mul op-div op-add op-mod op-eq ;
TUPLE: op type terms ;
C: <op> op

TUPLE: inp pos ;
C: <inp> inp

TUPLE: alu input regs ;
: <alu> ( input -- alu ) V{ } clone alu boa ;

TUPLE: sym-alu regs ;
: <sym-alu> ( -- alu ) 0 V{ } clone sym-alu boa ;

TUPLE: inst type args ;
C: <inst> inst

: reg ( reg alu -- v ) regs>> at ;

: load ( a alu -- x )
  over regs in? [ regs>> at 0 or ] [ drop string>number ] if
;
: store ( x a alu -- ) regs>> set-at ;

: inst-inp ( seq alu -- ) 
  { [ pos>> <inp> ]
    [ [ swap first ] dip store ]
    [ [ 1 + ] change-pos drop ]
  } cleave 
;

: inst-inp! ( seq alu -- ) 
  { [ input>> first 48 - ]
    [ [ swap first ] dip store ]
    [ [ rest ] change-input drop ]
  } cleave 
;

:: binary-op ( seq alu op -- )
  op seq [ alu load ] map <op> seq first alu store
; inline

: inst-add ( seq alu -- ) op-add binary-op ;
: inst-mul ( seq alu -- ) op-mul binary-op ; 
: inst-div ( seq alu -- ) op-div binary-op ; 
: inst-mod ( seq alu -- ) op-mod binary-op ; 
: inst-eql ( seq alu -- ) op-eq binary-op ; 

:: binary-op! ( seq alu quot: ( x y -- z ) -- )
  seq [ alu load ] map first2 quot call seq first alu store
; inline

: any-0? ( seq -- ? ) [ 0 = ] any? ;
: any-inp? ( seq -- ? ) [ inp? ] any? ;

:: build-op ( terms type -- op )
  terms length 1 = [ terms first ] [ type terms <op> ] if
;

:: op-type? ( op type -- ? )
 op { [ op? ] [ type>> type = ] } &&
;

:: merge-children ( terms type -- terms )
  terms [ type op-type? ] partition :> ( same diff )  
  same [ terms>> ] map-concat diff append
;

:: merge-num ( terms quot: ( seq -- x ) -- terms )
  terms [ number? not ] partition quot call suffix
; inline

: reject-0 ( terms -- terms ) [ 0 = ] reject ;

: all-num? ( seq -- ? ) [ number? ] all? ;

GENERIC: eval-op ( args op -- val )
GENERIC: eval ( obj -- val )
: eq1 ( x y -- z ) = 1 0 ? ;

M: op-add eval-op ( args op -- val ) drop
  op-add merge-children
  [ sum ] merge-num
  [ 0 = ] reject
  op-add build-op
;

M: op-mul eval-op ( args op -- val ) drop
  op-mul merge-children
  [ product ] merge-num
  [ 1 = ] reject
  op-mul build-op
;

M: op-div eval-op ( args op -- val ) drop {
    { [ dup all-num? ] [ first2 /i ] }
    { [ dup first 0 = ] [ drop 0 ] }
    { [ dup second 1 = ] [ first ] }
    [ op-div build-op ]
  } cond ;

M: op-mod eval-op ( args op -- val ) drop {
    { [ dup all-num? ] [ first2 mod ] }
    { [ dup first 0 = ] [ drop 0 ] }
    { [ dup second 1 = ] [ drop 0 ] }
    [ op-mod build-op ]
  } cond ;

M: op-eq eval-op ( args op -- val ) drop {
    { [ dup first2 = ] [ first2 eq1 ] }
    { [ dup { [ any-0? ] [ any-inp? ] } && ] [ drop 0 ] }
    [ op-eq build-op ]
  } cond ;

IDENTITY-MEMO: (eval-op) ( op -- val )
  get[ type terms ] [ eval ] map swap eval-op
;

M: op eval ( op -- val ) (eval-op) ;
M: inp eval ( inp -- val ) ;
M: number eval ( inp -- val ) ;

GENERIC: to-str ( obj -- str )
M: op-add to-str ( obj -- str ) drop "+" ;
M: op-div to-str ( obj -- str ) drop "/i" ;
M: op-mul to-str ( obj -- str ) drop "*" ;
M: op-mod to-str ( obj -- str ) drop "mod" ;
M: op-eq to-str ( obj -- str ) drop "eq1" ;
M: sequence to-str ( seq -- str ) [ to-str ] map " " join  ;

IDENTITY-MEMO: (op-to-str) ( obj -- str )
get{ terms type } to-str ;

M: op to-str ( obj -- str ) (op-to-str) ;
M: inp to-str ( inp -- str ) get[ pos ] "i%d" sprintf ;
M: number to-str ( num -- str ) number>string ;

: inst-add! ( seq alu -- ) [ + ] binary-op! ;
: inst-mul! ( seq alu -- ) [ * ] binary-op! ; 
: inst-div! ( seq alu -- ) [ /i ] binary-op! ; 
: inst-mod! ( seq alu -- ) [ mod ] binary-op! ; 
: inst-eql! ( seq alu -- ) [ = 1 0 ? ] binary-op! ; 

CONSTANT: insts {
 { "inp" [ inst-inp ] }
 { "add" [ inst-add ] }
 { "mul" [ inst-mul ] }
 { "div" [ inst-div ] }
 { "mod" [ inst-mod ] }
 { "eql" [ inst-eql ] }
}

: call-inst ( inst alu -- )
  [ get[ args type ] insts at ] dip swap call( seq alu -- )
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
PART1: <sym-alu> run-insts "z" swap reg eval ;
