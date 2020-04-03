use std::collections::HashMap;
use std::fmt::Write as FmtWrite;

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

    pub fn p1(name: String, details: HashMap<String, String>) -> Address {
        Address { name, details }
    }

    pub fn get_lineprotocol(self) -> Result<String, Box<dyn std::error::Error>> {
        let mut s = String::new();
        write!(&mut s, "{},", self.name).expect("error in measurement");

        for (key, val) in self.details {
            write!(&mut s, "{}={},", key, val).expect("error in tagset");
        }

        // remove the last comma from the tagset
        let strlen = s.len();
        let mut s1 = String::from(s);
        s1.remove(strlen - 1);

        // add in a space between the tagset and the fieldset
        write!(&mut s1, "{}", " ".to_string()).expect("error in space");

        Ok(s1)
    }
}
