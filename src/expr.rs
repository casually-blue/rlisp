use crate::error::LispError;
use crate::result::Result;

use std::collections::HashMap;
use core::fmt::Debug;

#[derive(Clone)]
pub enum LispExpr {
    Symbol(String),
    Number(f64),
    List(Vec<LispExpr>),
    Lambda(Vec<String>, Box<LispExpr>),
    Builtin(fn(&[LispExpr], LispEnv) -> Result<LispExpr>),
}

impl Debug for LispExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Builtin(_) => write!(f, "Builtin function"),
            Self::Number(n) => write!(f, "Number({})", n),
            Self::List(l) => write!(f, "List({:?})", l),
            Self::Symbol(s) => write!(f, "Symbol({})", s),
            Self::Lambda(a,b) => write!(f, "Lambda({:?}, {:?})", a, b),
        }
    }

}

#[derive(Debug, Clone)]
pub struct LispEnv {
    pub this: HashMap<String, LispExpr>,
    pub parent: Option<Box<LispEnv>>
}

impl LispEnv {
    pub fn lookup(&self, symbol: String) -> Result<LispExpr> {
        if let Some(expr) = self.this.get(&symbol) {
            Ok(expr.clone())
        } else if let Some(parent) = &self.parent {
            parent.lookup(symbol)
        } else {
            Err(LispError::reason(format!("Symbol not found {}", symbol)))
        }
    }

    pub fn build_default() -> LispEnv {
        let mut env = LispEnv {
            this: HashMap::new(),
            parent: None
        };

        env.this.insert("+".into(), LispExpr::Builtin(add));
        env.this.insert("-".into(), LispExpr::Builtin(sub));
        env.this.insert("lambda".into(), LispExpr::Builtin(lambda));

        env
    }
}

pub fn lambda(args: &[LispExpr], _env: LispEnv) -> Result<LispExpr> {
    if args.len() != 2 {
        return Err(LispError::reason(
            "Lambda takes two arguments, a list containing the parameters and a function body",
        ));
    }

    let mut params_str = vec![];
    if let LispExpr::List(params) = &args[0] {
        for p in params {
            if let LispExpr::Symbol(s) = p {
                params_str.push(s.clone());
            } else {
                return Err(LispError::reason(
                    "funtion parameters must be symbols to be bound in the body of the function",
                ));
            }
        }
    }

    Ok(LispExpr::Lambda(params_str, Box::new(args[1].clone())))
}

pub fn add(args: &[LispExpr], env: LispEnv) -> Result<LispExpr> {
    match args.len() {
        2 => {
            // We need two numbers and we know we have only two arguments
            // so we can just force unwrap them from the option
            let (x, xs) = args.split_first().unwrap();
            let (y, _) = xs.split_first().unwrap();

            // Call eval on both so that we have their applied form if they are
            // function calls
            match (x.eval(env.clone())?, y.eval(env)?) {
                (LispExpr::Number(a), LispExpr::Number(b)) => Ok(LispExpr::Number(a + b)),
                _ => Err(LispError::reason("Both arguments must be numbers")),
            }
        }
        _ => Err(LispError::reason("Addition takes two arguments")),
    }
}

pub fn sub(args: &[LispExpr], env: LispEnv) -> Result<LispExpr> {
    match args.len() {
        2 => {
            // We need two numbers and we know we have only two arguments
            // so we can just force unwrap them from the option
            let (x, xs) = args.split_first().unwrap();
            let (y, _) = xs.split_first().unwrap();

            // Call eval on both so that we have their applied form if they are
            // function calls
            match (x.eval(env.clone())?, y.eval(env)?) {
                (LispExpr::Number(a), LispExpr::Number(b)) => Ok(LispExpr::Number(a - b)),
                _ => Err(LispError::reason("Both arguments must be numbers")),
            }
        }
        _ => Err(LispError::reason("Subtract takes two arguments")),
    }
}

impl LispExpr {
    pub fn eval(&self, env: LispEnv) -> Result<LispExpr> {
        match self {
            // Symbols and numbers both are in their final form for now
            //
            // TODO: In the future we need to look up symbols in the environment and replace them
            // with their value
            //
            // A List has to be actually evaluated
            Self::List(list) => Self::eval_list(list, env),
            _ => Ok(self.clone()),
        }
    }

    fn eval_list(list: &[LispExpr], env: LispEnv) -> Result<LispExpr> {
        // We need to check if we have anything in the list so we can apply the function
        // Some functions may not take any arguments but they at least have a name
        let (f, args) = if let Some((f, args)) = list.split_first() {
            (f, args)
        } else {
            return Err(LispError::reason(
                "Expected a function application got empty list",
            ));
        };

        match f {
            Self::Symbol(name) => match env.lookup(name.into())? {
                Self::Builtin(f) => f(args, env),
                _ => Err(LispError::reason("expected a builtin function")),
            },
            // We didn't get a function to apply
            // TODO: in future function could be a lambda which will not be just a function name in
            // a lookup
            _ => Err(LispError::reason(format!(
                "Expected a function name, got {:?}",
                f
            ))),
        }
    }
}
