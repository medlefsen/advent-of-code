USING: aoc.day05 aoc.shared tools.test ;
IN: aoc.day05.tests

: test-rows ( -- seq ) TEST-FILE read-lines ;
: test-lines ( -- lines ) test-rows [ parse-line ] map ;

{ 5 } [
  test-lines without-diag count-multiple-overlaps
] unit-test
{ 12 } [ test-lines count-multiple-overlaps ] unit-test
