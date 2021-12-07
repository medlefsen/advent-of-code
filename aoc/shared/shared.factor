USING: io.files io.pathnames io.encodings.utf8 splitting
sequences parser ;
IN: aoc.shared

: read-lines ( path -- seq )
   utf8 file-contents "\n" split but-last
;

SYNTAX: INPUT-FILE
 location first "../input.txt" append-path suffix
;

SYNTAX: TEST-FILE
  location first "../test.txt" append-path suffix
;
