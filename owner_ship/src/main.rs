// fn main() {                  //s没被创造，所以是无效的
//     let s = "hello";   //s被创造了
// }                            //s离开了作用域，无效

fn  main() {
    let mut s = String::from("hello");   
    s.push_str(",world");
    println!("{}",s);
}