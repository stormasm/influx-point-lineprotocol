use std::collections::HashMap;
use std::fmt::Write as FmtWrite;

#[derive(Debug, Clone)]
pub struct Address {
    pub name: String,
    pub details: HashMap<String, String>,
    pub lp: String,
}

impl Address {
    pub fn new(&self, name: String, state: String, zip: String) -> Address {
        Address { name: name,
            details: self.set_details(state,zip),
            lp: self.set_lineprotocol()
        }
    }

    fn set_details(self, state: String, zip: String) -> HashMap<String,String> {
        let mut foo = HashMap::new();
        foo.insert("state".to_string(), state);
        foo.insert("zip".to_string(), zip);
        foo.clone()
    }

    fn set_lineprotocol(self) -> String {
        let mut s = String::new();
        write!(&mut s, "{},", self.name).expect("error in measurement");

        for (key, val) in self.details {
            write!(&mut s, "{}={},", key, val).expect("error in tagset");
        }

        s
    }
}
