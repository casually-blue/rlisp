use crate::error::LispError;
use crate::result::Result;

#[derive(Debug, Clone)]
pub enum LispExpr {
    Symbol(String),
    Number(f64),
    List(Vec<LispExpr>),
    Lambda(Vec<String>, Box<LispExpr>),
}

pub fn lambda(args: &[LispExpr]) -> Result<LispExpr> {
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

pub fn add(args: &[LispExpr]) -> Result<LispExpr> {
    // Create a variable to accumulate the result
    let mut result: f64 = 0.0;
    // Iterate over each arg and do something with it
    for arg in args {
        result += match arg.eval()? {
            // We have just a number so we can add it to the accumulator
            // directly
            LispExpr::Number(n) => n,
            // We probably have a function
            // We didn't get a number so we just error
            _ => return Err(LispError::reason("Expected a number")),
        }
    }
    // Return the result
    Ok(LispExpr::Number(result))
}

pub fn sub(args: &[LispExpr]) -> Result<LispExpr> {
    match args.len() {
        2 => {
            // We need two numbers and we know we have only two arguments
            // so we can just force unwrap them from the option
            let (x, xs) = args.split_first().unwrap();
            let (y, _) = xs.split_first().unwrap();

            // Call eval on both so that we have their applied form if they are
            // function calls
            match (x.eval()?, y.eval()?) {
                (LispExpr::Number(a), LispExpr::Number(b)) => Ok(LispExpr::Number(a - b)),
                _ => Err(LispError::reason("Both arguments must be numbers")),
            }
        }
        _ => Err(LispError::reason("Subtract only takes two arguments")),
    }
}

impl LispExpr {
    pub fn eval(&self) -> Result<LispExpr> {
        match self {
            // Symbols and numbers both are in their final form for now
            //
            // TODO: In the future we need to look up symbols in the environment and replace them
            // with their value
            //
            // A List has to be actually evaluated
            Self::List(list) => Self::eval_list(list),
            _ => Ok(self.clone()),
        }
    }

    fn lookup(symbol: &str) -> Result<Box<dyn Fn(&[LispExpr]) -> Result<LispExpr>>> {
        match symbol {
            "+" => Ok(Box::new(add)),
            "-" => Ok(Box::new(sub)),
            "lambda" => Ok(Box::new(lambda)),
            _ => Err(LispError::reason(format!("Unknown symbol {}", symbol))),
        }
    }

    fn eval_list(list: &[LispExpr]) -> Result<LispExpr> {
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
            Self::Symbol(name) => Self::lookup(name)?(args),
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
