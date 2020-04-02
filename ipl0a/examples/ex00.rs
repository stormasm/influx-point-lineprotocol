// https://users.rust-lang.org/t/cannot-move-out-of-x-which-is-behind-a-shared-reference/33263

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Child {}

#[derive(Default)]
struct A {
    map: HashMap<String, Child>,
}

impl A {
    pub fn default() -> A {
        let mut a = A {
            map: HashMap::new(),
        };
        a.map.insert("one".to_string(), Child{});
        a
    }

    pub fn get_map(&self) -> HashMap<String, Child> {
        self.map
    }
}

fn do_something(value: Child) {
    println!("Value: {:?}", value);
}

fn main() {
    let a = A::default();
    let x = a.get_map();

    for (key, value) in x {
        println!("{}", key);
        do_something(value);
    }
}
