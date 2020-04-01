use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Human {
    pub details: HashMap<String, String>,
}

impl Human {
    pub fn p1(details: HashMap<String, String>) -> Human {
        Human { details }
    }
}
