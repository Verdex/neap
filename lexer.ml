
open List
open Data
open Str

exception LexError of int * char * string

let is_number c = 
    match c with
    | '0' .. '9' -> true
    | _ -> false

let is_number_or_dot c =
    match c with
    | '0' .. '9' -> true
    | '.' -> true
    | _ -> false

(* TODO add hex support *)
(* TODO add bin support *)
let lex_number input = 
    let start = input#index in
    let ret = Buffer.create 16 in
    Buffer.add_char ret input#current
    ; while input#move_next && is_number_or_dot input#current do
        Buffer.add_char ret input#current
    done
    ; Number (Buffer.contents ret, start)

let is_whitespace c = 
    match c with
    | '\t' | '\n' | '\r' | ' ' -> true
    | _ -> false

let is_symbol_start c = 
    match c with
    | 'a' .. 'z' | 'A' .. 'Z' | '_' -> true
    | _ -> false

let is_symbol c = 
    match c with
    | '0' .. '9' -> true
    | v when is_symbol_start v -> true
    | _ -> false

let lex_symbol input = 
    let start = input#index in
    let ret = Buffer.create 16 in
    Buffer.add_char ret input#current
    ; while input#move_next && is_symbol input#current do
        Buffer.add_char ret input#current
    done
    ; Symbol (Buffer.contents ret, start)

let is_op c = 
    match c with 
    | '+' | '*' | '-' | '/' | '^'  
    | '|' | '&' | '>' | '<' | '=' 
    | '.' | '#' | '@' | '!' | '%' 
    | '$' | '?' | '~' | '`' | ':' -> true
    | _ -> false

let lex_op input = 
    let start = input#index in
    let ret = Buffer.create 4 in
    Buffer.add_char ret input#current
    ; while input#move_next && is_op input#current do
        Buffer.add_char ret input#current
    done
    ; Op (Buffer.contents ret, start)

let is_line_comment input = input#current = '/' && input#look_ahead 1 = Some '/'

let lex_line_comment input = 
    let not_endline c = 
        match c with
        | '\n' | '\r' -> false
        | _ -> true
    in

    while input#move_next && not_endline input#current do
        ()
    done

let is_block_comment input = input#current = '/' && input#look_ahead 1 = Some '*'
(* TODO nested comments *)
let lex_block_comment input = 
    let not_end_block input = not( input#current = '*' && input#look_ahead 1 = Some '/' )
    in

    let eat _ = ()
    in

    while input#move_next && not_end_block input do
        ()
    done
    ; if input#current <> '*' && input#look_ahead 1 = Some '/' then
        raise (LexError (input#index, input#current, "block comment lexer"))
    ; eat input#move_next

(* TODO string interp *)
let lex_string input =  
    let start = input#index in
    let ret = Buffer.create 16 in
    while input#move_next && input#current <> '"' do
        Buffer.add_char ret input#current
    done
    ; if input#current <> '"' then
        raise (LexError (input#index, input#current, "string lexer"))
    ; String (Buffer.contents ret, start)


let lex (input : <current : char
                 ;index : int
                 ;move_next : bool
                 ;look_ahead : int -> char option
                 >) =
    let ret = ref [] in 
    while input#move_next do
        match input#current with
        | c when is_whitespace c -> ()
        | _ when is_line_comment input -> lex_line_comment input
        | _ when is_block_comment input -> lex_block_comment input 
        | '(' -> ret := LParen input#index :: !ret 
        | ')' -> ret := RParen input#index :: !ret 
        | '{' -> ret := LCurl input#index :: !ret 
        | '}' -> ret := RCurl input#index :: !ret 
        | '[' -> ret := LSquare input#index :: !ret
        | ']' -> ret := RSquare input#index :: !ret
        | ',' -> ret := Comma input#index :: !ret 
        | ';' -> ret := Semi input#index :: !ret 
        | c when is_symbol_start c -> ret := lex_symbol input :: !ret 
        | c when is_number c -> ret := lex_number input :: !ret
        | c when is_op c -> ret := lex_op input :: !ret 
        | '"' -> ret := lex_string input :: !ret 
        | _ -> ()
    done
    ; rev !ret


