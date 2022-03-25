use crate::error::LispError;
use crate::result::Result;

#[derive(Debug, Clone)]
pub enum LispExpr {
    Symbol(String),
    Number(f64),
    List(Vec<LispExpr>),
}

impl LispExpr {
    pub fn eval(&self) -> Result<LispExpr> {
        match self {
            // Symbols and numbers both are in their final form for now
            // TODO: In the future we need to look up symbols in the environment and replace them
            // with their value
            Self::Symbol(_) | Self::Number(_) => Ok(self.clone()),

            // A List has to be actually evaluated
            Self::List(list) => {
                // We need to check if we have anything in the list so we can apply the function
                // Some functions may not take any arguments but they at least have a name
                let (f, args) = if let Some((f, args)) = list.split_first() {
                    (f, args)
                } else {
                    return Err(LispError::reason("Expected a function application"));
                };

                match f {
                    Self::Symbol(name) => {
                        match name.as_str() {
                            // Just the add function for now
                            "+" => {
                                // Create a variable to accumulate the result
                                let mut result: f64 = 0.0;
                                // Iterate over each arg and do something with it
                                for arg in args {
                                    result += match arg {
                                        // We have just a number so we can add it to the accumulator
                                        // directly
                                        Self::Number(n) => *n,
                                        // We probably have a function
                                        Self::List(_) => {
                                            match arg.eval()? {
                                                // If it evaluates to a number we just add it to the
                                                // accumulator
                                                Self::Number(n) => n,
                                                // The result of evaluating the function was wrongly
                                                // typed
                                                _ => {
                                                    return Err(LispError::reason(
                                                        "Wrong type of argument",
                                                    ))
                                                }
                                            }
                                        }
                                        // We didn't get a number so we just error
                                        _ => return Err(LispError::reason("Expected a number")),
                                    }
                                }
                                // Return the result
                                Ok(Self::Number(result))
                            }
                            "-" => {
                                match args.len() {
                                    2 => {
                                        // We need two numbers and we know we have only two arguments
                                        // so we can just force unwrap them from the option
                                        let a = args.first().unwrap();
                                        let b = args.iter().skip(1).next().unwrap();

                                        // Call eval on both so that we have their applied form if they are
                                        // function calls
                                        match (a.eval()?, b.eval()?) {
                                            (Self::Number(a), Self::Number(b)) => {
                                                Ok(Self::Number(a - b))
                                            }
                                            _ => Err(LispError::reason(
                                                "Both arguments must be numbers",
                                            )),
                                        }
                                    }
                                    _ => {
                                        Err(LispError::reason("Subtract only takes two arguments"))
                                    }
                                }
                            }
                            _ => {
                                // There is no function with that name
                                Err(LispError::reason("Unknown function"))
                            }
                        }
                    }
                    // We didn't get a function to apply
                    _ => Err(LispError::reason("Expected a function name")),
                }
            }
        }
    }
}
