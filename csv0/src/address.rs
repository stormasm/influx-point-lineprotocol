use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Address {
    pub name: String,
    pub details: HashMap<String, String>,
}

impl Address {
    pub fn p0(name: String, state: String, zip: String) -> Address {
        let mut details = HashMap::new();
        details.insert("state".to_string(), state);
        details.insert("zip".to_string(), zip);
        Address { name, details }
    }
}
