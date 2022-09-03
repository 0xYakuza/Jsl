use crate::types::*;

#[derive(Debug, Clone)]
pub enum Token {
    Number(f64),
    Add,
    Minus,
    Div,
    Mul,
    Swap,
    Rot,
    Put,
    Eq,
    Macro(Macro),
    Noteq,
    Bigger,
    Smaller,
    Then,
    True,
    False,
    Dup,
    Drop,
    Str(String),
    Times(Vec<Token>),
    Import(Vec<Token>),
    Let(String),
    Set(String),
    Mempop,
    Memusage,
    Function(Function),
    Call(String),
    Ident(String),
    Array(Vec<Token>),
    PushArray,
}
