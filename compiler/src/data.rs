
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
pub enum TypeInfo {
    Simple(String),
    Abstract { name : String, sig_instance_name : String }, 
    Struct(String, Vec<TypeInfo>),
    Union(String, Vec<TypeInfo>),
    Fun { inputs : Vec<TypeInfo>, output : Option<Box<TypeInfo>> },
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Var {
    name : String,
    type_info : Option<TypeInfo>,
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
pub enum Expr {
    Try(Box<Expr>),    
    List(Vec<Expr>),
    Dot(Vec<Expr>),
    Var(String),
    Bool(bool),
    Str(String),
    Number(f64),
    ShortLambda(Box<Expr>),
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Statement {
    While { condition : Expr, exprs : Vec<Expr> },
    Set { var : Expr, expr : Expr },  // dot or var
    Let { var : String, type_info : Option<TypeInfo>, expr : Expr },
    Use(Vec<String>),
    Return(Option<Expr>),
    Continue,
    Break,
    Fun { name : String, type_parameters : Vec<String>, parameters : Vec<Var>, return_type : Option<TypeInfo> },
}
