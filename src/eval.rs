use crate::expr::{LispExpr, LispList};
use crate::env::LispEnv;
use crate::result::Result;

pub fn eval(expr: LispExpr, env: LispEnv) -> Result<LispExpr> {
    use LispExpr::*;

    match expr {
        Number(_) => Ok(expr),
        Symbol(s) => env.lookup(s),
        List(l) => eval_list(*l, env),
    }
}

pub fn eval_list(list: LispList, env: LispEnv) -> Result<LispExpr> {
    use LispExpr::*;
    use LispList::*;
    match &list {
        Cons(_cdr, car) => {
            match car {
                List(l) => eval_list(*l.clone(), env.clone()),
                Symbol(s) if s == "lambda" => {
                    Ok(car.clone())
                },

                Symbol(s) => env.lookup(s.to_string()),
                _ => Ok(eval(car.clone(), env.clone())?)
            }
        },
        Nil => Ok(LispExpr::List(box list))
    }
}
