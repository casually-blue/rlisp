#[derive(Clone)]
pub enum LispExpr {
    Symbol(String),
    Number(u64),
    List(Box<LispList>),
}

#[derive(Clone, Debug)]
pub enum LispList {
    Cons(Box<LispList>, LispExpr),
    Nil
}

impl core::fmt::Debug for LispExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Number(n) => write!(f, "Number({})", n),
            Self::List(l) => write!(f, "List({:?})", l),
            Self::Symbol(s) => write!(f, "Symbol({})", s),
        }
    }

}
