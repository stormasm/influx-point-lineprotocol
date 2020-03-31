fn take_ownership2(v2: &mut Vec<i32>) -> &Vec<i32> {
    v2.push(50);
    v2.push(51);
    v2.push(52);
    v2
}

fn take_ownership1(v1: &mut Vec<i32>) -> &Vec<i32> {
    v1.push(42);
    v1.push(43);
    v1.push(44);
    v1
}

fn main() {
    let mut v1 = vec![1, 2, 3];
    let x = take_ownership1(&mut v1);
    println!("{:?}", x);
    let y = take_ownership2(&mut v1);
    println!("{:?}", y);
}
