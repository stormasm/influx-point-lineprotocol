fn take_ownership(v1: &mut Vec<i32>) -> &Vec<i32>{
    v1.push(42);
    v1.push(43);
    v1.push(44);
    v1
}

fn main() {
    let mut v1 = vec![1, 2, 3];
    let x = take_ownership(&mut v1);
    println!("{:?}",x);
}
