
open Lexer
open Data

let value = lex (object
        val mutable _index = -1
        val stuff = {|blah123_ _blah
                      &*&^ ?
                      100
                      // blah blah blah
                      []
                      {}
                      ()
                      /* comment1
                      comment2
                      commeent3 commengt5
                      */
                      ,
                      "blah ikky
                      sap wow"
                      ;
                      1.0
                     *
                     //|} 
        method move_next = 
            if _index < String.length stuff - 1 then begin
                _index <- _index + 1
                ; true
                end
            else 
                false
        method current = stuff.[_index]  
        method index = _index
        method look_ahead a = 
            if _index + a  < String.length stuff then
                Some stuff.[_index + a]
            else
                None
     end
     );;

let p = function
    | Number (n,i) -> Printf.sprintf "number %s %d" n i
    | Symbol (s,i) -> Printf.sprintf "symbol %s %d" s i
    | Op (o,i) -> Printf.sprintf "op %s %d" o i
    | Comma i -> Printf.sprintf "comma %d" i
    | Semi i -> Printf.sprintf "semi %d" i
    | LCurl i -> Printf.sprintf "lcurl %d" i
    | RCurl i -> Printf.sprintf "rcurl %d" i
    | LParen i -> Printf.sprintf "lparen %d" i
    | RParen i -> Printf.sprintf "rparen %d" i
    | LSquare i -> Printf.sprintf "lsquare %d" i
    | RSquare i -> Printf.sprintf "rsqure %d" i
    | String (v,i) -> Printf.sprintf "string %s %d" v i 
    ;;

List.iter (fun x -> Printf.printf "%s\n" (p x)) value


