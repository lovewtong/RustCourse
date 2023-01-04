// 为什么rust变量要设计成默认情况下是不可变的呢？
// 
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
