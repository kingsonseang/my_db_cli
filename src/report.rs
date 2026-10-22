use crate::backend::Backend;

pub struct Report {
    pub backend: Backend,
    pub prefix: String,
    pub touched: Vec<String>, // DBs actually created/cleared/dropped
    pub skipped: Vec<String>, // DBs seen but outside the scope
}

impl Report {
    pub fn empty(backend: Backend, prefix: &str) -> Self {
        Self {
            backend,
            prefix: prefix.to_string(),
            touched: Vec::new(),
            skipped: Vec::new(),
        }
    }
}
