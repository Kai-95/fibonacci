
fn main() {
    let mut a :i64  = 0;
    println!("{}",a);
    let mut b :i64 = 1;
    println!("{}",b);
    for index in 2..50 {
        let c = a + b;
        println!("{}",c);
        a = b;
        b = c;
    }
}
