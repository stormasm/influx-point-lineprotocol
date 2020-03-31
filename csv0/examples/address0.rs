use ipl::address::Address;

fn main() {
    let t = Address::new("michael".to_string());
    println!("Name: {}", t.name);
}
