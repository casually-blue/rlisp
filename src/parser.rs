use crate::error::LispError;
use crate::expr::LispExpr;
use crate::result::Result;

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
                    _ => return Err(LispError::reason("Expected list closing")),
                }
            }
            // Return the expression that we just built
            Ok((LispExpr::List(list), rest))
        }
        // If we get the end of a list we are doing something wrong here, we should only be parsing
        // the start of expressions
        ")" => Err(LispError::reason("Unexpected list closing")),
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

pub fn eval(code: &str) -> Result<LispExpr> {
    // Tokenize the lisp code
    let code = code.replace("(", " ( ").replace(")", " ) ");
    let code: Vec<&str> = code.split_whitespace().collect();

    // Parse and then return the expression instead of the remaining tokens
    match parse(&code) {
        Ok((expr, rest)) => {
            if rest.len() == 0 {
                Ok(expr)
            } else {
                Err(LispError::reason("Failed to parse all of input"))
            }
        }
        Err(e) => Err(e),
    }
}
