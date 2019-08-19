
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    Mod,
    Use,
    Sig,
    Impl,
    Sub,
    Equal,
    While,
    Set,
    For,
    In,
    If,
    Elseif,
    Else,
    Break,
    Continue,
    Return,
    Test,
    Let,
    Struct,
    Union,
    Abstract,
    Fun,
    Try,
    Case,
    Dot,
    Colon,
    Semi,
    Comma,
    LAngle,
    RAngle,
    LSquare,
    RSquare,
    LCurl,
    RCurl,
    LParen,
    RParen,
    Percent,
    True,
    False,
    Unit,
    Number(String),
    Str(String),
    Symbol(String),
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Ast {
    Delete,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Use {
    names : Vec<String> 
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TypeInfo {
    Simple(String),
    Complex(String, Vec<TypeInfo>),
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Var {
    name : String,
    type_info : Option<TypeInfo>
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Struct {
    type_params : Vec<String>,
    fields : Vec<Var>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Union {
    type_params : Vec<String>,
    fields : Vec<Var>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Let {
    var : Var,
    expr : Expr,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Set {
    var : String,
    expr : Expr,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Try {
    expr : Expr,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Expr {

}
