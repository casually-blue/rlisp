use std::collections::HashMap;

use crate::error::LispError;
use crate::expr::LispExpr;

use crate::result::Result;


#[derive(Debug, Clone)]
pub struct LispEnv {
    pub this: HashMap<String, LispExpr>,
    pub parent: Option<Box<LispEnv>>
}

impl LispEnv {
    pub fn lookup(&self, symbol: String) -> Result<LispExpr> {
        if let Some(expr) = self.this.get(&symbol) {
            Ok(expr.clone())
        } else if let Some(parent) = &self.parent {
            parent.lookup(symbol)
        } else {
            Err(LispError::reason(format!("Symbol not found {}", symbol)))
        }
    }

    pub fn new() -> Self {
        let env = LispEnv {
            this: HashMap::new(),
            parent: None
        };

        env
    }
}
