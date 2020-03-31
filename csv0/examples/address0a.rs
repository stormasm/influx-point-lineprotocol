use ipl::address::Address;

fn main() {
    let t1 = Address::p0("michael".to_string(), "pa".to_string(), "15132".to_string());
    let t2 = Address::p0("hb".to_string(), "pa".to_string(), "15132".to_string());
    let t3 = Address::p0("iris".to_string(), "pa".to_string(), "15132".to_string());

    let v1 = vec![t1, t2, t3];

    for t in v1.iter() {
        println!("{:?}\n", t);
        println!("Name: {}", t.name);
        println!("{:?}", t);
        println!("{:?}", t.get_lineprotocol());
    }
}
