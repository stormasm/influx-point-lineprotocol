use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Human {
    pub details: HashMap<String, String>,
}

impl Human {
    pub fn p1(details: HashMap<String, String>) -> Human {
        Human { details }
    }

    pub fn getlp(&self) -> String {
        let mut s = String::new();
        s.push_str(self.details.get("state").unwrap());
        s.push_str(", country!");
        s
    }
}
