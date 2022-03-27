use crate::result::Result;
use crate::token::*;

pub fn tokenize(code: &str) -> Result<Vec<Token>> {
    let mut code = code;
    let mut tokens = vec![];
    loop {
        let (token, rest) = get_token(skip_whitespace(code))?;
        match token {
            Token::EOF => {
                return Ok(tokens);
            },
            token => {
                tokens.push(token);
                code = rest;
            },
        }
    }
}

fn skip_whitespace(code: &str) -> &str {
    let mut whitespace_end_idx = 0;
    let mut chrs = code.chars();

    while let Some(c) = chrs.next() {
        if !c.is_whitespace() {
            break;
        }
        whitespace_end_idx += 1;
    }

    &code[whitespace_end_idx..]
}

fn get_token(code: &str) -> Result<(Token, &str)> {
    match code.chars().nth(0) {
        Some('(') => Ok((Token::LOpen, &code[1..])),
        Some(')') => Ok((Token::LClose, &code[1..])),
        Some('"') | Some('\'') => get_string(code),
        Some(x) if x.is_digit(10) => get_number(code),
        Some(_) => get_symbol(code),
        _ => Ok((Token::EOF, code))
    }
}

fn get_string(code: &str) -> Result<(Token, &str)> {
    let mut end_idx = 0;
    let mut chrs = code.chars();
    let ending_char = chrs.next().expect("Unexpected end of code while trying to parse beginning of string");

    while let Some(c) = chrs.next() {
        end_idx += 1;

        match c {
            c if c == ending_char => break,
            '\\' => {
                chrs.next().expect("Unexpected end of code in escape sequence");
                end_idx += 1
            },
            _ => {},
        }
    }

    Ok((Token::StrLit(code[1..end_idx].to_owned()), &code[end_idx+1..]))
}


fn get_number(code: &str) -> Result<(Token, &str)> {
    let mut end_idx = 0;
    let mut chrs = code.chars();

    while let Some(c) = chrs.next() {
        if !c.is_digit(10) {
            break;
        }
        end_idx += 1;
    }

    Ok((Token::Number(code[0..end_idx].parse()?), &code[end_idx..]))
}

fn get_symbol(code: &str) -> Result<(Token, &str)> {
    let mut chrs = code.chars();
    let mut end_idx = 0;

    // Eat chars until the end of the identifier
    while let Some(c) = chrs.next() {
        if c == '(' || c == ')' || c == '"' || c == '\'' || c.is_whitespace() {
            break;
        } else {
            end_idx += 1;
        }
    }

    Ok((Token::Symbol(code[..end_idx].to_owned()), &code[end_idx..]))
}
