extern crate alloc;

mod prompt;
use prompt::ReplPrompt;
use reedline::{Reedline, Signal};
use xdg::*;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone)]
enum LispExpr {
    Symbol(String),
    Number(f64),
    List(Vec<LispExpr>),
}

impl LispExpr {
    fn eval(&self) -> Result<LispExpr> {
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

#[derive(Debug)]
enum LispError {
    Reason(String),
}

impl std::fmt::Display for LispError {
    // Just output the debug result
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for LispError {}

fn parse<'a>(code: &'a [&'a str]) -> Result<(LispExpr, &'a [&'a str])> {
    // If we don't have anything we can just return a blank input
    let (first, mut rest) = match code.split_first() {
        Some((f, r)) => (f, r),
        _ => return Ok((LispExpr::List(vec![]), code)),
    };

    match *first {
        // We need to parse a sequence of expressions
        "(" => {
            let mut list = vec![];
            loop {
                // Try to get the next token
                match rest.split_first() {
                    // If we have the end of a list we just exit the loop
                    Some((&")", r)) => {
                        rest = r;
                        break;
                    }
                    // We parse the element of the list recursively
                    Some((_, _)) => {
                        let (expr, r) = parse(rest)?;
                        // add the expr to the current list
                        list.push(expr);
                        rest = r;
                    }
                    // We reached the end of the input without getting anything
                    _ => return Err(Box::new(LispError::Reason("Expected list closing".into()))),
                }
            }
            // Return the expression that we just built
            Ok((LispExpr::List(list), rest))
        }
        // If we get the end of a list we are doing something wrong here, we should only be parsing
        // the start of expressions
        ")" => Err(Box::new(LispError::Reason(
            "Unexpected list closing".into(),
        ))),
        // Check if we have a number
        // Unwrapping is safe here because we
        // know that there is at least one character
        // in the split
        x if x.chars().next().unwrap().is_digit(10) => Ok((LispExpr::Number(x.parse()?), rest)),

        // Check if we have a alpha character (start of an identifier)
        // Unrwrapping applies here as wel
        x => Ok((LispExpr::Symbol(x.into()), rest)),
    }
}

fn eval(code: &str) -> LispExpr {
    // Tokenize the lisp code
    let code = code.replace("(", " ( ").replace(")", " ) ");
    let code: Vec<&str> = code.split_whitespace().collect();

    // Parse and then return the expression instead of the remaining tokens
    // TODO: fix this so that it returns an error if there is any remaining input
    parse(&code).unwrap().0
}

fn main() -> Result<()> {
    // Initialize xdg dirs
    let xdg_dirs = BaseDirectories::with_prefix("rlisp").unwrap();
    let history_path = xdg_dirs
        .place_cache_file("rlisp_history")
        .expect("Could not create config directory");

    // Setup the readline library
    let history = Box::new(
        reedline::FileBackedHistory::with_file(9000, history_path)
            .expect("Error configuring history with file"),
    );
    let mut line_editor = Reedline::create()?
        .with_history(history)
        .expect("Failed to setup history file");

    loop {
        // Use the prompt
        // TODO: extend the functionality of the prompt to keep track of stuff like loaded modules
        // and errors
        match line_editor.read_line(&ReplPrompt {})? {
            Signal::Success(text) => {
                // If we got some text, we evaluate it and print the result
                let result = eval(&text);
                println!("{:?}", result);
                println!("eval {:?}", result.eval());
            }

            // End the program if we are asked to or we reach end of input
            Signal::CtrlD | Signal::CtrlC => {
                break;
            }

            // Clear the screen
            Signal::CtrlL => {
                line_editor.clear_screen()?;
            }
        }
    }

    Ok(())
}
