USING: aoc.shared splitting kernel sequences accessors
math.parser math locals memoize combinators ;
IN: aoc.day02

TUPLE: command dir distance ;
: <command> ( dir distance -- command ) command boa ;

TUPLE: position horiz depth aim ;
: <position> ( -- position ) 0 0 0 position boa ;
: mul-pos ( position -- mul ) [ horiz>> ] [ depth>> ] bi * ;

:: apply-command ( position command -- position )
  command distance>>
  position  
  command dir>>
  { 
    { "forward" [ [ + ] change-horiz ] }
    { "down" [ [ + ] change-depth ] }
    { "up" [ [ swap - ] change-depth ] }
  } case
;

:: apply-command-2 ( position command -- position )
  command distance>>
  position  
  command dir>>
  { 
    { "forward" [
        [ + ] change-horiz 
        command distance>> position aim>> * swap
        [ + ] change-depth
    ] }
    { "down" [ [ + ] change-aim ] }
    { "up" [ [ swap - ] change-aim ] }
  } case
;

INPUT: [ " " split first2 string>number <command> ] map ;
PART1: <position> [ apply-command ] reduce mul-pos ;
PART2: <position> [ apply-command-2 ] reduce mul-pos ;
