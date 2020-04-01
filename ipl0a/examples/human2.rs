use ipl::human::Human;
use std::collections::HashMap;

fn getlp(h1: Human) -> String {
    let mut s = String::new();
    s.push_str(h1.details.get("state").unwrap());
    s.push_str(", world!");
    s
}

fn main() {
    let mut details = HashMap::new();
    details.insert("state".to_string(), "nm".to_string());
    details.insert("zip".to_string(), "87110".to_string());

    let h1 = Human::p1(details);
    let v1 = vec![h1];
    for t in v1.iter() {
        println!("{:?}", getlp(*t));
    }
}
