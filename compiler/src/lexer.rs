
use super::data::Token;
use super::input::Input;

#[derive(Debug)]
#[derive(PartialEq)]
enum Mode {
    Normal,
    ProtoComment,
    LineComment,
    BlockComment,
    MaybeEndBlockComment,
    Str, 
    Number,
    Symbol,
}

fn lex_normal( c : char, toks : &mut Vec<Token>, buffer : &mut Vec<char> ) -> Mode {
    match c {
        t if t.is_whitespace() => Mode::Normal,
        t if t.is_digit(10) => { buffer.push( t ); Mode::Number },
        t if t.is_alphabetic() => { buffer.push( t );  Mode::Symbol },
        t if t == '_' => { buffer.push( t );  Mode::Symbol },
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
        _ => panic!( "blarg" ), // TODO better error message
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
        _ => Mode::LineComment,
    }
}

fn lex_block_comment(c : char) -> Mode {
    match c {
        '*' => Mode::MaybeEndBlockComment,
        _ => Mode::BlockComment,
    }
}

fn lex_maybe_end_block_comment(c : char) -> Mode {
    match c {
        '/' => Mode::Normal,
        _ => Mode::BlockComment,
    }
}

fn lex_str( c : char, toks : &mut Vec<Token>, buffer : &mut Vec<char> ) -> Mode {
    match c {
        '"' => { 
            toks.push(Token::Str(buffer.iter().collect())); 
            buffer.clear();
            Mode::Normal 
        },
        _ => { buffer.push(c); Mode::Str },
    }
}

fn lex_number( c : char,  
               i : usize, 
               input : &mut Input, 
               toks : &mut Vec<Token>, 
               buffer : &mut Vec<char> ) -> Mode {

    match c {
        t if t.is_digit(10) => { buffer.push(c); Mode::Number },
        '.' => { buffer.push(c); Mode::Number },
        _ => {  
            toks.push(Token::Number(buffer.iter().collect())); 
            buffer.clear();
            input.push( i, c );
            Mode::Normal
        },
    }
}

fn map_symbol( s : String ) -> Token {
    match s.as_ref() {
        "if" => Token::If,
        "elseif" => Token::Elseif,
        "else" => Token::Else,
        "true" => Token::True,
        "false" => Token::False,
        "case" => Token::Case,
        "break" => Token::Break,
        "try" => Token::Try,
        "fun" => Token::Fun,
        "abstract" => Token::Abstract,
        "union" => Token::Union,
        "struct" => Token::Struct,
        "continue" => Token::Continue,
        "return" => Token::Return,
        "test" => Token::Test,
        "let" => Token::Let,
        "set" => Token::Set,
        "for" => Token::For,
        "in" => Token::In,
        "use" => Token::Use,
        "mod" => Token::Mod,
        "impl" => Token::Impl,
        "sig" => Token::Sig,
        "while" => Token::While,
        "unit" => Token::Unit,
        _ => Token::Symbol( s ),
    }
}

fn lex_symbol( c : char,  
               i : usize, 
               input : &mut Input, 
               toks : &mut Vec<Token>, 
               buffer : &mut Vec<char> ) -> Mode {

    match c {
        t if t.is_digit(10) => { buffer.push(c); Mode::Symbol },
        t if t.is_alphabetic() => { buffer.push(c); Mode::Symbol },
        t if t == '_' => { buffer.push(c); Mode::Symbol },
        _ => {  
            toks.push(map_symbol(buffer.iter().collect())); 
            buffer.clear();
            input.push( i, c );
            Mode::Normal
        },
    }
}

pub fn lex(input : &str) -> Vec<Token> {
    let ci = input.char_indices();
    let mut input = Input{ main: ci, pushed: vec! [] }; 
    let mut toks : Vec<Token> = vec! [];
    let mut buffer : Vec<char> = vec! [];
    let mut mode = Mode::Normal;

    loop {
        match input.next() {
            Some((i, c)) => {
                match mode {
                   Mode::Normal => mode = lex_normal(c, &mut toks, &mut buffer),
                   Mode::ProtoComment => mode = lex_proto_comment(c),
                   Mode::LineComment => mode = lex_line_comment(c), 
                   Mode::BlockComment => mode = lex_block_comment(c), 
                   Mode::MaybeEndBlockComment => mode = lex_maybe_end_block_comment(c),
                   Mode::Str => mode = lex_str(c, &mut toks, &mut buffer), 
                   Mode::Number => mode = lex_number(c, i, &mut input, &mut toks, &mut buffer), 
                   Mode::Symbol => mode = lex_symbol(c, i, &mut input, &mut toks, &mut buffer), 
                }
            },
            None => {
                if mode == Mode::BlockComment 
                    || mode == Mode::ProtoComment
                    || mode == Mode::MaybeEndBlockComment
                    || mode == Mode::Str {
                
                    panic!( "encountered EOF while in {:?} mode", mode );
                }

                if buffer.len() > 0 {
                    match mode {
                       Mode::Number => { lex_number('\0', 0, &mut input, &mut toks, &mut buffer); },
                       Mode::Symbol => { lex_symbol('\0', 0, &mut input, &mut toks, &mut buffer); },
                       _ => panic!( "encountered EOF while in {:?} mode with non-zero buffer", mode ),
                    }
                }
                break 
            }
        }
    }
    toks
}

#[cfg(test)]
mod tests {
    use super::*;

        // assert_eq!( blah, blah );
        // r#" "#
    #[test]
    fn should_handle_keywords() {
        let words = vec! [ ("if", Token::If) 
                         , ("elseif", Token::Elseif)
                         , ("else", Token::Else)
                         , ("true", Token::True)
                         , ("false", Token::False)
                         , ("case", Token::Case)
                         , ("break", Token::Break)
                         , ("continue", Token::Continue)
                         , ("try", Token::Try)
                         , ("fun", Token::Fun)
                         , ("abstract", Token::Abstract)
                         , ("union", Token::Union)
                         , ("struct", Token::Struct)
                         , ("return", Token::Return)
                         , ("test", Token::Test)
                         , ("let", Token::Let)
                         , ("set", Token::Set)
                         , ("for", Token::For)
                         , ("in", Token::In)
                         , ("use", Token::Use)
                         , ("mod", Token::Mod)
                         , ("impl", Token::Impl)
                         , ("sig", Token::Sig)
                         , ("while", Token::While)
                         , ("unit", Token::Unit)
                         ];

        for (s, r) in words {
            let o = lex( s );
            assert_eq!( 1, o.len() );
            assert_eq!( r, o[0] );
        }
    }

    #[test]
    fn should_handle_punctuation() {
        let punc = vec! [ (",", Token::Comma) 
                        , (":", Token::Colon)
                        , (";", Token::Semi)
                        , ("<", Token::LAngle)
                        , (">", Token::RAngle)
                        , ("[", Token::LSquare)
                        , ("]", Token::RSquare)
                        , ("{", Token::LCurl)
                        , ("}", Token::RCurl)
                        , ("(", Token::LParen)
                        , (")", Token::RParen)
                        , ("-", Token::Sub)
                        , (".", Token::Dot)
                        , ("=", Token::Equal)
                        , ("%", Token::Percent)
                        ];

        for (s, r) in punc {
            let o = lex( s );
            assert_eq!( 1, o.len() );
            assert_eq!( r, o[0] );
        }
    }

    #[test]
    fn should_handle_symbol() {
        let symbols = vec! [ ("symbol", Token::Symbol("symbol".to_owned())) 
                           , ("symbol123", Token::Symbol("symbol123".to_owned()))
                           , ("s_123", Token::Symbol("s_123".to_owned()))
                           , ("_123", Token::Symbol("_123".to_owned()))
                           , ("_123_", Token::Symbol("_123_".to_owned()))
                           , ("_", Token::Symbol("_".to_owned()))
                           , ("i", Token::Symbol("i".to_owned()))
                           ];

        for (s, r) in symbols {
            let o = lex( s );
            assert_eq!( 1, o.len() );
            assert_eq!( r, o[0] );
        }
    }

    #[test]
    fn should_handle_symbol_like_keyword() {
        let symbols = vec! [ ("if_", Token::Symbol("if_".to_owned())) 
                           , ("_if", Token::Symbol("_if".to_owned()))
                           ];

        for (s, r) in symbols {
            let o = lex( s );
            assert_eq!( 1, o.len() );
            assert_eq!( r, o[0] );
        }
    }

    #[test]
    fn should_handle_numbers() {
        let numbers = vec! [ ("1.0", Token::Number("1.0".to_owned())) 
                           , ("1.1", Token::Number("1.1".to_owned()))
                           , ("1", Token::Number("1".to_owned()))
                           , ("1234567890", Token::Number("1234567890".to_owned()))
                           ];

        for (s, r) in numbers {
            let o = lex( s );
            assert_eq!( 1, o.len() );
            assert_eq!( r, o[0] );
        }
    }

    #[test]
    fn should_handle_strings() {
        let strings = vec! [ ("\"blah 425346#$%@#$%@.\"", Token::Str("blah 425346#$%@#$%@.".to_owned())) 
                           ];

        for (s, r) in strings {
            let o = lex( s );
            assert_eq!( 1, o.len() );
            assert_eq!( r, o[0] );
        }
    }
    // example code
    // comment
    // block comment
    // nested block comment

}

