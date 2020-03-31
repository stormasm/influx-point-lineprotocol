use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Address {
    pub name: String,
    pub details: HashMap<String, String>,
}

impl Address {
    pub fn new(name: String) -> Address {
        let mut details = HashMap::new();
        details.insert("state".to_string(), "pa".to_string());
        details.insert("zip".to_string(), "15132".to_string());
        details.insert("age".to_string(), "49".to_string());
        Address { name, details }
    }
}
