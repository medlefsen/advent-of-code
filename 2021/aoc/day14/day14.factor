USING: kernel aoc.shared sequences splitting locals accessors
slots.syntax grouping sequences.extras arrays assocs math
math.statistics math.functions ;
IN: aoc.day14

TUPLE: instructions first template rules ;
C: <instructions> instructions

: parse-rules ( lines -- rules )
  [ " -> " split1
  dupd [ swap first prefix ] [ swap last suffix ] 2bi 2array ]
  { } map>assoc
;

: parse-template ( str -- template )
  2 clump H{ } clone [ over inc-at ] reduce
;

:: apply-rules ( rules template -- template )
  H{ } clone :> new-template
  template [| pair n |
    pair rules at [| pair-seq |
      pair-seq [ n swap new-template at+ ] each
    ] [ pair new-template inc-at ] if*
  ] assoc-each
  new-template
;

:: step-instructions ( instructions -- instructions )
  instructions get[ rules template ] apply-rules
  instructions swap set[ template ]
;

: run-instructions ( instructions n -- instructions )
  [ step-instructions ] times
;

: count-letters ( assoc first -- assoc )
  V{ } clone [ inc-at ] keep swap [| assoc pair n |
    n pair last assoc at+
    assoc
  ] assoc-each 
  values
;

: run-and-score ( instructions n -- score )
  run-instructions get[ template first ] count-letters
  minmax swap -
;

INPUT: { "" } split1 [ drop first first ]
2keep [ first parse-template ] [ parse-rules ]
bi* <instructions>
;

PART1: 10 run-and-score ;
PART2: 40 run-and-score ;
