use crate::error::LispError;
use crate::expr::*;
use crate::result::Result;

use crate::token::*;

fn parse_list(mut code: &[Token]) -> Result<(LispExpr, &[Token])> {
    use LispExpr::*;
    use LispList::*;

    let mut list = Nil;
    loop {
        // Try to get the next token
        match code.split_first() {
            // If we have the end of a list we just exit the loop
            Some((Token::LClose, r)) => {
                code = r;
                break Ok((List(box list), code));
            }
            // We parse the element of the list recursively
            Some((_, _)) => {
                let (expr, r) = parse(code)?;
                // add the expr to the current list
                list = Cons(box list, expr);
                code = r;
            }
            // We reached the end of the input without getting anything
            _ => return Err(LispError::reason("Expected list closing")),
        }
    }
    // Return the expression that we just built
}

fn parse(code: &[Token]) -> Result<(LispExpr, &[Token])> {
    // If we don't have anything we can just return a blank input
    let (first, rest) = match code.split_first() {
        Some((f, r)) => (f, r),
        _ => return Ok((LispExpr::List(box LispList::Nil), code)),
    };

    match first {
        // We need to parse a sequence of expressions
        Token::LOpen => parse_list(rest),
        // If we get the end of a list we are doing something wrong here, we should only be parsing
        // the start of expressions
        Token::LClose => Err(LispError::reason("Unexpected list closing")),
        // Check if we have a number
        // Unwrapping is safe here because we
        // know that there is at least one character
        // in the split
        Token::Number(n) => Ok((LispExpr::Number(*n), rest)),

        // Check if we have a alpha character (start of an identifier)
        // Unrwrapping applies here as wel
        Token::Symbol(s) => Ok((LispExpr::Symbol(s.clone()), rest)),

        _ => Err(LispError::reason("Unsupported token type")),
    }
}

pub fn tl_parse(code: &str) -> Result<LispExpr> {
    // Tokenize the lisp code
    let tokens = crate::lexer::tokenize(code)?;

    match parse(&tokens)? {
        (expr, []) => Ok(expr),
        _ => Err(LispError::reason("Failed to parse all of input"))
    }
}
