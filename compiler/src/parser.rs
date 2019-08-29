
use super::data::Token;
use super::data::Ast;

fn blarg(t : Token) {

}

pub fn parse( lexemes : Vec<Token> ) -> Ast {
    let mut input = lexemes.into_iter();

    loop {
        match input.next() {
            Some(lexeme) => blarg(lexeme),
            None => break,
        }
    }
    Ast::Delete 
}

