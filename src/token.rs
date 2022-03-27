#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Token {
    EOF,

    StrLit(String),
    Symbol(String),
    Number(u64),

    LOpen,
    LClose,
}
