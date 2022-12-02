USING: kernel io.files io.pathnames io.encodings.utf8 splitting
sequences parser fry words effects memoize lexer locals
namespaces vocabs.parser ;
IN: aoc.shared

:: define-input-contents ( name path -- word )
  name create-word-in :> word
  word
  path '[ _ utf8 file-contents ]
  { } { "str" } <effect>
  define-memoized
  word
;

:: define-input ( name contents-word parser -- )
  name create-word-in
  contents-word parser '[ _ execute( -- str ) string-lines @ ]
  { } { "input" } <effect>
  define-declared
;

:: define-part ( name input quot -- )
  name create-word-in
  input quot '[ _ execute( -- v ) @ ]
  { } { "result" } <effect>
  define-declared
;

SYNTAX: INPUT: [let
  location first :> path
  parse-definition :> parser
  
  "input-contents" path "../input.txt" append-path
  define-input-contents :> input-contents

  "input" input-contents parser define-input

  "test-input-contents" path "../test.txt" append-path
  define-input-contents :> test-input-contents

  "test-input" test-input-contents parser define-input
] ;

SYNTAX: PART1: [let
  parse-definition :> quot
  "input" current-vocab lookup-word :> input
  "part1" input quot define-part

  "test-input" current-vocab lookup-word :> input
  "test-part1" input quot define-part
] ;

SYNTAX: PART2: [let
  parse-definition :> quot
  "input" current-vocab lookup-word :> input
  "part2" input quot define-part

  "test-input" current-vocab lookup-word :> input
  "test-part2" input quot define-part
] ;
