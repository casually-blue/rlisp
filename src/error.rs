
#[derive(Debug)]
pub enum LispError {
    Reason(String),
}

impl std::fmt::Display for LispError {
    // Just output the debug result
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for LispError {}
