USING: kernel aoc.shared splitting math.parser regexp sequences
sequences.product assocs sets math accessors
arrays sequences.extras locals memoize literals math.distances
aoc.linalg math.vectors ;
IN: aoc.day19

TUPLE: scanner id beacons transform ;
C: <scanner> scanner

MEMO: orientations ( -- seq )
  {
    $[ rot-x-90 ] $[ rot-x-90 ] $[ rot-x-90 ]
    $[ rot-x-90 rot-z-90 m* ]
    $[ rot-x-90 ] $[ rot-x-90 ] $[ rot-x-90 ]
    $[ rot-x-90 rot-z-90 m* ]
    $[ rot-x-90 ] $[ rot-x-90 ] $[ rot-x-90 ]
    $[ rot-x-90 rot-z-90 m* ]
    $[ rot-x-90 ] $[ rot-x-90 ] $[ rot-x-90 ]
    $[ rot-x-90 rot-z-90 m* rot-y-90 m* ]
    $[ rot-x-90 ] $[ rot-x-90 ] $[ rot-x-90 ]
    $[ rot-x-90 rot-y-90 m* rot-y-90 m* ]
    $[ rot-x-90 ] $[ rot-x-90 ] $[ rot-x-90 ]
  } identity [ m* ] accumulate swap suffix
;

:: gen-transform ( from to orientation -- matrix )
    to from orientation vm* v- translate
    orientation m*
;

:: possible-transforms ( from to -- seq )
  orientations [| orientation |
    from to orientation gen-transform
  ] map
;

:: valid-transform? ( from to transform -- ? )
  from transform vm* to =
;

:: find-transform ( assoc -- transform )
  assoc [
    first2 possible-transforms [| transform |
      assoc [ first2 transform valid-transform? ] count 11 >
    ] find nip
  ] map-find drop
;

:: find-match ( beacon beacons -- pairs )
  beacons [ second beacon second intersect length 11 > ] filter
  [ first beacon first swap 2array ] map
;

:: matches ( seq beacons -- assoc )
  seq [ beacons find-match ] map-concat
;

:: scanner-find-transform ( beacons scanner -- transform )
  scanner transform>> [
    scanner beacons>> beacons matches [
      find-transform [ scanner transform<< ] when* 
    ] unless-empty
  ] unless scanner transform>>
;

: beacon-distances ( beacons -- distances )
  dup dup [ distance ] cartesian-map zip
;

:: scanner-add-beacons ( beacons scanner -- beacons )
  scanner beacons>> [ first scanner transform>> vm* ] map
  beacons [ first ] map union beacon-distances
;

: parse-scanner ( lines -- scanner )
  unclip R/ \d+/ first-match string>number swap
  [ "," split [ string>number ] map ] map beacon-distances
  f <scanner>
;

:: scanner-matches ( s1 s2 -- pair )
  s1 beacons>> s2 beacons>> matches
;

:: add-scanner ( beacons scanner -- beacons )
  beacons scanner scanner-find-transform [
    beacons scanner scanner-add-beacons
  ] [ beacons ] if
;

: add-scanners ( scanners beacons -- beacons )
  [ over [ transform>> not ] filter dup empty? ]
  [ [ add-scanner ] each ] until
  drop nip
;

:: max-manhattan-distance ( scanners -- distance )
  scanners [ transform>> flip fourth but-last ] map
  dup 2array [ first2 manhattan-distance ] 
  product-map supremum
;

: add-first-scanner ( scanners -- scanners beacons )
 dup first identity over transform<< beacons>>
;

INPUT: { "" } split [ parse-scanner ] map ;
PART1: add-first-scanner add-scanners length ;
PART2: dup add-first-scanner add-scanners drop
max-manhattan-distance ;
