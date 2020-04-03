use ipl::address::Address;

fn main() {
    let a1 = Address::p0("michael".to_string(), "pa".to_string(), "15132".to_string());
    let v1 = vec![a1];

    for t in v1.iter() {
        println!("Name: {}", t.name);
        println!("Details: {:?}", t.details);
        //println!("{:?}", t.get_lineprotocol());
    }
}
