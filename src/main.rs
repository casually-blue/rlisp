#![feature(box_syntax)]

extern crate alloc;

mod error;
mod expr;
mod parser;
mod prompt;
mod result;
mod lexer;
mod token;
mod env;
mod eval;

use env::LispEnv;
use eval::eval;
use parser::tl_parse;
use prompt::ReplPrompt;
use result::Result;

use reedline::{Reedline, Signal};
use xdg::*;

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
                let result = tl_parse(&text);
                println!("Parse Tree: {:?}", result);
                /*
                match &result {
                    Ok(res) => println!("Eval result: {:?}", eval(res.clone(), LispEnv::new())),
                    Err(_) => println!("Eval result: {:?}", result),
                }
                */
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
