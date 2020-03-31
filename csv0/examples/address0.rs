use ipl::address::Address;

fn main() {
    let t = Address::new("michael".to_string(), "pa".to_string(), "15132".to_string());
    println!("Name: {}", t.name);
    println!("{:?}",t);
}
