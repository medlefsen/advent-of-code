USING: kernel sequences math.vectors locals ;
IN: aoc.linalg

: homo ( v3 -- v4 ) 1 suffix ;

:: vm* ( vec matrix -- vec )
  matrix vec homo [ v. ] curry map but-last
;

: m* ( m1 m2 -- m ) flip [ v. ] cartesian-map ;

: identity ( -- matrix )
  { { 1 0 0 0 } { 0 1 0 0 } { 0 0 1 0 } { 0 0 0 1 } }
; inline

: rot-x-90 ( -- matrix )
  { { 1 0 0 0 } { 0 0 -1 0 } { 0 1 0 0 } { 0 0 0 1 } }
; inline

: rot-y-90 ( -- matrix )
  { { 0 0 1 0 } { 0 1 0 0 } { -1 0 0 0 } { 0 0 0 1 } }
; inline

: rot-z-90 ( -- matrix )
  { { 0 -1 0 0 } { 1 0 0 0 } { 0 0 1 0 } { 0 0 0 1 } }
; inline

: translate ( vec -- matrix )
  homo identity [
    clone 3 swap [ set-nth ] keep
  ] 2map
;
