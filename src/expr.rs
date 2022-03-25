use crate::result::Result;
use crate::error::LispError;

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
                    return Err(Box::new(LispError::Reason(
                        "Expected a function application".into(),
                    )));
                };

                match f {
                    Self::Symbol(name) => {
                        // Just the add function for now
                        if name == "+" {
                            // Create a variable to accumulate the result
                            let mut result: f64 = 0.0;
                            // Iterate over each arg and do something with it
                            for arg in args {
                                match arg {
                                    // We have just a number so we can add it to the accumulator
                                    // directly
                                    Self::Number(n) => {
                                        result += n;
                                    }
                                    // We probably have a function
                                    Self::List(_) => {
                                        match arg.eval()? {
                                            // If it evaluates to a number we just add it to the
                                            // accumulator
                                            Self::Number(n) => {
                                                result += n;
                                            }
                                            // The result of evaluating the function was wrongly
                                            // typed
                                            _ => {
                                                return Err(Box::new(LispError::Reason(
                                                    "Wrong type of argument".into(),
                                                )))
                                            }
                                        }
                                    }
                                    // We didn't get a number so we just error
                                    _ => {
                                        return Err(Box::new(LispError::Reason(
                                            "Expected a number".into(),
                                        )))
                                    }
                                }
                            }
                            // Return the result
                            Ok(Self::Number(result))
                        } else if name == "-" {
                            if args.len() > 2 {
                                Err(Box::new(LispError::Reason(
                                    "Subtraction doesn't take more than 2 arguments currently"
                                        .into(),
                                )))
                            // We need two numbers and we know we have only two arguments
                            // so we can just force unwrap them from the option
                            } else if args.len() == 2 {
                                let a = args.first().unwrap();
                                let b = args.iter().skip(1).next().unwrap();

                                // Call eval on both so that we have their applied form if they are
                                // function calls
                                match (a.eval()?, b.eval()?) {
                                    (Self::Number(a), Self::Number(b)) => Ok(Self::Number(a - b)),
                                    _ => Err(Box::new(LispError::Reason(
                                        "Both arguments must be numbers".into(),
                                    ))),
                                }
                            } else {
                                Err(Box::new(LispError::Reason("Expected two arguments".into())))
                            }
                        } else {
                            // There is no function with that name
                            Err(Box::new(LispError::Reason("Unknown function".into())))
                        }
                    }
                    // We didn't get a function to apply
                    _ => Err(Box::new(LispError::Reason(
                        "Expected a function name".into(),
                    ))),
                }
            }
        }
    }
}
