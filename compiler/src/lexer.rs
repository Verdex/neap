
use super::data::Token;

enum Mode {
    Normal,
    ProtoComment,
    LineComment,
    BlockComment,
    Str, 
    Number,
    Symbol,
}

fn lex_normal( c : char, toks : &mut Vec<Token>, buffer : &mut Vec<char> ) -> Mode {
    match c {
        t if t.is_whitespace() => Mode::Normal,
        t if t.is_digit(10) => { buffer.push( t ); Mode::Number },
        t if t.is_alphabetic() => { buffer.push( t );  Mode::Symbol },
        t if t == '"' => Mode::Str,
        ';' => { toks.push( Token::Semi ); Mode::Normal },
        ',' => { toks.push( Token::Comma ); Mode::Normal },
        ':' => { toks.push( Token::Colon ); Mode::Normal },
        '<' => { toks.push( Token::LAngle ); Mode::Normal },
        '>' => { toks.push( Token::RAngle ); Mode::Normal },
        '[' => { toks.push( Token::LSquare ); Mode::Normal },
        ']' => { toks.push( Token::RSquare ); Mode::Normal },
        '{' => { toks.push( Token::LCurl ); Mode::Normal },
        '}' => { toks.push( Token::RCurl ); Mode::Normal },
        '(' => { toks.push( Token::LParen ); Mode::Normal },
        ')' => { toks.push( Token::RParen ); Mode::Normal },
        '-' => { toks.push( Token::Sub ); Mode::Normal },
        '=' => { toks.push( Token::Equal ); Mode::Normal },
        '.' => { toks.push( Token::Dot ); Mode::Normal },
        '%' => { toks.push( Token::Percent ); Mode::Normal },
        '/' => Mode::ProtoComment,
        _ => panic!( "blarg" ),
    }
}

fn lex_proto_comment( c : char ) -> Mode {
    match c {
        '/' => Mode::LineComment,
        '*' => Mode::BlockComment,
        _ => panic!( "expected comment token, but encountered {}", c ),
    }
}

fn lex_line_comment(c : char) -> Mode {
    match c {
        '\n' => Mode::Normal,
        '\r' => Mode::Normal,
        // TODO EOF
        _ => Mode::LineComment,
    }
}

fn lex_block_comment(c : char) -> Mode {
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
                   Mode::ProtoComment => mode = lex_proto_comment(c),
                   Mode::LineComment => mode = lex_line_comment(c), 
                   Mode::BlockComment => mode = lex_block_comment(c), 
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

