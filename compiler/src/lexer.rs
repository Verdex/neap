
use super::data::Token;

enum Mode {
    Normal,
    LineComment,
    BlockComment,
    Str, 
    Number,
    Symbol,
}

fn lex_normal( c : char, toks : &mut Vec<Token>, buffer : &mut Vec<char> ) -> Mode {
    match c {
        ';' => toks.push( Token::Semi ),
        ',' => toks.push( Token::Comma ),
        ':' => toks.push( Token::Colon ),
        '<' => toks.push( Token::LAngle ),
        '>' => toks.push( Token::RAngle ),
        '[' => toks.push( Token::LSquare ),
        ']' => toks.push( Token::RSquare ),
        '{' => toks.push( Token::LCurl ),
        '}' => toks.push( Token::RCurl ),
        '(' => toks.push( Token::LParen ),
        ')' => toks.push( Token::RParen ),
        '-' => toks.push( Token::Sub ),
        '=' => toks.push( Token::Equal ),
        '.' => toks.push( Token::Dot ),
        '%' => toks.push( Token::Percent ),
    };
    Mode::Normal
}

fn lex_line_comment( c : char, toks : &mut Vec<Token>, buffer : &mut Vec<char> ) -> Mode {
    Mode::Normal
}

fn lex_block_comment( c : char, toks : &mut Vec<Token>, buffer : &mut Vec<char> ) -> Mode {
    Mode::Normal
}

fn lex_str( c : char, toks : &mut Vec<Token>, buffer : &mut Vec<char> ) -> Mode {
    Mode::Normal
}

fn lex_number( c : char, toks : &mut Vec<Token>, buffer : &mut Vec<char> ) -> Mode {
    Mode::Normal
}

fn lex_symbol( c : char, toks : &mut Vec<Token>, buffer : &mut Vec<char> ) -> Mode {
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
                   Mode::Normal => mode = lex_normal(c, &mut toks, &mut buffer),
                   Mode::LineComment => mode = lex_line_comment(c, &mut toks, &mut buffer), 
                   Mode::BlockComment => mode = lex_block_comment(c, &mut toks, &mut buffer), 
                   Mode::Str => mode = lex_str(c, &mut toks, &mut buffer), 
                   Mode::Number => mode = lex_number(c, &mut toks, &mut buffer), 
                   Mode::Symbol => mode = lex_symbol(c, &mut toks, &mut buffer), 
                }
            },
            None => { break }
        }
    }
    toks
}

