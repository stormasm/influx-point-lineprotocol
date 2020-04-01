use ipl::human::Human;
use std::collections::HashMap;

fn main() {
    let mut details = HashMap::new();
    details.insert("state".to_string(), "nm".to_string());
    details.insert("zip".to_string(), "87110".to_string());

    let t = Human::p1(details);
    println!("{:?}", t);
}
