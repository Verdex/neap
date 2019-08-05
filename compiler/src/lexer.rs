
use super::data::Token;

enum Mode {
    Normal,
    LineComment,
    BlockComment,
    Str, 
    Number,
    Symbol,
}

fn lex_normal( toks : &mut Vec<Token>, buffer : &mut Vec<char> ) -> Mode {
    Mode::Normal
}


pub fn lex(input : &str) -> Vec<Token> {
    let mut ci = input.char_indices();
    let mut toks : Vec<Token> = vec! [];
    let mut buffer : Vec<char> = vec! [];
    let mut mode = Mode::Normal;

    loop {
        match ci.next() {
            Some((i, c)) => {
                match mode {
                   Mode::Normal => mode = lex_normal( &mut toks, &mut buffer),
                }
            },
            None => { break }
        }
    }
    toks
}

