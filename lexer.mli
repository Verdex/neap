
open Data

exception LexError of int * char * string

val lex : <current : char
          ;index : int
          ;move_next : bool
          ;look_ahead : int -> char option> -> token list

