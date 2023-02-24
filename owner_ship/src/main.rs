// fn main() {                  //s没被创造，所以是无效的
//     let s = "hello";   //s被创造了
// }                            //s离开了作用域，无效

// fn  main() {
//     let mut s = String::from("hello");   
//     s.push_str(",world");
//     println!("{}",s);
// }



// fn main() {
//     let s1 = String::from("hello"); // 创建一个 String
//     let len = calculate_length(&s1); // 传入一个引用
//     println!("The length of '{}' is {}.", s1, len); // 输出结果
// }

// fn calculate_length(s: &String) -> usize { // 函数使用引用作为参数
//     s.len() // 返回值为字符串的长度
// }


fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
