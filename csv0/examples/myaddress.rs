use ipl::address::Address;

fn main() {
    let t = Address::new("michael".to_string(),
                        "new mexico".to_string(),
                        "87107".to_string())

    println!("Name: {}", t.name);
}
