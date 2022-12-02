USING: kernel sequences aoc.shared splitting graphs locals
arrays hashtables fry slots.syntax unicode sets accessors
assocs math ;
IN: aoc.day12 

TUPLE: visitor graph skip paths ;
: <visitor> ( graph skip -- visitor )
  0 visitor boa
;

:: paths ( path visitor -- edges )
  path last visitor get[ graph ] at members path
  visitor get[ skip ] call( s -- s ) without
;

: visit ( path visitor -- edges )
  over last "end" =
  [ [ 1 + ] change-paths 2drop { } ]
  [ paths ]
  if
;

: traverse ( path quot: ( path -- edges ) -- )
 [ call ] 2keep '[ _ swap suffix _ traverse ] each
; inline recursive

: count-paths ( graph skip -- n )
  <visitor>
  dup '[ _ visit ] { "start" } swap traverse
  get[ paths ]
; inline

:: connect ( a b graph -- )
  a b 1array graph add-vertex
  b a 1array graph add-vertex
;

INPUT: dup length <hashtable> swap [
  "-" split first2 pick connect
] each
;

: skip-lower ( path -- skip )
  [ [ 0 1 ] dip subseq lower? ] filter
;

: skip-multiple-lower ( path -- skip )
  skip-lower dup all-unique? [ drop { "start" } ] when
;

PART1: [ skip-lower ] count-paths ;
PART2: [ skip-multiple-lower ] count-paths ;
