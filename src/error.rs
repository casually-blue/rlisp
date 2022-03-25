#[derive(Debug)]
pub enum LispError {
    Reason(String),
}

impl LispError {
    pub fn reason<T: Into<String>>(reason: T) -> Box<dyn std::error::Error> {
        Box::new(LispError::Reason(reason.into()))
    }
}

impl std::fmt::Display for LispError {
    // Just output the debug result
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for LispError {}
