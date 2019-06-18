
type meta_data = int

type token = 
    | String of string * meta_data
    | Symbol of string * meta_data
    | Op of string * meta_data
    | Number of string * meta_data
    | LCurl of meta_data
    | RCurl of meta_data
    | LParen of meta_data
    | RParen of meta_data
    | LSquare of meta_data
    | RSquare of meta_data
    | Comma of meta_data
    | Semi of meta_data

type ast = 
    | Block of 

