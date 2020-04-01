use ipl::address::Address;
use std::fmt::Write as FmtWrite;

pub fn get_lineprotocol(myadr: Address) -> Result<String, Box<dyn std::error::Error>> {
    let mut s = String::new();
    write!(&mut s, "{},", myadr.name).expect("error in measurement");
    Ok(s)
}

fn main() {
    let a1 = Address::p0("michael".to_string(), "pa".to_string(), "15132".to_string());
    let v1 = vec![a1];

    for t in v1.iter() {
        println!("Name: {}", t.name);
        println!("Details: {:?}", t.details);
        //println!("{:?}", t.get_lineprotocol());
        println!("Protocol: {:?}", get_lineprotocol(*t));
    }
}
